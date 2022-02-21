use crate::*;
use core::fmt::{self, Debug, Formatter};

impl Debug for NtStatusSeverity {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        let s = match *self {
            STATUS::SEVERITY::SUCCESS       => "STATUS::SEVERITY::SUCCESS",
            STATUS::SEVERITY::INFORMATIONAL => "STATUS::SEVERITY::INFORMATIONAL",
            STATUS::SEVERITY::WARNING       => "STATUS::SEVERITY::WARNING",
            STATUS::SEVERITY::ERROR         => "STATUS::SEVERITY::ERROR",
            _                               => "STATUS::SEVERITY::???",
        };
        write!(fmt, "{}", s)
    }
}

include!("gen/debug.rs");
