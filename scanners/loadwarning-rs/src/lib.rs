mod config_utils;
mod load_avg;
mod interval;

use crate::{
    config_utils::{
        get_interval_from_config,
        get_threasholds_and_warning_level,
    },
    interval::Interval,
    load_avg::{LoadAvg, AvgTimeframe},
};
use elosplugin::{
    event::{self, Classification, Event, EventSeverity, EventSource},
    load_plugin,
    plugin::{api::ElosPluginApi, type_wraps::SafuResult, ElosPluginConfig, Plugin, PluginType},
    start_plugin, stop_plugin, unload_plugin,
};
use samconf::Config;
use std::{
    sync::{Arc, Condvar, Mutex}, time::Duration
};

const ELOS_MSG_CODE_HIGH_SYSTEM_LOAD: u32 = 1201;
const ELOS_MSG_CODE_NORMAL_SYSTEM_LOAD: u32 = 1202;

#[derive(Debug, Clone)]
struct LoadScanner {
    source: EventSource,
    running: Arc<(Mutex<bool>, Condvar)>,
    hardware_id: Option<String>,
    avg_timeframe: AvgTimeframe,
    interval: Duration,
    bucket: Interval,
    thresholds: Vec<f64>,
    warning: Option<f64>,
    epsilon: f64,
}

impl LoadScanner {
    fn find_interval(&self, load: f64) -> Interval {
        if self.thresholds.is_empty() {
            return Interval::Empty;
        }
        let mut l = 0;
        let mut r = self.thresholds.len() - 1;
        if self.thresholds[l] > load {
            return Interval::Bottom(self.thresholds[l]);
        }
        if self.thresholds[r] < load {
            return Interval::Top(self.thresholds[r]);
        }
        if l == r {
            return Interval::Bottom(self.thresholds[l]);
        }
        while r - l > 1 {
            let i = (r + l) / 2;
            if self.thresholds[i] > load {
                r = i;
            } else if self.thresholds[i] <= load {
                l = i;
            }
        }
        Interval::Segment(self.thresholds[l], self.thresholds[r])
    }
    fn find_level(&self, bucket: &Interval) -> EventSeverity {
        let warn_level = match self.warning {
            Some(w) => w,
            None => {
                return EventSeverity::Info;
            }
        };
        match bucket {
            Interval::Bottom(_) => EventSeverity::Info,
            Interval::Top(_) => EventSeverity::Warn,
            Interval::Segment(left, _) if *left >= warn_level => EventSeverity::Warn,
            _ => EventSeverity::Info,
        }
    }
    fn is_new(&self, load: f64) -> bool {
        match self.bucket {
            Interval::Top(t) => t - self.epsilon > load,
            Interval::Bottom(b) => b + self.epsilon < load,
            Interval::Segment(b, t) => {
                t + self.epsilon < load || b - self.epsilon > load
            }
            Interval::Empty => false,
        }
    }
    fn build_new_event(&mut self, load: f64) -> Option<Event> {
        if self.is_new(load) {
            let bucket = self.find_interval(load);
            let level = self.find_level(&bucket);
            let code = if level == EventSeverity::Info {
                ELOS_MSG_CODE_NORMAL_SYSTEM_LOAD
            } else {
                ELOS_MSG_CODE_HIGH_SYSTEM_LOAD
            };
            let ev = Event::new()
                .source(self.source.clone())
                .add_classification(Classification::Kernel)
                .severity(level)
                .message_code(code);
            let ev = match &self.hardware_id {
                Some(id) => ev.with_hardware_id(id.clone()),
                None => ev,
            };
            let ev = match bucket {
                Interval::Empty => return None,
                Interval::Top(t) => ev.payload(format!("CPU load is above {}", t)),
                Interval::Bottom(b) => ev.payload(format!("CPU load is below {} again", b)),
                Interval::Segment(b, t) if self.bucket < load => {
                    ev.payload(format!("CPU load is above {}, and below {}", b, t))
                }
                Interval::Segment(b, t) => ev.payload(format!(
                    "CPU load below {} againe, but still above {}",
                    t, b
                )),
            };
            self.bucket = bucket;
            Some(ev)
        } else {
            None
        }
    }
    fn scan(&mut self) -> Option<Event> {
        let load = LoadAvg::get();
        let load_val = match self.avg_timeframe {
            AvgTimeframe::One => load.one,
            AvgTimeframe::Five => load.five,
            AvgTimeframe::Fifteen => load.fifteen,
        };
        self.build_new_event(load_val)
    }
}

impl Plugin for LoadScanner {
    fn load(api: &ElosPluginApi<LoadScanner>) -> LoadScanner {
        let conf = api.config();
        let source = conf.key().unwrap_or("LoadScanner");
        let avg_timeframe = match conf.get("Config/AvgTimeframe") {
            Config::String(s) => match s.value() {
                "One" => AvgTimeframe::One,
                "Five" => AvgTimeframe::Five,
                "Fifteen" => AvgTimeframe::Fifteen,
                _ => AvgTimeframe::default(),
            },
            _ => AvgTimeframe::default(),
        };
        let interval = get_interval_from_config(&conf.get("Config/Interval"));
        let epsilon = match conf.get("Config/Epsilon") {
            Config::Real(r) => r.value(),
            Config::Int(i) => i.value() as f64,
            _ => 0.1,
        };
        let (warning, thresholds) = get_threasholds_and_warning_level(&conf.get("Config/Thresholds"));
        LoadScanner {
            source: EventSource::new().app(source.to_owned()),
            running: Arc::new((Mutex::new(false), Condvar::new())),
            hardware_id: event::get_hardware_id(),
            bucket: if thresholds.is_empty() { Interval::Empty } else { Interval::Bottom(thresholds[0]) },
            avg_timeframe,
            interval,
            thresholds,
            warning,
            epsilon,
        }
    }
    fn stop(&mut self) -> SafuResult {
        let (lock, cvar) = &*self.running;
        let mut run = match lock.lock() {
            Ok(r) => r,
            Err(_) => return SafuResult::Failed,
        };
        *run = false;
        cvar.notify_all();
        SafuResult::Ok
    }
    fn run(&mut self, api: &ElosPluginApi<LoadScanner>) -> SafuResult {
        if self.thresholds.is_empty() {
            return SafuResult::Ok;
        }
        let (lock, cvar) = &*self.running.clone();
        let mut run = match lock.lock() {
            Ok(r) => r,
            Err(_) => return SafuResult::Failed,
        };
        *run = true;

        let publ = api.create_publisher();
        'scan: loop {
            let res = cvar.wait_timeout(run, self.interval);
            run = match res {
                Ok(r) => r,
                Err(_) => return SafuResult::Failed,
            }
            .0;
            if !(*run) {
                break 'scan;
            }

            if let Some(ev) = self.scan() {
                publ.publish(&ev);
                api.store(&ev);
            }
        }
        SafuResult::Ok
    }
}

impl Drop for LoadScanner {
    fn drop(&mut self) {
    }
}

#[allow(non_upper_case_globals)]
#[no_mangle]
static elosPluginConfig: ElosPluginConfig<LoadScanner> = ElosPluginConfig {
    plugin_type: PluginType::Scanner,
    load: load_plugin!(LoadScanner),
    unload: unload_plugin!(LoadScanner),
    start: start_plugin!(LoadScanner),
    stop: stop_plugin!(LoadScanner),
};
