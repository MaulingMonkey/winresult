use crate::*;
use core::convert::Infallible;
use bytemuck::*;



/// \[[microsoft.com](https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-erref/0642cb2f-2075-4469-918c-4441e69c548a)\]
/// FACILITY_\* values corresponding to Microsoft (non-customer) `HRSEULT`s.
///
/// Note that NTSTATUS facilities, despite also being prefixed with FACILITY_\*, are incompatible (overlapping values interpreted differently!)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)] pub struct HResultFacilityMicrosoft(pub(crate) u16);

impl HResultFacilityMicrosoft {
    // Microsofts specs list facilities as only having 11 bits: https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-erref/0642cb2f-2075-4469-918c-4441e69c548a
    // However, DirectDraw and Direct3D9 uses 0x876, which sets a 12th bit.
    // Additionally, Direct3D 10, Direct3D 11, etc. also get up into the 12 bits area.
    // This goes into the reserved `X` bit.  Since Microsoft is using what Microsoft reserved, that's fine.
    #[doc(hidden)] pub const fn from_constant(value: u16) -> Self { assert!(value <= 0xFFF, "HRESULT facilities are only 12 bits"); Self(value) }

    pub const fn to_u16(self) -> u16 { self.0 }
    pub const fn to_u32(self) -> u32 { self.0 as _ }
}

impl From<HResultFacilityMicrosoft> for u16 { fn from(f: HResultFacilityMicrosoft) -> Self { f.0 } }
impl From<HResultFacilityMicrosoft> for u32 { fn from(f: HResultFacilityMicrosoft) -> Self { f.0.into() } }



/// \[[microsoft.com](https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-erref/0642cb2f-2075-4469-918c-4441e69c548a)\]
/// Success HRESULT
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Zeroable)] #[repr(transparent)] pub struct HResultSuccess(pub(crate) u32);
// DO NOT IMPLEMENT:
//  Pod (error bit patterns are forbidden)

impl HResultSuccess {
    #[doc(hidden)] pub const fn from_constant(value: u32) -> Self { assert!(value & 0x8000_0000 == 0, "HResultSuccess::from_constant: HRESULT is an error (high bit set)"); Self(value) }

    //  const fn is_reserved(self) -> bool { self.0 & 0x40000000 != 0 }
    pub const fn is_customer(self) -> bool { self.0 & 0x20000000 != 0 }
    pub const fn is_ntstatus(self) -> bool { self.0 & 0x10000000 != 0 }

    pub const fn facility   (self) -> u16  { (self.0 & 0x0FFF0000 >> 16) as _ }
    pub const fn code       (self) -> u16  { self.0 as _ }
    pub const fn to_u32     (self) -> u32  { self.0 }
}

impl From<HResultSuccess> for u32 { fn from(hr: HResultSuccess) -> Self { hr.0 } }
impl From<HResultSuccess> for i32 { fn from(hr: HResultSuccess) -> Self { hr.0 as _ } } // for winapi interop
impl From<Infallible> for HResultSuccess { fn from(i: Infallible) -> Self { match i {} } }

#[allow(useless_deprecated)]
#[deprecated = "allows the construction of 'successful' hresults from error values"]
impl From<u32> for HResultSuccess { fn from(hr: u32) -> Self { Self(hr) } }



/// \[[microsoft.com](https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-erref/0642cb2f-2075-4469-918c-4441e69c548a)\]
/// Error HRESULT
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)] #[repr(transparent)] pub struct HResultError(pub(crate) u32);
// DO NOT IMPLEMENT:
//  Pod         (success bit patterns are forbidden)
//  Default     (0 is success)
//  Zeroable    (0 is success)

impl HResultError {
    /// HRESULT_FROM_WIN32, but for errors specifically
    pub const fn from_win32(value: ErrorCode) -> Self { Self(0x80070000 | (value.0 as u32)) }

    #[doc(hidden)] pub const fn from_constant(value: u32) -> Self { assert!(value & 0x8000_0000 != 0, "HResultError::from_constant: HRESULT is a success (high bit not set)"); Self(value) }

    //  const fn is_reserved(self) -> bool { self.0 & 0x40000000 != 0 }
    pub const fn is_customer(self) -> bool { self.0 & 0x20000000 != 0 }
    pub const fn is_ntstatus(self) -> bool { self.0 & 0x10000000 != 0 }

    pub const fn facility   (self) -> u16  { (self.0 & 0x0FFF0000 >> 16) as _ }
    pub const fn code       (self) -> u16  { self.0 as _ }
    pub const fn to_u32     (self) -> u32  { self.0 }
}

