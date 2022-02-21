use crate::*;
use bytemuck::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/openspecs/windows_protocols/ms-erref/0642cb2f-2075-4469-918c-4441e69c548a)\]
/// FACILITY_\* values corresponding to Microsoft (non-customer) `HRSEULT`s.
///
/// Note that NTSTATUS facilities, despite also being prefixed with FACILITY_\*, are incompatible (overlapping values interpreted differently!)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)] pub struct FacilityHrMicrosoft(pub(crate) u16);

impl FacilityHrMicrosoft {
    // Microsofts specs list facilities as only having 11 bits: https://docs.microsoft.com/en-us/openspecs/windows_protocols/ms-erref/0642cb2f-2075-4469-918c-4441e69c548a
    // However, DirectDraw and Direct3D9 uses 0x876, which sets a 12th bit.
    // Additionally, Direct3D 10, Direct3D 11, etc. also get up into the 12 bits area.
    // This goes into the reserved `X` bit.  Since Microsoft is using what Microsoft reserved, that's fine.
    #[doc(hidden)] pub const fn from_constant(value: u16) -> Self { assert!(value <= 0xFFF, "HRESULT facilities are only 12 bits"); Self(value) }

    pub const fn to_u16(self) -> u16 { self.0 }
    pub const fn to_u32(self) -> u32 { self.0 as _ }
}

impl From<FacilityHrMicrosoft> for u16 { fn from(f: FacilityHrMicrosoft) -> Self { f.0 } }
impl From<FacilityHrMicrosoft> for u32 { fn from(f: FacilityHrMicrosoft) -> Self { f.0.into() } }



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/openspecs/windows_protocols/ms-erref/0642cb2f-2075-4469-918c-4441e69c548a)\]
/// Success HRESULT
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Zeroable)] #[repr(transparent)] pub struct SuccessHResult(pub(crate) u32);
// DO NOT IMPLEMENT:
//  Pod (error bit patterns are forbidden)

impl SuccessHResult {
    #[doc(hidden)] pub const fn from_constant(value: u32) -> Self { assert!(value & 0x8000_0000 == 0, "SuccessHResult::from_constant: HRESULT is an error (high bit set)"); Self(value) }

    //  const fn is_reserved(self) -> bool { self.0 & 0x40000000 != 0 }
    pub const fn is_customer(self) -> bool { self.0 & 0x20000000 != 0 }
    pub const fn is_ntstatus(self) -> bool { self.0 & 0x10000000 != 0 }

    pub const fn facility   (self) -> u16  { (self.0 & 0x0FFF0000 >> 16) as _ }
    pub const fn code       (self) -> u16  { self.0 as _ }
    pub const fn to_u32     (self) -> u32  { self.0 }
}

impl From<SuccessHResult> for u32 { fn from(hr: SuccessHResult) -> Self { hr.0 } }
impl From<u32> for SuccessHResult { fn from(hr: u32) -> Self { Self(hr) } }
impl From<(FacilityHrMicrosoft, SuccessCodeMicrosoft)> for SuccessHResult { fn from((fac, code): (FacilityHrMicrosoft, SuccessCodeMicrosoft)) -> Self { Self((fac.to_u32()<<16) | code.to_u32()) } }



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/openspecs/windows_protocols/ms-erref/0642cb2f-2075-4469-918c-4441e69c548a)\]
/// Error HRESULT
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)] #[repr(transparent)] pub struct ErrorHResult(pub(crate) u32);
// DO NOT IMPLEMENT:
//  Pod         (success bit patterns are forbidden)
//  Default     (0 is success)
//  Zeroable    (0 is success)

impl ErrorHResult {
    #[doc(hidden)] pub const fn from_constant(value: u32) -> Self { assert!(value & 0x8000_0000 != 0, "ErrorHResult::from_constant: HRESULT is a success (high bit not set)"); Self(value) }

    //  const fn is_reserved(self) -> bool { self.0 & 0x40000000 != 0 }
    pub const fn is_customer(self) -> bool { self.0 & 0x20000000 != 0 }
    pub const fn is_ntstatus(self) -> bool { self.0 & 0x10000000 != 0 }

    pub const fn facility   (self) -> u16  { (self.0 & 0x0FFF0000 >> 16) as _ }
    pub const fn code       (self) -> u16  { self.0 as _ }
    pub const fn to_u32     (self) -> u32  { self.0 }
}

impl From<ErrorHResult> for u32 { fn from(hr: ErrorHResult) -> Self { hr.0 } }
impl From<u32> for ErrorHResult { fn from(hr: u32) -> Self { Self(hr) } }
impl From<(FacilityHrMicrosoft, ErrorCodeMicrosoft)> for ErrorHResult { fn from((fac, code): (FacilityHrMicrosoft, ErrorCodeMicrosoft)) -> Self { Self(0x8000_0000 | (fac.to_u32()<<16) | code.to_u32()) } }



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/openspecs/windows_protocols/ms-erref/0642cb2f-2075-4469-918c-4441e69c548a)\]
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

    pub const fn succeeded(self) -> Result<SuccessHResult, ErrorHResult> {
        if self.is_error() {
            Err(ErrorHResult(self.0))
        } else {
            Ok(SuccessHResult(self.0))
        }
    }
}

impl From<HResult>          for u32     { fn from(hr: HResult       ) -> Self { hr.0 } }
impl From<u32>              for HResult { fn from(hr: u32           ) -> Self { Self(hr) } }
impl From<SuccessHResult>   for HResult { fn from(hr: SuccessHResult) -> Self { Self(hr.0) } }
impl From<ErrorHResult>     for HResult { fn from(hr: ErrorHResult  ) -> Self { Self(hr.0) } }
impl From<(FacilityHrMicrosoft, ErrorCodeMicrosoft  )> for HResult { fn from((fac, code): (FacilityHrMicrosoft, ErrorCodeMicrosoft  )) -> Self { Self(0x8000_0000 | (fac.to_u32()<<16) | code.to_u32()) } }
impl From<(FacilityHrMicrosoft, SuccessCodeMicrosoft)> for HResult { fn from((fac, code): (FacilityHrMicrosoft, SuccessCodeMicrosoft)) -> Self { Self(              (fac.to_u32()<<16) | code.to_u32()) } }
