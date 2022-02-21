/// Success values that "aren't" HRESULTs (but might be implicitly convertable)
///
/// In general, if you squint hard enough, there *are* HRESULTs using FACILITY_NULL.
/// But, since they overlap with ERROR_\* values, things get muddled.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)] pub struct SuccessCodeMicrosoft(pub(crate) u16);

impl SuccessCodeMicrosoft {
    #[doc(hidden)] pub const fn from_constant(value: u16) -> Self { Self(value) }

    pub const fn to_u16(self) -> u16 { self.0 }
    pub const fn to_u32(self) -> u32 { self.0 as _ }
}

impl From<u16> for SuccessCodeMicrosoft { fn from(c: u16) -> Self { Self(c) } }
impl From<SuccessCodeMicrosoft> for u16 { fn from(c: SuccessCodeMicrosoft) -> Self { c.0 } }
impl From<SuccessCodeMicrosoft> for u32 { fn from(c: SuccessCodeMicrosoft) -> Self { c.0.into() } }



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/debug/system-error-codes)\]
/// ERROR_\* values that aren't HRESULTs (but might be implicitly convertable)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)] pub struct ErrorCodeMicrosoft(pub(crate) u16);

impl ErrorCodeMicrosoft {
    #[doc(hidden)] pub const fn from_constant(value: u16) -> Self { Self(value) }

    pub const fn to_u16(self) -> u16 { self.0 }
    pub const fn to_u32(self) -> u32 { self.0 as _ }
}

impl From<u16> for ErrorCodeMicrosoft { fn from(c: u16) -> Self { Self(c) } }
impl From<ErrorCodeMicrosoft> for u16 { fn from(c: ErrorCodeMicrosoft) -> Self { c.0 } }
impl From<ErrorCodeMicrosoft> for u32 { fn from(c: ErrorCodeMicrosoft) -> Self { c.0.into() } }
