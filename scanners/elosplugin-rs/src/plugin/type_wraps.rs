#[repr(C)]
#[derive(PartialEq, Eq)]
pub enum SafuResult {
    Failed = -1,
    Ok = 0,
    NotFound = 1,
    Closed = 2
}

pub(crate) type SamconfConfig = ::std::os::raw::c_void;
pub(crate) type ElosPluginControl = ::std::os::raw::c_void;
pub(crate) type Publisher = ::std::os::raw::c_void;
pub(crate) type Subscriber = ::std::os::raw::c_void;
pub(crate) type Subscription = ::std::os::raw::c_void;
pub(crate) type SafuVec = ::std::os::raw::c_void;
