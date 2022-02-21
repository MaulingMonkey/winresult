use bytemuck::*;
use core::fmt::{self, Debug, Formatter};



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/synchapi/nf-synchapi-waitformultipleobjectsex#return-value)\]
/// WAIT_\* values returned by various WaitFor\* and other win32 functions.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Zeroable)] #[repr(transparent)] pub struct WaitCode(pub(crate) u32);

impl WaitCode {
    pub const fn to_u32(self) -> u32 { self.0 }

    /// ### Returns
    /// *   `Some(0)`   for [`WAIT::OBJECT_0`]
    /// *   `Some(63)`  for `WAIT::OBJECT_0 + 63`
    /// *   `None`      for `WAIT::OBJECT_0 + 64` (>= `MAXIMUM_WAIT_OBJECTS`)
    pub fn to_object<T: From<u8>>(self) -> Option<T> {
        if WAIT::OBJECT_0 <= self && self <= WAIT::OBJECT_63 {
            Some(((self.0 - WAIT::OBJECT_0.0) as u8).into())
        } else {
            None
        }
    }

    /// ### Returns
    /// *   `Some(0)`   for [`WAIT::ABANDONED_0`]
    /// *   `Some(63)`  for `WAIT::ABANDONED_0 + 63`
    /// *   `None`      for `WAIT::ABANDONED_0 + 64` (>= `MAXIMUM_WAIT_OBJECTS`)
    pub fn to_abandoned<T: From<u8>>(self) -> Option<T> {
        if WAIT::ABANDONED_0 <= self && self <= WAIT::ABANDONED_63 {
            Some(((self.0 - WAIT::ABANDONED_0.0) as u8).into())
        } else {
            None
        }
    }

    pub fn to_object_u32        (self) -> Option<u32  > { self.to_object() }
    pub fn to_object_usize      (self) -> Option<usize> { self.to_object() }
    pub fn to_abandoned_u32     (self) -> Option<u32  > { self.to_abandoned() }
    pub fn to_abandoned_usize   (self) -> Option<usize> { self.to_abandoned() }
}

impl Debug for WaitCode {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        if      let Some(n) = self.to_object_u32()      { write!(fmt, "WAIT::OBJECT_{n}") }
        else if let Some(n) = self.to_abandoned_u32()   { write!(fmt, "WAIT::ABANDONED_{n}") }
        else if *self == WAIT::IO_COMPLETION            { write!(fmt, "WAIT::IO_COMPLETION") }
        else if *self == WAIT::TIMEOUT                  { write!(fmt, "WAIT::TIMEOUT") }
        else if *self == WAIT::FAILED                   { write!(fmt, "WAIT::FAILED") }
        else                                            { write!(fmt, "WaitCode(0x{:08X})", self.0) }
    }
}

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/synchapi/nf-synchapi-waitformultipleobjectsex#return-value)\]
/// WAIT_\* values returned by various WaitFor\* and other win32 functions.
#[allow(non_snake_case)]
pub mod WAIT {
    use super::*;

    pub         const OBJECT_0      : WaitCode = WaitCode(         0); // STATUS_WAIT_0
    pub(crate)  const OBJECT_63     : WaitCode = WaitCode(        63); // STATUS_WAIT_0 + 63

    // DO NOT INCLUDE: CHILD, GRANDCHILD (these are for _cwait, not WaitFor*, and ignored to boot according to process.h!

    pub         const ABANDONED_0   : WaitCode = WaitCode(      0x80); // STATUS_ABANDONED_WAIT_0
    pub(crate)  const ABANDONED_63  : WaitCode = WaitCode(      0xBF); // STATUS_ABANDONED_WAIT_0 + 63

    /// The wait was ended by one or more user-mode [asynchronous procedure calls](https://docs.microsoft.com/en-us/windows/desktop/Sync/asynchronous-procedure-calls) (APC) queued to the thread.
    pub const IO_COMPLETION : WaitCode = WaitCode(      0xC0); // STATUS_USER_APC

    /// The time-out interval elapsed.
    pub const TIMEOUT       : WaitCode = WaitCode(       258); // = 0x102 = STATUS_TIMEOUT

    // PENDING = 0x103 = 259 (STATUS_*, but no WAIT_*?)

    /// The function has failed. To get extended error information, generally call [GetLastError](https://docs.microsoft.com/en-us/windows/desktop/api/errhandlingapi/nf-errhandlingapi-getlasterror).
    pub const FAILED        : WaitCode = WaitCode(0xFFFFFFFF);

    /// WAIT_OBJECT_0 + n
    ///
    /// ### Returns
    /// *   [None]                  if `n >= 64` (MAXIMUM_WAIT_OBJECTS)
    /// *   [Some]\([WaitCode]\)    otherwise
    pub const fn OBJECT(n: u32) -> Option<WaitCode> {
        if n >= MAXIMUM_WAIT_OBJECTS { return None }
        Some(WaitCode(WAIT::OBJECT_0.0 + n))
    }

    /// WAIT_ABANDONED_0 + n
    ///
    /// ### Returns
    /// *   [None]                  if `n >= 64` (MAXIMUM_WAIT_OBJECTS)
    /// *   [Some]\([WaitCode]\)    otherwise
    pub const fn ABANDONED(n: u32) -> Option<WaitCode> {
        if n >= MAXIMUM_WAIT_OBJECTS { return None }
        Some(WaitCode(WAIT::ABANDONED_0.0 + n))
    }
}

/// Maximum number of wait objects
const MAXIMUM_WAIT_OBJECTS : u32 = 64;
