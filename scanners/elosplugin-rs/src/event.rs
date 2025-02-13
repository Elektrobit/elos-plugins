
use bitflags::bitflags;
use libc::{
    pid_t, timespec
};
use std::{
    ffi::CString, fmt, fs::read_to_string, process, time::{
        self, Duration, SystemTime
    }
};

const HARDWARE_ID_FILE: &str = "/etc/machine-id";

#[derive(Debug, Clone, Default)]
pub struct EventSource {
    app_name: Option<String>,
    file_name: Option<String>,
    pid: Option<u32>,
}

impl EventSource {
    pub fn new() -> EventSource {
        EventSource::default()
    }
    pub fn app(mut self, name: String) -> EventSource {
        self.app_name = Some(name);
        self
    }
    pub fn file(mut self, name: String) -> EventSource {
        self.file_name = Some(name);
        self
    }
    pub fn pid(mut self, id: u32) -> EventSource {
        self.pid = Some(id);
        self
    }
    /// sets the pid to the one of the current process ([process::id()])
    pub fn this_pid(self) -> EventSource {
        self.pid(process::id())
    }
}

bitflags! {
    /// the bitflags for the elos event classification
    #[derive(Debug, Copy, Clone, Default)]
    pub struct Classification: u64 {
        const Undefined     = 0x0000000000000000;
        const Kernel        = 0x0000000000000001;
        const Network       = 0x0000000000000002;
        const Security      = 0x0000000000000004;
        const Power         = 0x0000000000000008;
        const Storage       = 0x0000000000000010;
        const Process       = 0x0000000000000020;
        const Ipc           = 0x0000000000000040;
        const Hardware      = 0x0000000000000080;
        const Elos          = 0x0000000000000100;
        const ProcessErrors = 0x0000000000000200;
        const User0         = 0x0000000200000000;
        const User1         = 0x0000000400000000;
        const User2         = 0x0000000800000000;
        const User3         = 0x0000001000000000;
        const User4         = 0x0000002000000000;
        const User5         = 0x0000004000000000;
        const User6         = 0x0000008000000000;
        const User7         = 0x0000010000000000;
        const User8         = 0x0000020000000000;
    }
}

/// the elos severity level
#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub enum EventSeverity {
    #[default]
    Off = 0,
    Fatal,
    Error,
    Warn,
    Info,
    Debug,
    Verbose,
}

/// a safe elos Event representation
#[derive(Debug, Clone, Default)]
pub struct Event {
    date: Option<SystemTime>,
    source: Option<EventSource>,
    severity: EventSeverity,
    hardware_id: Option<String>,
    classification: Option<Classification>,
    message_code: Option<u32>,
    payload: Option<String>,
}

impl Event {
    /// creates a new Event with date [SystemTime::now()]
    pub fn new() -> Event {
        Event {
            date: Some(SystemTime::now()),
            ..Default::default()
        }
    }
    /// sets the payload string
    pub fn payload(mut self, pay: String) -> Event {
        self.payload = Some(pay);
        self
    }
    /// sets the severity
    pub fn severity(mut self, sev: EventSeverity) -> Event {
        self.severity = sev;
        self
    }
    /// sets the severity to [EventSeverity::Fatal]
    pub fn fatal(self) -> Event {
        self.severity(EventSeverity::Fatal)
    }
    /// sets the severity to [EventSeverity::Error]
    pub fn error(self) -> Event {
        self.severity(EventSeverity::Error)
    }
    /// sets the severity to [EventSeverity::Warn]
    pub fn warn(self) -> Event {
        self.severity(EventSeverity::Warn)
    }
    /// sets the severity to [EventSeverity::Info]
    pub fn info(self) -> Event {
        self.severity(EventSeverity::Info)
    }
    /// sets the severity to [EventSeverity::Debug]
    pub fn debug(self) -> Event {
        self.severity(EventSeverity::Debug)
    }
    /// sets the severity to [EventSeverity::Verbose]
    pub fn verbose(self) -> Event {
        self.severity(EventSeverity::Verbose)
    }
    pub fn with_hardware_id(mut self, id: String) -> Event {
        self.hardware_id = Some(id);
        self
    }
    pub fn source(mut self, source: EventSource) -> Event {
        self.source = Some(source);
        self
    }
    /// adds a [Classification] flag to the event
    pub fn add_classification(mut self, class: Classification) -> Event {
        self.classification = Some(match self.classification {
            None => class,
            Some(c) => c | class,
        });
        self
    }
    /// adds a whole list of [Classification]s to the list
    pub fn add_classifications<I>(self, class: I) -> Event
    where
         I: Iterator<Item = Classification> {
        class.fold(self, |event, c| event.add_classification(c))

    }
}