impl From<HResultError> for u32 { fn from(hr: HResultError) -> Self { hr.0 } }
impl From<HResultError> for i32 { fn from(hr: HResultError) -> Self { hr.0 as _ } } // for winapi interop
impl From<(HResultFacilityMicrosoft, ErrorCode)> for HResultError { fn from((fac, code): (HResultFacilityMicrosoft, ErrorCode)) -> Self { Self(0x8000_0000 | (fac.to_u32()<<16) | code.to_u32()) } }
impl From<Infallible> for HResultError { fn from(i: Infallible) -> Self { match i {} } }

#[allow(useless_deprecated)]
#[deprecated = "allows the construction of 'error' hresults from success values"]
impl From<u32> for HResultError { fn from(hr: u32) -> Self { Self(hr) } }




/// \[[microsoft.com](https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-erref/0642cb2f-2075-4469-918c-4441e69c548a)\]
/// HRESULT
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Pod, Zeroable)] #[repr(transparent)] pub struct HResult(pub(crate) u32);

impl HResult {
    #[doc(hidden)] pub const fn from_constant(value: u32) -> Self { Self(value) }

    pub const fn is_error   (self) -> bool { self.0 & 0x80000000 != 0 }
    //  const fn is_reserved(self) -> bool { self.0 & 0x40000000 != 0 }
    pub const fn is_customer(self) -> bool { self.0 & 0x20000000 != 0 }
    pub const fn is_ntstatus(self) -> bool { self.0 & 0x10000000 != 0 }

    pub const fn facility   (self) -> u16  { (self.0 & 0x0FFF0000 >> 16) as _ }
    pub const fn code       (self) -> u16  { self.0 as _ }
    pub const fn to_u32     (self) -> u32  { self.0 }

    pub const fn succeeded(self) -> Result<HResultSuccess, HResultError> {
        if self.is_error() {
            Err(HResultError(self.0))
        } else {
            Ok(HResultSuccess(self.0))
        }
    }
}

impl From<HResult>          for u32     { fn from(hr: HResult       ) -> Self { hr.0 } }
impl From<HResult>          for i32     { fn from(hr: HResult       ) -> Self { hr.0 as _ } } // for winapi interop
impl From<u32>              for HResult { fn from(hr: u32           ) -> Self { Self(hr) } }
impl From<i32>              for HResult { fn from(hr: i32           ) -> Self { Self(hr as _) } } // for winapi interop
impl From<HResultSuccess>   for HResult { fn from(hr: HResultSuccess) -> Self { Self(hr.0) } }
impl From<HResultError>     for HResult { fn from(hr: HResultError  ) -> Self { Self(hr.0) } }
impl From<Infallible>       for HResult { fn from(i: Infallible     ) -> Self { match i {} } }
impl From<(HResultFacilityMicrosoft, ErrorCode)> for HResult { fn from((fac, code): (HResultFacilityMicrosoft, ErrorCode)) -> Self { Self(0x8000_0000 | (fac.to_u32()<<16) | code.to_u32()) } }



macro_rules! compare { ($($a:ident == $z:ident),* $(,)?) => {$(
    impl PartialEq<$a> for $z { fn eq(&self, other: &$a) -> bool { HResult::from(*self).0 == HResult::from(*other).0 } }
    impl PartialEq<$z> for $a { fn eq(&self, other: &$z) -> bool { HResult::from(*self).0 == HResult::from(*other).0 } }
)*}}

compare! {
    HResult         == u32,
    HResult         == i32, // winapi HRESULT
    HResultSuccess  == HResult,
    HResultSuccess  == u32,
    HResultSuccess  == i32,
    HResultError    == HResult,
    HResultError    == u32,
    HResultError    == i32,
}

impl<O, E: PartialEq<HResultError>> PartialEq<HResultError> for Result<O, E> {
    fn eq(&self, other: &HResultError) -> bool {
        match self.as_ref() {
            Ok(_)   => false,
            Err(e)  => *e == *other,
        }
    }
}

impl<O, E: PartialEq<HResultError>> PartialEq<Result<O, E>> for HResultError {
    fn eq(&self, other: &Result<O, E>) -> bool {
        match other.as_ref() {
            Ok(_)   => false,
            Err(e)  => *e == *self,
        }
    }
}

impl<O: PartialEq<HResultSuccess>, E> PartialEq<HResultSuccess> for Result<O, E> {
    fn eq(&self, other: &HResultSuccess) -> bool {
        match self.as_ref() {
            Ok(o)   => *o == *other,
            Err(_)  => false,
        }
    }
}

impl<O: PartialEq<HResultSuccess>, E> PartialEq<Result<O, E>> for HResultSuccess {
    fn eq(&self, other: &Result<O, E>) -> bool {
        match other.as_ref() {
            Ok(o)   => *o == *self,
            Err(_)  => false,
        }
    }
}
