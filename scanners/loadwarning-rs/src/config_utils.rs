
use samconf::Config;
use std::{
    time::Duration,
    cmp::Ordering, 
};

pub(crate) fn get_interval_from_config(conf: &Config) -> Duration {
       match conf {
           Config::Int(i) => Duration::from_secs(i.value() as u64),
           Config::Real(r) => {
               let (sec, nano) = float_to_sec_and_nano(r.value());
               Duration::new(sec, nano)
           },
           Config::Array(a) => {
               let (sec, nano) = match a.at(0) {
                   Some(Config::Int(i)) => (i.value() as u64, 0),
                   Some(Config::Real(r)) => float_to_sec_and_nano(r.value()),
                   _ => (1, 0),
               };
               match a.at(1) {
                   Some(Config::Int(i)) => Duration::new(sec + i.value() as u64, nano),
                   Some(Config::Real(r)) => Duration::new(sec, nano + r.value() as u32),
                   _ => Duration::new(sec, nano),
               }
           },
           Config::Object(o) => {
               let dur = match o.get("Seconds") {
                   Config::Int(i) => Duration::from_secs(i.value() as u64),
                   _ => Duration::ZERO,
               } + match o.get("MilliSeconds") {
                   Config::Int(i) => Duration::from_millis(i.value() as u64),
                   _ => Duration::ZERO,
               } + match o.get("MicroSeconds") {
                   Config::Int(i) => Duration::from_micros(i.value() as u64),
                   _ => Duration::ZERO,
               } + match o.get("NanoSeconds") {
                   Config::Int(i) => Duration::from_nanos(i.value() as u64),
                   _ => Duration::ZERO,
               };
               if dur.is_zero() {
                   Duration::from_secs(1)
               } else {
                   dur
               }
           },
           _ => Duration::from_secs(1),
       }
}

pub(crate) fn get_threasholds_and_warning_level(conf: &Config) -> (Option<f64>, Vec<f64>) {
    match conf {
        Config::Object(o) => {
            let warn = match o.get("Warn") {
                Config::Int(i) => vec![i.value() as f64],
                Config::Real(r) => vec![r.value() as f64],
                Config::Array(a) => get_level_list(a.iter()),
                _ => Vec::new(),
            };
            let warning = warn.get(0).copied();
            let threasholds = match o.get("Info") {
                Config::Int(i) => vec![i.value() as f64],
                Config::Real(r) => vec![r.value() as f64],
                Config::Array(a) => get_level_list(a.iter()),
                _ => Vec::new(),
            }.into_iter().filter(|&i| match warning {
                None => true,
                Some(w) => i < w,
            }).chain(warn.into_iter()).collect();
            (warning, threasholds)
        },
        Config::Array(a) => (None, get_level_list(a.iter())),
        Config::Real(r) => (Some(r.value()), vec![r.value()]),
        Config::Int(i) => (Some(i.value() as f64), vec![i.value() as f64]),
        _ => (None, Vec::new()),
    }
}

fn float_to_sec_and_nano(real: f64) -> (u64, u32) {
    let sec = real as u64;
    let nano = (real - sec as f64) * 1_000_000_000.0;
    (sec, nano as u32)
}

fn get_level_list(it: samconf::ConfigArrayIter) -> Vec<f64> {
    let mut level_list: Vec<f64> = it.filter_map(|c| match c {
        Config::Int(i) => Some(i.value() as f64),
        Config::Real(r) => Some(r.value()),
        _ => None,
    }).collect();
    level_list.sort_by(|a,b| a.partial_cmp(b).unwrap_or(Ordering::Equal));
    level_list
}