/// gets the hardware id from `/etc/machine-id` [None] if thats not possible
pub fn get_hardware_id() -> Option<String> {
    match read_to_string(HARDWARE_ID_FILE) {
        Ok(id) => Some(id.trim().to_owned()),
        Err(_) => None,
    }
}

/// the elos event representation in the form that elos understands it 
/// should be generated with [`ElosEvent::from::<Event>`()]
#[repr(C)]
#[derive(Clone)]
pub struct ElosEvent {
    date: timespec,
    source: ElosEventSource,
    severity: EventSeverity,
    hardwareid: *mut ::std::os::raw::c_char,
    classification: u64,
    message_code: u32,
    payload: *mut ::std::os::raw::c_char,
}

fn time_to_timespec(t: &SystemTime) -> timespec {
    let ti = t.duration_since(time::UNIX_EPOCH).unwrap_or(Duration::new(0,0));
    let sec = ti.as_secs() as i64;
    let nsec = ti.subsec_nanos() as i64;
    timespec { tv_sec: sec, tv_nsec: nsec }
}
fn cstr_or_null_ptr(s: Option<String>) -> *mut ::std::os::raw::c_char {
    match s {
        None => ::std::ptr::null_mut(),
        Some(s) => {
            CString::new(s).map(|s| s.into_raw()).unwrap_or(::std::ptr::null_mut())
        }
    }
}

impl From<&Event> for ElosEvent {
    fn from(event: &Event) -> ElosEvent {
        ElosEvent {
            date: event.date.map_or(timespec { tv_sec: 0, tv_nsec: 0 }, |t|time_to_timespec(&t)),
            source: event.source.clone().map(|s| s.into()).unwrap_or_default(),
            severity: event.severity,
            hardwareid: cstr_or_null_ptr(event.hardware_id.clone()),
            classification: event.classification.map_or(0, |c| c.bits()),
            message_code: event.message_code.unwrap_or(0),
            payload: cstr_or_null_ptr(event.payload.clone()),
        }
    }
}

impl From<Event> for ElosEvent {
    fn from(event: Event) -> ElosEvent {
        ElosEvent {
            date: event.date.map_or(timespec { tv_sec: 0, tv_nsec: 0 }, |t|time_to_timespec(&t)),
            source: event.source.map(|s| s.into()).unwrap_or_default(),
            severity: event.severity,
            hardwareid: ::std::ptr::null_mut(),
            classification: event.classification.map_or(0, |c| c.bits()),
            message_code: event.message_code.unwrap_or_default(),
            payload: cstr_or_null_ptr(event.payload),
        }
    }
}

/// the event source representation for elosd should be generated with
/// [`ElosEventSource::from::<EventSource>`()]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct ElosEventSource {
    app_name: *mut ::std::os::raw::c_char,
    file_name: *mut ::std::os::raw::c_char,
    pid: pid_t,
}

impl From<EventSource> for ElosEventSource {
    fn from(source: EventSource) -> ElosEventSource {
        ElosEventSource {
            app_name: cstr_or_null_ptr(source.app_name),
            file_name: cstr_or_null_ptr(source.file_name),
            pid: source.pid.unwrap_or(0) as i32,
        }
    }
}

impl Default for ElosEventSource {
    fn default() -> ElosEventSource {
        ElosEventSource {
            app_name: ::std::ptr::null_mut(),
            file_name: ::std::ptr::null_mut(),
            pid: 0,
        }
    }
}

struct Ts(timespec);
impl fmt::Debug for Ts {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("timespec")
            .field("tv_sec", &self.0.tv_sec)
            .field("tv_nsec", &self.0.tv_nsec)
            .finish()
    }
}

impl fmt::Debug for ElosEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ElosEvent")
            .field("date", &Ts(self.date))
            .field("source", &self.source)
            .field("severity", &self.severity)
            .field("hardwareid", &self.hardwareid)
            .field("classification", &self.classification)
            .field("message_code", &self.message_code)
            .field("payload", &self.payload)
            .finish()
    }
}
