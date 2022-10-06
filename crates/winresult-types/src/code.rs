use core::convert::Infallible;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/debug/system-error-codes)\]
/// ERROR_\* values that aren't HRESULTs (but might be implicitly convertable)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)] pub struct ErrorCode(pub(crate) u16);

impl ErrorCode {
    #[doc(hidden)] pub const fn from_constant(value: u16) -> Self { Self(value) }

    pub const fn to_u16(self) -> u16 { self.0 }
    pub const fn to_u32(self) -> u32 { self.0 as _ }
}

impl From<u16> for ErrorCode { fn from(c: u16) -> Self { Self(c) } }
impl From<Infallible> for ErrorCode { fn from(i: Infallible) -> Self { match i {} } }
impl From<ErrorCode> for u16 { fn from(c: ErrorCode) -> Self { c.0 } }
impl From<ErrorCode> for u32 { fn from(c: ErrorCode) -> Self { c.0.into() } }

impl<O, E: PartialEq<ErrorCode>> PartialEq<ErrorCode> for Result<O, E> {
    fn eq(&self, other: &ErrorCode) -> bool {
        match self.as_ref() {
            Ok(_)   => false,
            Err(e)  => *e == *other,
        }
    }
}

impl<O, E: PartialEq<ErrorCode>> PartialEq<Result<O, E>> for ErrorCode {
    fn eq(&self, other: &Result<O, E>) -> bool {
        match other.as_ref() {
            Ok(_)   => false,
            Err(e)  => *e == *self,
        }
    }
}
