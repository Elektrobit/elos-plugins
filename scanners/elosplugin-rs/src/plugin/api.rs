
use super::type_wraps::*;
use crate::event::{
    Event,
    ElosEvent,
};
use std::ffi::CStr;
use libc::{
    timespec,
    eventfd_read,
    eventfd_write,
};
use samconf::{
    self,
    ffi::samconfConfig,
};

/// a wrapper around the plugin context given by elosd
#[repr(C)]
#[derive(Debug)]
pub struct ElosPluginApi<T: Sized> {
    config: *const samconfConfig,
    use_env: bool,
    id: u32,
    data: *mut T,
    sync: ::std::os::raw::c_int,
    stop: ::std::os::raw::c_int,
    instance_ref: *const ElosPluginControl,
    publish: fn(*const Publisher, *const ElosEvent) -> SafuResult,
    store: fn(*const ElosPluginControl, *const ElosEvent) -> SafuResult,
    find_events: fn(*const ElosPluginControl, CStr, *const timespec, *const timespec, *mut SafuVec) -> SafuResult,
    subscribe: fn(*const Subscriber, CStr, usize, *mut *mut Subscription) -> SafuResult,
    read_queue: fn(*const Subscriber, *const Subscription, *mut *mut SafuVec) -> SafuResult,
    unsubscribe: fn(*const Subscriber, *const Subscription) -> SafuResult,
    unsubscribe_all: fn(*const Subscriber) -> SafuResult,
    create_publisher: fn(*const ElosPluginControl, *mut *mut Publisher) -> SafuResult,
    delete_publisher: fn(*const ElosPluginControl, *const Publisher) -> SafuResult,
    create_subscriber: fn(*const ElosPluginControl, *mut *mut Subscriber) -> SafuResult,
    delete_subscriber: fn(*const ElosPluginControl, *const Subscriber) -> SafuResult,
}

impl<'api, T> ElosPluginApi<T> {
    pub fn init(&mut self, plug: T) {
        let plg = Box::new(plug);
            self.data = Box::into_raw(plg);
    }
    pub fn config(&'api self) -> samconf::Config<'api> {
        unsafe {
            samconf::Config::new(&*self.config)
        }
    }
    /// gets a reference to the plugin instance
    pub fn plugin(&self) -> &T {
        unsafe {
            &*self.data
        }
    }
    /// gets a mutable reference to the plugin instance
    pub fn plugin_mut(&mut self) -> &mut T {
        unsafe {
            &mut *self.data
        }
    }
    /// gets a publisher from elos
    pub fn create_publisher(&'api self) -> ElosPublisher<'api> {
        let mut publisher = unsafe {
            ElosPublisher {
                publ: ::std::ptr::null_mut(),
                control_ref: &*self.instance_ref,
                publish: self.publish,
                delete: self.delete_publisher,
            }
        };
        (self.create_publisher)(self.instance_ref, &mut publisher.publ);
        publisher
    }
    /// stores an event to the elos backends
    pub fn store(&self, ev: &Event) {
        let event = ElosEvent::from(ev);
        (self.store)(self.instance_ref, &event);
    }
    /// get the plugin id given by elosd
    pub fn id(&self) -> u32 {
        self.id
    }
    /// calls [drop()] for the plugin instance freeing all resources
    pub fn drop_plugin(&self) {
        unsafe {
            drop(Box::from_raw(self.data));
        }
    }
    /// tells elosd that the plugin is started
    /// needs to be called from the start method of the plugin
    /// but will be used by in the [start_plugin!](crate::start_plugin!) macro
    /// right before it calls [Plugin::run()](crate::plugin::Plugin::run())
    /// so there is no need to manually call it
    pub fn report_as_started(&self) -> SafuResult {
        let res = unsafe {
            eventfd_write(self.sync, 1u64)
        };
        if res < 0 {
            SafuResult::Failed
        } else {
            SafuResult::Ok
        }
    }
    /// blocks until elosd tells the plugin it's stopped
    /// will automatically used in the [start_plugin!](crate::start_plugin!) macro
    /// after [Plugin::run()](crate::plugin::Plugin::run()) finished
    /// so no need to manuly call it
    pub fn stop_trigger_wait(&self) -> SafuResult {
        let mut fd = 0u64;
        let res = unsafe {
            eventfd_read(self.stop, &mut fd)
        };
        if res < 0 {
            SafuResult::Failed
        } else {
            SafuResult::Ok
        }
    }
    pub fn stop_trigger_write(&self) -> SafuResult {
        let res = unsafe {
            eventfd_write(self.stop, 1u64)
        };
        if res < 0 {
            SafuResult::Failed
        } else {
            SafuResult::Ok
        }
    }
}

/// a wrapper for an elos publisher can be created by [ElosPluginApi::create_publisher()]
pub struct ElosPublisher<'api> {
    publ: *mut Publisher,
    control_ref: &'api ElosPluginControl,
    publish: fn(*const Publisher, *const ElosEvent) -> SafuResult,
    delete: fn(*const ElosPluginControl, *const Publisher) -> SafuResult,
}

impl ElosPublisher<'_> {
    /// publish an event to elos with this publisher
    pub fn publish(&self, ev: &Event) {
        let event = ElosEvent::from(ev);
        unsafe {
            (self.publish)(&*self.publ, &event);
        }
    }
}

impl Drop for ElosPublisher<'_> {
    fn drop(&mut self) {
        (self.delete)(self.control_ref, self.publ);
    }
}
