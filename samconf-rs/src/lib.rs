
use std::{
    ffi::{
        CStr,
        CString,
    },
    fmt,
};

pub use samconf_sys as ffi;

pub struct SamconfString<'s>(pub &'s ffi::samconfConfig);
pub struct SamconfInt<'s>(pub &'s ffi::samconfConfig);
pub struct SamconfReal<'s>(pub &'s ffi::samconfConfig);
pub struct SamconfBoolean<'s>(pub &'s ffi::samconfConfig);
pub struct SamconfArray<'s>(pub &'s ffi::samconfConfig);
pub struct SamconfObject<'s>(pub &'s ffi::samconfConfig);

impl<'s> SamconfString<'s> {
    pub fn value(&self) -> &'s str {
        unsafe {
            CStr::from_ptr((*self.0).value.string).to_str().unwrap()
        }
    }
    pub fn get(&self, path: &str) -> Config {
        get_from_config(self.0, path)
    }
}
impl<'s> SamconfInt<'s> {
    pub fn value(&self) -> i64 {
        unsafe {
            (*self.0).value.integer
        }
    }
    pub fn get(&self, path: &str) -> Config {
        get_from_config(self.0, path)
    }
}
impl<'s> SamconfReal<'s> {
    pub fn value(&self) -> f64 {
        unsafe {
            (*self.0).value.real
        }
    }
    pub fn get(&self, path: &str) -> Config {
        get_from_config(self.0, path)
    }
}
impl<'s> SamconfBoolean<'s> {
    pub fn value(&self) -> bool {
        unsafe {
            (*self.0).value.boolean as bool
        }
    }
    pub fn get(&self, path: &str) -> Config {
        get_from_config(self.0, path)
    }
}
impl<'s> SamconfArray<'s> {
    pub fn iter(&self) -> ConfigArrayIter {
        ConfigArrayIter::new(self.0)
    }
    pub fn get(&self, path: &str) -> Config {
        get_from_config(self.0, path)
    }
    pub  fn at(&self, idx: usize) -> Option<Config> {
        if idx < self.len() {
            unsafe {
                Some(Config::new(&**(*self.0).children.add(idx)))
            }
        } else {
            None
        }
    }
    pub fn len(&self) -> usize {
        (*self.0).childCount
    }
}
impl<'s> SamconfObject<'s> {
    pub fn iter(&self) -> ConfigArrayIter {
        ConfigArrayIter::new(self.0)
    }
    pub fn get(&self, path: &str) -> Config {
        get_from_config(self.0, path)
    }
}

fn get_from_config<'s>(conf: &'s ffi::samconfConfig, path: &str) -> Config<'s> {
    let mut node: *const ffi::samconfConfig = std::ptr::null();
    let res = unsafe {
        let path = CString::new(path).expect("failed to get c string form path");
        ffi::samconfConfigGet(conf, path.as_ptr(), &mut node)
    };
    if res != ffi::samconfConfigStatusE_SAMCONF_CONFIG_OK {
        Config::None
    } else {
        unsafe {
            Config::new(&*node)
        }
    }
}

pub struct ConfigArrayIter<'s> {
    node: &'s ffi::samconfConfig,
    idx: usize,
}
impl<'s> ConfigArrayIter<'s> {
    fn new(conf: &'s ffi::samconfConfig) -> ConfigArrayIter<'s> {
        ConfigArrayIter {
            node: conf,
            idx: 0,
        }
    }
}
impl<'s> Iterator for ConfigArrayIter<'s> {
    type Item = Config<'s>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.idx >= (*self.node).childCount {
            self.idx = (*self.node).childCount;
            return None;
        }
        let res = unsafe {
            Some(Config::new(&**(*self.node).children.add(self.idx)))
        };
        self.idx += 1;
        res
    }
}

