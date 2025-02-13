
pub mod api;
pub mod type_wraps;

use api::ElosPluginApi;
use type_wraps::SafuResult;

/// interface to implement the plugin functionality 
/// to clean up any resources [drop()] should also be implemented for each plugin
pub trait Plugin {
    /// constructs the plugin instance
    fn load() -> Self;
    /// run the plugin main loop that handles all the plugin logic
    fn run<T>(&mut self, api: &ElosPluginApi<T>) -> SafuResult;
    /// triggers the plugin to stop and shut down gracefully
    fn stop(&mut self) -> SafuResult;
    // INFO:  unload implemented through drop
}

#[repr(C)]
pub enum PluginType {
    Scanner = 1,
    StorageBackend = 2,
    ClientConnection = 3,
}

/// interface for elosd to call the plugin functionality
/// ```
/// use loadscanner::{
///     plugin::{
///         api::ElosPluginApi,
///         Plugin,
///         PluginType,
///         ElosPluginConfig,
///         type_wraps::SafuResult,
///     },
///     load_plugin, start_plugin, stop_plugin, unload_plugin,
/// };
/// #[derive(Debug)]
/// struct Plug;
/// impl Plugin for Plug {
///     fn load() -> Plug {
///         Plug
///     }
///     fn run<Plug>(&mut self, api: &ElosPluginApi<Plug>) -> SafuResult {
///         SafuResult::Ok
///     }
///     fn stop(&mut self) -> SafuResult {
///         SafuResult::Ok
///         
///     }
/// }
/// #[unsafe(no_mangle)]
/// static elosPluginConfig: ElosPluginConfig<Plug> = ElosPluginConfig {
///     plugin_type: PluginType::Scanner,
///     load: load_plugin!(Plug),
///     unload: unload_plugin!(Plug),
///     start: start_plugin!(Plug),
///     stop: stop_plugin!(Plug),
/// };
/// ```
#[repr(C)]
pub struct ElosPluginConfig<T> {
    pub plugin_type: PluginType,
    pub load: fn(*mut ElosPluginApi<T>) -> SafuResult,
    pub unload: fn(*mut ElosPluginApi<T>) -> SafuResult,
    pub start: fn(*mut ElosPluginApi<T>) -> SafuResult,
    pub stop: fn(*mut ElosPluginApi<T>) -> SafuResult,
}

/// Generates a function for elos to load the plugin resources
/// using [Plugin::load()] internally
#[macro_export]
macro_rules! load_plugin {
    ($plugin:ty) => {
        |elos_plugin: *mut ElosPluginApi<$plugin>| -> SafuResult {
            let plg = <$plugin>::load();
            unsafe {
                (*elos_plugin).init(plg);
            }
            SafuResult::Ok
        }
    }
}

/// generates a function for elos to start the plugin logic loop
/// wrapping [Plugin::run()] in all the required setup and stop boilerplate
#[macro_export]
macro_rules! start_plugin {
    ($plugin:ty) => {
        |elos_plugin: *mut ElosPluginApi<$plugin>| -> SafuResult {
            let plg: &mut $plugin = unsafe {
                (*elos_plugin).plugin_mut()
            };

            let mut res = unsafe {
                (*elos_plugin).report_as_started()
            };
            if res != SafuResult::Failed {
                res = unsafe {
                    plg.run(&*elos_plugin)
                };
                res = match unsafe {
                    (*elos_plugin).stop_trigger_wait()
                } {
                    SafuResult::Ok => res,
                    e => e,
                };
            }
            res
        }
    }
}

/// Generates a function for elosd that wraps [Plugin::stop()]
/// to shut down the plugin
#[macro_export]
macro_rules! stop_plugin {
    ($plugin:ty) => {
        |elos_plugin: *mut ElosPluginApi<$plugin>| -> SafuResult {
            unsafe {
                (*elos_plugin).plugin_mut()
            }.stop();

            let res = unsafe {
                (*elos_plugin).stop_trigger_write()
            };
            res
        }
    }
}

/// Generates a function for elosd wrapping [drop()] for the Plugin
/// to free all the plugin resources
#[macro_export]
macro_rules! unload_plugin {
    ($plugin:ty) => {
        |elos_plugin: *mut ElosPluginApi<$plugin>| -> SafuResult {
            unsafe {
                (*elos_plugin).drop_plugin();
            }
            SafuResult::Ok
        }
    }
}

