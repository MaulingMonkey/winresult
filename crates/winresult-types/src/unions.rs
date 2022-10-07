use crate::*;
use core::convert::Infallible;



/// \[[microsoft.com](https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-erref/0642cb2f-2075-4469-918c-4441e69c548a)\]
/// [HResultError] or [ErrorCode]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)] #[repr(transparent)] pub struct ErrorHResultOrCode(pub(crate) u32);

impl ErrorHResultOrCode {
    //         0 ..=      FFFF: ErrorCode
    //    1_0000 ..= 7FFF_FFFF: <invalid/reserved>
    // 8000_0000 ..= FFFF_FFFF: HResultError

    pub const fn to_u32(self)       -> u32                  { self.0 }
    pub const fn to_code(self)      -> Option<ErrorCode>    { if self.0 <= 0xFFFF       { Some(ErrorCode(self.0 as _))  } else { None } }
    pub const fn to_hresult(self)   -> Option<HResultError> { if 0x8000_0000 <= self.0  { Some(HResultError(self.0))    } else { None } }
}

impl From<ErrorHResultOrCode> for u32                { fn from(v: ErrorHResultOrCode) -> Self { v.0 } }
impl From<ErrorCode         > for ErrorHResultOrCode { fn from(v: ErrorCode         ) -> Self { Self(v.0.into()) } }
impl From<HResultError      > for ErrorHResultOrCode { fn from(v: HResultError      ) -> Self { Self(v.0) } }
impl From<Infallible        > for ErrorHResultOrCode { fn from(i: Infallible        ) -> Self { match i {} } }

// These allow construction of values in the 1_000 ..= 7FFF_FFFF range, which is a bit sketchy
impl From<u32               > for ErrorHResultOrCode { fn from(v: u32               ) -> Self { Self(v) } }
impl From<i32               > for ErrorHResultOrCode { fn from(v: i32               ) -> Self { Self(v as _) } }



macro_rules! compare { ($($a:ident == $z:ident),* $(,)?) => {$(
    impl PartialEq<$a> for $z { fn eq(&self, other: &$a) -> bool { ErrorHResultOrCode::from(*self).0 == ErrorHResultOrCode::from(*other).0 } }
    impl PartialEq<$z> for $a { fn eq(&self, other: &$z) -> bool { ErrorHResultOrCode::from(*self).0 == ErrorHResultOrCode::from(*other).0 } }
)*}}

compare! {
    ErrorHResultOrCode  == ErrorCode,
    ErrorHResultOrCode  == HResultError,
    ErrorHResultOrCode  == u32,
    ErrorHResultOrCode  == i32, // winapi HRESULT
}

impl<O, E: PartialEq<ErrorHResultOrCode>> PartialEq<ErrorHResultOrCode> for Result<O, E> {
    fn eq(&self, other: &ErrorHResultOrCode) -> bool {
        match self.as_ref() {
            Ok(_)   => false,
            Err(e)  => *e == *other,
        }
    }
}

impl<O, E: PartialEq<ErrorHResultOrCode>> PartialEq<Result<O, E>> for ErrorHResultOrCode {
    fn eq(&self, other: &Result<O, E>) -> bool {
        match other.as_ref() {
            Ok(_)   => false,
            Err(e)  => *e == *self,
        }
    }
}
