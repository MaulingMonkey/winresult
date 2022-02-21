use bytemuck::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/openspecs/windows_protocols/ms-erref/87fba13e-bf06-450e-83b1-9241dc81e781)\]
/// NTSTATUS
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Pod, Zeroable)] #[repr(transparent)] pub struct NTSTATUS(pub(crate) u32);

impl NTSTATUS {
    #[doc(hidden)] pub const fn from_constant(value: u32) -> Self { Self(value) }

    pub const fn sev                (self) -> NtStatusSeverity { NtStatusSeverity((self.0 >> 30) as _) }

    pub const fn is_error           (self) -> bool { matches!(self.sev(), STATUS::SEVERITY::ERROR            ) }
    pub const fn is_warning         (self) -> bool { matches!(self.sev(), STATUS::SEVERITY::WARNING          ) }
    pub const fn is_informational   (self) -> bool { matches!(self.sev(), STATUS::SEVERITY::INFORMATIONAL    ) }
    pub const fn is_success         (self) -> bool { matches!(self.sev(), STATUS::SEVERITY::SUCCESS          ) }

    pub const fn is_customer        (self) -> bool { self.0 & 0x20000000 != 0 }
    pub const fn is_ntstatus        (self) -> bool { self.0 & 0x10000000 != 0 }

    pub const fn facility           (self) -> u16  { (self.0 & 0x0FFF0000 >> 16) as _ }
    pub const fn code               (self) -> u16  { self.0 as _ }
    pub const fn to_u32             (self) -> u32  { self.0 }
}

impl From<NTSTATUS> for u32 { fn from(hr: NTSTATUS) -> Self { hr.0 } }
impl From<u32> for NTSTATUS { fn from(hr: u32) -> Self { Self(hr) } }



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/openspecs/windows_protocols/ms-erref/87fba13e-bf06-450e-83b1-9241dc81e781)\]
/// NTSTATUS::Sev
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)] pub struct NtStatusSeverity(pub(crate) u8);

impl NtStatusSeverity {
    #[doc(hidden)] pub const fn from_constant(value: u8) -> Self { assert!(value < 4, "NTSTATUS severities are only 2 bits"); Self(value) }

    pub const fn to_u8 (self) ->  u8 { self.0 }
    pub const fn to_u16(self) -> u16 { self.0 as _ }
    pub const fn to_u32(self) -> u32 { self.0 as _ }
}



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/openspecs/windows_protocols/ms-erref/87fba13e-bf06-450e-83b1-9241dc81e781)\]
/// FACILITY_\* values corresponding to Microsoft (non-customer) `NTSTATUS`es.
///
/// Note that HRESULT facilities, despite also being prefixed with FACILITY_\*, are incompatible (overlapping values interpreted differently!)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)] pub struct FacilityNtStatusMicrosoft(pub(crate) u16);

impl FacilityNtStatusMicrosoft {
    #[doc(hidden)] pub const fn from_constant(value: u16) -> Self { assert!(value <= 0xFFF, "NTSTATUS facilities are only 12 bits"); Self(value) }

    pub const fn to_u16(self) -> u16 { self.0 }
    pub const fn to_u32(self) -> u32 { self.0 as _ }
}

impl From<FacilityNtStatusMicrosoft> for u16 { fn from(f: FacilityNtStatusMicrosoft) -> Self { f.0 } }
impl From<FacilityNtStatusMicrosoft> for u32 { fn from(f: FacilityNtStatusMicrosoft) -> Self { f.0.into() } }



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/openspecs/windows_protocols/ms-erref/87fba13e-bf06-450e-83b1-9241dc81e781)\]
/// NTSTATUS::Sev
#[allow(non_snake_case)]
pub mod STATUS {
    /// [SUCCESS](Self::SUCCESS)
    /// [INFORMATIONAL](Self::INFORMATIONAL)
    /// [WARNING](Self::WARNING)
    /// [ERROR](Self::ERROR)
    pub mod SEVERITY {
        use crate::*;
        pub const SUCCESS       : NtStatusSeverity = NtStatusSeverity::from_constant(0);
        pub const INFORMATIONAL : NtStatusSeverity = NtStatusSeverity::from_constant(1);
        pub const WARNING       : NtStatusSeverity = NtStatusSeverity::from_constant(2);
        pub const ERROR         : NtStatusSeverity = NtStatusSeverity::from_constant(3);
    }
}