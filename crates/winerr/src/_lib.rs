pub use winerr_core::*;



/// **FACILITY_\* Values** for [HRESULT]s and [NTSTATUS]es.<br>
/// pub mod [FACILITY::HRESULT::*](Self::HRESULT),
/// [FACILITY::NTSTATUS::*](Self::NTSTATUS);
/// <br><br>
#[allow(non_snake_case)]
pub mod FACILITY {
    /// HRESULT facilities
    pub mod HRESULT {
        use crate::FacilityHrMicrosoft;
        macro_rules! microsoft_hresult_facilities {($(
            #define FACILITY_ $f:ident $value:literal
        )*) => {$(
            #[doc = concat!("`", stringify!($value), "` (HRESULT)")]
            pub const $f : FacilityHrMicrosoft = FacilityHrMicrosoft::from_constant($value);
        )*}}
        include!("hresult/extra.facilities.rs");
        include!("hresult/winerror.facilities.rs");
    }

    /// NTSTATUS facilities
    pub mod NTSTATUS {
        use crate::FacilityNtStatusMicrosoft;
        macro_rules! microsoft_ntstatus_facilities {($(
            #define $prefix:ident $f:ident $value:literal
        )*) => {$(
            #[doc = concat!("`", stringify!($value), "` (NTSTATUS)")]
            pub const $f : FacilityNtStatusMicrosoft = FacilityNtStatusMicrosoft::from_constant($value);
        )*}}
        include!("ntstatus/facilities.rs");
    }

    #[doc(inline)] pub use HRESULT::*;
    #[doc(inline)] pub use NTSTATUS::*;
}

/// [NTSTATUS] Errors and Codes (for use in e.g. Kernel / Driver)
#[allow(non_snake_case)]
pub mod STATUS {
    pub use winerr_core::STATUS::*;
}