#[derive(Debug)]
pub enum Config<'s> {
    None,
    String(SamconfString<'s>),
    Int(SamconfInt<'s>),
    Real(SamconfReal<'s>),
    Boolean(SamconfBoolean<'s>),
    Array(SamconfArray<'s>),
    Object(SamconfObject<'s>),
}

impl<'s> Config<'s> {
    pub fn new(conf: &'s ffi::samconfConfig) -> Config<'s> {
        match (*conf).type_ {
            ffi::samconfConfigValueTypeE_SAMCONF_CONFIG_VALUE_STRING => Config::String(SamconfString(conf)),
            ffi::samconfConfigValueTypeE_SAMCONF_CONFIG_VALUE_INT => Config::Int(SamconfInt(conf)),
            ffi::samconfConfigValueTypeE_SAMCONF_CONFIG_VALUE_REAL => Config::Real(SamconfReal(conf)),
            ffi::samconfConfigValueTypeE_SAMCONF_CONFIG_VALUE_BOOLEAN => Config::Boolean(SamconfBoolean(conf)),
            ffi::samconfConfigValueTypeE_SAMCONF_CONFIG_VALUE_ARRAY => Config::Array(SamconfArray(conf)),
            ffi::samconfConfigValueTypeE_SAMCONF_CONFIG_VALUE_OBJECT => Config::Object(SamconfObject(conf)),
            _ => Config::None,
        }
    }
    pub fn get<'a>(&'s self, path: &'a str) -> Config<'s> {
        match self {
            Config::None => Config::None,
            Config::Int(i) => i.get(path),
            Config::Real(r) => r.get(path),
            Config::Boolean(b) => b.get(path),
            Config::String(s) => s.get(path),
            Config::Array(a) => a.get(path),
            Config::Object(o) => o.get(path),
        }
    }
    pub fn key(&'s self) -> Option<&'s str> {
        match self {
            Config::None => None,
            Config::Int(i) => unsafe { CStr::from_ptr((*i.0).key).to_str().ok() },
            Config::Real(r) => unsafe { CStr::from_ptr((*r.0).key).to_str().ok() },
            Config::Boolean(b) => unsafe { CStr::from_ptr((*b.0).key).to_str().ok() },
            Config::String(s) => unsafe { CStr::from_ptr((*s.0).key).to_str().ok() },
            Config::Array(a) => unsafe { CStr::from_ptr((*a.0).key).to_str().ok() },
            Config::Object(o) => unsafe { CStr::from_ptr((*o.0).key).to_str().ok() },
        }
    }
}

impl<'s> fmt::Debug for SamconfString<'s> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        unsafe {
            f.debug_struct("SamconfString").field("key", &CStr::from_ptr((*self.0).key)).field("value", &self.value()).finish_non_exhaustive()
        }
    }
}
impl<'s> fmt::Debug for SamconfInt<'s> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        unsafe {
            f.debug_struct("SamconfInt").field("key", &CStr::from_ptr((*self.0).key)).field("value", &self.value()).finish_non_exhaustive()
        }
    }
}
impl<'s> fmt::Debug for SamconfReal<'s> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        unsafe {
            f.debug_struct("SamconfReal").field("key", &CStr::from_ptr((*self.0).key)).field("value", &self.value()).finish_non_exhaustive()
        }
    }
}
impl<'s> fmt::Debug for SamconfBoolean<'s> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        unsafe {
            f.debug_struct("SamconfBoolean").field("key", &CStr::from_ptr((*self.0).key)).field("value", &self.value()).finish_non_exhaustive()
        }
    }
}
struct ChildListDebug<'s>(&'s ffi::samconfConfig);
impl<'s> fmt::Debug for ChildListDebug<'s> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let iter = ConfigArrayIter::new(self.0);
        f.debug_list().entries(iter).finish()
    }
}
impl<'s> fmt::Debug for SamconfArray<'s> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        unsafe {
            f.debug_struct("SamconfArray").field("key", &CStr::from_ptr((*self.0).key)).field("Children", &ChildListDebug(self.0)).finish_non_exhaustive()
        }
    }
}
impl<'s> fmt::Debug for SamconfObject<'s> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        unsafe {
            f.debug_struct("SamconfObject").field("key", &CStr::from_ptr((*self.0).key)).field("Children", &ChildListDebug(self.0)).finish_non_exhaustive()
        }
    }
}
