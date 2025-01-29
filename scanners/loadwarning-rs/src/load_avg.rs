
use libc::getloadavg;

#[derive(Debug, Copy, Clone, Default)]
pub(crate) struct LoadAvg {
    pub one: f64,
    pub five: f64,
    pub fifteen: f64,
}

impl LoadAvg {
    pub(crate) fn get() -> LoadAvg {
        let mut load = [0f64; 3];
        unsafe {
            getloadavg(&mut load as *mut f64, 3);
        }
        LoadAvg {
            one: load[0],
            five: load[1],
            fifteen: load[2],
        }
    }
}
