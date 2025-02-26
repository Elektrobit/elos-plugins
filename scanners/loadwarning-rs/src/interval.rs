
use std::cmp::Ordering;

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub(crate) enum Interval {
    Top(f64),
    Segment(f64, f64),
    Bottom(f64),
    Empty,
}

impl PartialEq<f64> for Interval {
    fn eq(&self, other: &f64) -> bool {
        match self {
            Interval::Top(t) => t <= other,
            Interval::Bottom(b) => b >= other,
            Interval::Segment(b, t) => b <= other && t >= other,
            Interval::Empty => false,
        }
    }
}
impl PartialOrd<f64> for Interval {
    fn partial_cmp(&self, other: &f64) -> Option<Ordering> {
        match self {
            Interval::Top(t) if t > other => Some(Ordering::Greater),
            Interval::Bottom(b) if b < other => Some(Ordering::Less),
            Interval::Segment(_, t) if t < other => Some(Ordering::Less),
            Interval::Segment(b, _) if b > other => Some(Ordering::Greater),
            _ => None,
        }
    }
}
