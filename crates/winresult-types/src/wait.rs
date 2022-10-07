use bytemuck::*;
use core::fmt::{self, Debug, Formatter};



/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/synchapi/nf-synchapi-waitformultipleobjectsex#return-value)\]
/// WAIT_\* values returned by various WaitFor\* and other win32 functions.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Zeroable)] #[repr(transparent)] pub struct WaitCode(pub(crate) u32);

impl WaitCode {
    #[doc(hidden)] pub const fn from_constant(value: u32) -> Self { Self(value) }

    pub const fn to_u32(self) -> u32 { self.0 }

    /// ### Returns
    /// *   `Some(0)`   for `WAIT::OBJECT_0`
    /// *   `Some(63)`  for `WAIT::OBJECT_0 + 63`
    /// *   `None`      for `WAIT::OBJECT_0 + 64` (>= `MAXIMUM_WAIT_OBJECTS`)
    pub fn to_object<T: From<u8>>(self) -> Option<T> {
        if (0 .. 64).contains(&self.0) {
            Some((self.0 as u8).into())
        } else {
            None
        }
    }

    /// ### Returns
    /// *   `Some(0)`   for `WAIT::ABANDONED_0`
    /// *   `Some(63)`  for `WAIT::ABANDONED_0 + 63`
    /// *   `None`      for `WAIT::ABANDONED_0 + 64` (>= `MAXIMUM_WAIT_OBJECTS`)
    pub fn to_abandoned<T: From<u8>>(self) -> Option<T> {
        if (0x80 .. 0xC0).contains(&self.0) {
            Some(((self.0 - 0x80) as u8).into())
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
        else if self.0 == 0xC0                          { write!(fmt, "WAIT::IO_COMPLETION") }
        else if self.0 == 0x102                         { write!(fmt, "WAIT::TIMEOUT") }
        else if self.0 == 0xFFFFFFFF                    { write!(fmt, "WAIT::FAILED") }
        else                                            { write!(fmt, "WaitCode(0x{:08X})", self.0) }
    }
}
