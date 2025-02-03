
use elosplugin::{
    plugin::{
        api::ElosPluginApi,
        ElosPluginConfig,
        PluginType,
        Plugin,
        type_wraps::SafuResult,
    },
    event::{
        Classification,
        Event,
        EventSeverity,
        EventSource,
        get_hardware_id,
    },
    load_plugin,
    start_plugin,
    stop_plugin,
    unload_plugin,
};

use std::{
    time::SystemTime,
    sync::{Arc, Condvar, Mutex},
    time::Duration,
};

const ELOS_MSG_CODE_DEBUG_LOG: u32 = 1101;

#[derive(Debug, Clone)]
struct Plgn {
    running: Arc<(Mutex<bool>, Condvar)>,
    data_n_stuff: u64,
    hardware_id: Option<String>,
    sev: EventSeverity,
}

impl Plgn {
    fn next_sever(&mut self) -> EventSeverity {
        self.sev = match self.sev {
            EventSeverity::Off => EventSeverity::Fatal,
            EventSeverity::Fatal => EventSeverity::Error,
            EventSeverity::Error => EventSeverity::Warn,
            EventSeverity::Warn => EventSeverity::Info,
            EventSeverity::Info => EventSeverity::Debug,
            EventSeverity::Debug => EventSeverity::Verbose,
            EventSeverity::Verbose => EventSeverity::Off
        };
        self.sev
    }
}

impl Plugin for Plgn {
    fn load() -> Plgn {
        println!("Loading RustScannerPlugin!");
        Plgn {
            running: Arc::new((Mutex::new(false), Condvar::new())),
            data_n_stuff: 42,
            hardware_id: get_hardware_id(),
            sev: EventSeverity::default(),
        }
    }
    fn stop(&mut self) -> SafuResult {
        println!("Stoping RustScannerPlugin!");
        assert_eq!(self.data_n_stuff, 4);
        self.data_n_stuff = 69;
        let (lock, cvar) = &*self.running;
        let mut run = match lock.lock() {
            Ok(r) => r,
            Err(_) => return SafuResult::Failed,
        };
        *run = false;
        cvar.notify_all();
        SafuResult::Ok
    }
    fn run<Plgn>(&mut self, api: &ElosPluginApi<Plgn>) -> SafuResult {
        let (lock, cvar) = &*self.running.clone();
        let mut run = match lock.lock() {
            Ok(r) => r,
            Err(_) => return SafuResult::Failed,
        };
        *run = true;
        println!("Starting RustScannerPlugin!");
        assert_eq!(self.data_n_stuff, 42);
        self.data_n_stuff = 4;
        let publ = api.create_publisher();
        'scan: loop {
            let res = cvar.wait_timeout(run, Duration::from_secs(1));
            run = match res {
                Ok(r) => r,
                Err(_) => return SafuResult::Failed,
            }
            .0;
            if !(*run) {
                break 'scan;
            }


            let source = EventSource::new().file(file!().to_owned()).app("RustScanner".to_owned()).this_pid();
            let ev = Event::new().payload(format!("foo: {:?}", SystemTime::now()));
            let ev = match &self.hardware_id {
                Some(id) => ev.with_hardware_id(id.clone()),
                None => ev,
            }.severity(self.next_sever()).source(source)
            .add_classification(Classification::User0)
            .message_code(ELOS_MSG_CODE_DEBUG_LOG);

            publ.publish(&ev);
            api.store(&ev);
        }
        SafuResult::Ok
    }
}

impl Drop for Plgn {
    fn drop(&mut self) {
        println!("Unloding RustScannerPlugin!");
        assert_eq!(self.data_n_stuff, 69);
    }
}

#[allow(non_upper_case_globals)]
#[no_mangle]
static elosPluginConfig: ElosPluginConfig<Plgn> = ElosPluginConfig {
    plugin_type: PluginType::Scanner,
    load: load_plugin!(Plgn),
    unload: unload_plugin!(Plgn),
    start: start_plugin!(Plgn),
    stop: stop_plugin!(Plgn),
};
