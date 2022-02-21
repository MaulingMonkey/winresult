use crate::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/openspecs/windows_protocols/ms-erref/0642cb2f-2075-4469-918c-4441e69c548a)\]
/// [ErrorHResult] or [ErrorCodeMicrosoft]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)] #[repr(transparent)] pub struct ErrorHResultOrCode(pub(crate) u32);

impl ErrorHResultOrCode {
    //         0 ..=         0: <invalid/reserved>
    //         1 ..=      FFFF: ErrorCodeMicrosoft
    //    1_0000 ..= 7FFF_FFFF: <invalid/reserved>
    // 8000_0000 ..= FFFF_FFFF: ErrorHResult

    pub const fn to_u32(self)       -> u32                          { self.0 }
    pub const fn to_code(self)      -> Option<ErrorCodeMicrosoft>   { if (0 < self.0) && (self.0 <= 0xFFFF) { Some(ErrorCodeMicrosoft(self.0 as _)) } else { None } }
    pub const fn to_hresult(self)   -> Option<ErrorHResult>         { if 0x8000_0000 <= self.0              { Some(ErrorHResult(self.0))            } else { None } }
}

impl From<ErrorHResultOrCode> for u32                { fn from(v: ErrorHResultOrCode) -> Self { v.0 } }
impl From<ErrorCodeMicrosoft> for ErrorHResultOrCode { fn from(v: ErrorCodeMicrosoft) -> Self { Self(v.0.into()) } }
impl From<ErrorHResult      > for ErrorHResultOrCode { fn from(v: ErrorHResult      ) -> Self { Self(v.0) } }
