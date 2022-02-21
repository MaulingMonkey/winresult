use crate::*;
use core::fmt::{self, Debug, Formatter};

impl Debug for NtStatusSeverity {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        let s = match self.0 {
            0 => "STATUS::SEVERITY::SUCCESS",
            1 => "STATUS::SEVERITY::INFORMATIONAL",
            2 => "STATUS::SEVERITY::WARNING",
            3 => "STATUS::SEVERITY::ERROR",
            _ => "STATUS::SEVERITY::???",
        };
        write!(fmt, "{}", s)
    }
}

include!("gen/debug.rs");
