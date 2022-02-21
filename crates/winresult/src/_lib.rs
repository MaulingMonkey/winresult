#![doc = include_str!("../doc/intro.md")]
//!
//! ### Types
//!
//! |        min |        max | type                                        | notes |
//! | ----------:| ----------:| ------------------------------------------- | ----- |
//! |          0 |     0xFFFF | [`ErrorCode`]
//! |          0 | 0x7FFFFFFF | [`HResultSuccess`]
//! | 0x80000000 | 0xFFFFFFFF | [`HResultError`]
//! |          0 | 0xFFFFFFFF | [`HResult`]                                 | [`HResultSuccess`] \| [`HResultError`]
//! |          0 |      0xFFF | &nbsp; &nbsp; [`HResultFacilityMicrosoft`]  |
//! |          0 | 0xFFFFFFFF | [`NtStatus`]                                | ~~`SuccessNtStatus` \| `ErrorNtStatus`~~
//! |          0 |      0xFFF | &nbsp; &nbsp; [`NtStatusFacilityMicrosoft`] |
//! |          0 |          4 | &nbsp; &nbsp; [`NtStatusSeverity`]          |
//! |          0 | 0xFFFFFFFF | [`WaitCode`]                                | mostly <= 0x102
//!
//! ### Buggy Bitwise Comparisons to Forbid
//!
//! | left          | right                 | why |
//! | ------------- | --------------------- | --- |
//! | [`ErrorCode`] | [`HResultError`]      | never `true`, non-overlapping ranges, need to add or remove facility
//! | [`ErrorCode`] | [`HResultSuccess`]    | `ERROR_INVALID_FUNCTION == S_FALSE`, need to add or remove facility
//! | [`ErrorCode`] | [`HResult`]           | `ERROR_INVALID_FUNCTION == S_FALSE`, need to add or remove facility
//! | [`ErrorCode`] | [`WaitCode`]          | `ERROR_INVALID_FUNCTION == WAIT_OBJECT_0+1`
//! | \*Success     | \*Error\*             | never `true` except by accident
//!
//! ### Conversions
//!
//! *   [HResultSuccess] → [HResult]
//! *   ([HResultFacilityMicrosoft], [ErrorCode]) → [HResultError] → [HResult]

#![no_std]



extern crate winresult_types as types;

pub use types::{
    ErrorCode,
    HResult,
    HResultFacilityMicrosoft,
    HResultSuccess,
    HResultError,
    NtStatus,
    NtStatusFacilityMicrosoft,
    NtStatusSeverity,
    WaitCode,
    WAIT,
    ErrorHResultOrCode,
};

pub use gen::codes::{*, STATUS};



/// **FACILITY_\* Values** for [HResult]s and [NtStatus]es.<br>
/// pub mod [FACILITY::HRESULT::*](Self::HRESULT),
/// [FACILITY::NTSTATUS::*](Self::NTSTATUS);
/// <br><br>
#[allow(non_snake_case)]
pub mod FACILITY {
    /// HRESULT facilities
    pub mod HRESULT {
        use crate::HResultFacilityMicrosoft;
        macro_rules! microsoft_hresult_facilities {($(
            #define FACILITY_ $f:ident $value:literal
        )*) => {$(
            #[doc = concat!("`", stringify!($value), "` (HRESULT)")]
            pub const $f : HResultFacilityMicrosoft = HResultFacilityMicrosoft::from_constant($value);
        )*}}
        include!("hresult/extra.facilities.rs");
        include!("hresult/winerror.facilities.rs");
    }

    /// NTSTATUS facilities
    pub mod NTSTATUS {
        use crate::NtStatusFacilityMicrosoft;
        macro_rules! microsoft_ntstatus_facilities {($(
            #define $prefix:ident $f:ident $value:literal
        )*) => {$(
            #[doc = concat!("`", stringify!($value), "` (NTSTATUS)")]
            pub const $f : NtStatusFacilityMicrosoft = NtStatusFacilityMicrosoft::from_constant($value);
        )*}}
        include!("ntstatus/facilities.rs");
    }

    #[doc(inline)] pub use HRESULT::*;
    #[doc(inline)] pub use NTSTATUS::*;
}



mod gen {
    pub mod codes {
        #![allow(non_snake_case)]
        #![allow(non_upper_case_globals)]
        use types::{ErrorCode, HResultSuccess, HResultError};



        /// [NtStatus](types::NtStatus) errors, warnings, and other codes (for use in e.g. Kernel / Drivers)
        pub mod STATUS {
            use types::NtStatus;
            pub use types::STATUS::*;

            // TODO: SUCCESS = 0 ?

            #[path = "../STATUS.rs"] mod _SELF;
            pub use _SELF::*;
        }

        /// **Success codes**
        pub mod S;



        // ---- ERRORS ----

        /// [WinRT](https://en.wikipedia.org/wiki/Windows_Runtime) / [UWP](https://en.wikipedia.org/wiki/Universal_Windows_Platform) AppModel
        pub mod APPMODEL;

        /// [APPX](https://en.wikipedia.org/wiki/Universal_Windows_Platform_apps#APPX) package
        pub mod APPX;

        /// Background Task
        pub mod BT;

        /// Remote Desktop Protocol Bitmap Cache?
        pub mod CACHE;

        /// [COM Categories](https://docs.microsoft.com/en-us/windows/win32/api/comcat/)
        pub mod CAT;

        /// Certificates (for e.g. HTTPS etc.)
        pub mod CERT;

        /// Certificate Server (for e.g. Certificate Authority validation, etc.)
        pub mod CERTSRV;

        /// COM Class
        pub mod CLASS;

        /// Clipboard
        pub mod CLIPBRD;

        /// COM
        pub mod CO;

        /// [COM Admin / Catalog](https://docs.microsoft.com/en-us/windows/win32/api/comadmin/)
        pub mod COMADMIN;

        /// [COM+ Queued Components Protocol](https://docs.microsoft.com/en-us/openspecs/windows_protocols/ms-wpo/55d601ed-c63b-485b-8648-53866b3e8e21)
        pub mod COMQC;

        /// [DCOM Context](https://docs.microsoft.com/en-us/openspecs/windows_protocols/ms-dcom/94a587a3-826a-4bac-969b-ae0bbfc9a663)?
        pub mod CONTEXT;
        pub mod CONVERT10;
        pub mod CRYPT;
        pub mod CS;

        /// [Direct3D](https://docs.microsoft.com/en-us/windows/win32/direct3d)
        pub mod D3D { use super::*; pub const OK : HResultSuccess = HResultSuccess::from_constant(0); }

        /// [Direct3D](https://docs.microsoft.com/en-us/windows/win32/direct3d) 10+
        pub mod D3D10;

        /// [Direct3D 11](https://docs.microsoft.com/en-us/windows/win32/direct3d11/atoc-dx-graphics-direct3d-11)+
        pub mod D3D11;

        /// [Direct3D 12](https://docs.microsoft.com/en-us/windows/win32/direct3d12/direct3d-12-graphics)+
        pub mod D3D12;

        /// [Direct3D](https://docs.microsoft.com/en-us/windows/win32/direct3d) Errors
        pub mod D3DERR;

        /// [Direct3D](https://docs.microsoft.com/en-us/windows/win32/direct3d)
        pub mod D3DOK;

        /// OLE / Clipboard Stuff?
        pub mod DATA;

        /// [DirectComposition](https://docs.microsoft.com/en-us/windows/win32/directcomp/directcomposition-portal)
        pub mod DCOMPOSITION;

        /// Digital Signature
        pub mod DIGSIG;

        /// [IDispatch](https://docs.microsoft.com/en-us/windows/win32/api/oaidl/nf-oaidl-idispatch-invoke)
        pub mod DISP;

        /// Domain Name Services
        pub mod DNS;

        /// [Drag and Drop](https://docs.microsoft.com/en-us/windows/win32/com/drag-and-drop)
        pub mod DRAGDROP;

        /// OLE / Data Values / Clipboard Stuff?
        pub mod DV;

        /// Display Window Manager (desktop rendering composition)
        pub mod DWM;

        /// [DirectWrite](https://docs.microsoft.com/en-us/windows/win32/directwrite/direct-write-portal)
        pub mod DWRITE;

        /// DirectX
        pub mod DXCORE;

        /// [DXGI](https://docs.microsoft.com/en-us/windows/win32/direct3ddxgi/dx-graphics-dxgi)
        pub mod DXGI;

        /// **Errors Codes**.  Typically [HResultError]s.
        pub mod E;

        /// Exchange ActiveSync
        pub mod EAS;

        /// DO NOT EXPOSE THIS MESS AS IS.  See [doc/ept-and-rpc-codes-are-evil.md](https://github.com/MaulingMonkey/winresult/blob/5094a8a5568392ef855babd8bc62458f29153e46/crates/winresult/doc/ept-and-rpc-codes-are-evil.md) for details.
        ///
        /// **E**ntry **P**oin**t** for Remote Procedure Calls
        #[allow(dead_code)] mod EPT {}

        /// **Error Codes**.  Mostly a mixture of [HResultError]s and [ErrorCode]s.
        /// submodules:
        /// [CLOUD_FILE](Self::CLOUD_FILE),
        /// [CLUSTER](Self::CLUSTER),
        /// [DBG](Self::DBG),
        /// [DS](Self::DS),
        /// [EVT](Self::EVT),
        /// [GRAPHICS](Self::GRAPHICS),
        /// [IPSEC](Self::IPSEC),
        /// [MRM](Self::MRM),
        /// [MUI](Self::MUI),
        /// [NDIS](Self::NDIS),
        /// [PRI_MERGE](Self::PRI_MERGE),
        /// [SECUREBOOT](Self::SECUREBOOT),
        /// [SERVICE](Self::SERVICE),
        /// [SVHDX](Self::SVHDX),
        /// [SXS](Self::SXS),
        /// [SXS::XML](Self::SXS::XML),
        /// [VHD](Self::VHD),
        /// [WMI](Self::WMI)
        /// <br>
        /// Note that `ERROR::SUBCATEGORY::CODE` is also generally exported as `ERROR::SUBCATEGORY_CODE`, although the latter is hidden from the docs to reduce clutter.
        /// <br><br>
        pub mod ERROR {
            use types::{ErrorCode, HResultSuccess, HResultError};

            // TODO: SUCCESS = 0 ?

            #[path = "../ERROR.rs"] mod _SELF;
            pub use _SELF::*;

            /// WinSpool / Printer related
            pub mod BIDI;

            /// [OneDrive](https://en.wikipedia.org/wiki/OneDrive) / [Cloud Filter API](https://docs.microsoft.com/en-us/windows/win32/api/_cloudapi/)
            pub mod CLOUD_FILE;

            /// [Windows Clustering](https://docs.microsoft.com/en-us/windows/win32/api/_mscs/)
            pub mod CLUSTER;

            /// [Debugging](https://docs.microsoft.com/en-us/windows/win32/debug/debugging-functions)
            pub mod DBG;

            /// DHCP-related?
            pub mod DDS;

            /// DHCP
            pub mod DHCP;

            /// [Domain Services](https://en.wikipedia.org/wiki/Active_Directory#Domain_Services)
            pub mod DS;

            /// [Windows Event Log](https://docs.microsoft.com/en-us/windows/win32/wes/windows-event-log-error-constants)
            pub mod EVT;

            /// I/O Filter
            pub mod FLT;

            /// WinINet / File Transfer Protocol
            pub mod FTP;

            /// Direct3D and other graphics APIs
            pub mod GRAPHICS;

            /// WinINet / Gopher Protocol
            pub mod GOPHER;

            /// WinINet / Hyper Text Transfer Protocol
            pub mod HTTP;

            /// WinINet
            pub mod INTERNET;

            /// [IPSec](https://en.wikipedia.org/wiki/IPsec)
            pub mod IPSEC;

            /// [Package Resource Indexing](https://docs.microsoft.com/en-us/windows/win32/menurc/pri-indexing-reference)
            pub mod MRM;

            /// [Multilingual User Interface](https://en.wikipedia.org/wiki/Multilingual_User_Interface)
            pub mod MUI;

            /// Network Driver Interface Services
            pub mod NDIS;

            pub mod PATCH;

            /// PatchWiz
            pub mod PCW;

            /// [Package Resource Indexing](https://docs.microsoft.com/en-us/windows/win32/menurc/pri-indexing-reference)
            pub mod PRI_MERGE;

            /// [Secure Boot](https://docs.microsoft.com/en-us/windows-hardware/design/device-experiences/oem-secure-boot)
            pub mod SECUREBOOT;

            pub mod SERVER;

            /// [Service Application](https://docs.microsoft.com/en-us/windows/win32/services/services)
            pub mod SERVICE;

            /// [Shared Virtual Hard Disk](https://docs.microsoft.com/en-us/previous-versions/windows/it-pro/windows-server-2012-r2-and-2012/dn281956(v=ws.11))s (w/ \*.vhdx)
            pub mod SVHDX;

            /// [Side-by-side assembly](https://en.wikipedia.org/wiki/Side-by-side_assembly)
            pub mod SXS {
                use types::ErrorCode;

                #[path = "../SXS.rs"] mod _SELF;
                pub use _SELF::*;

                /// Manifest parsing errors
                #[path = "../../ERROR_SXS_XML.rs"] pub mod XML;
            }

            /// Virtual Hard Disk (\*.vhd)
            pub mod VHD;

            pub mod WINHTTP;

            /// WinRS / WinRM shell/client for WS-Management Service?
            pub mod WINRS;

            /// [Windows Management Instrumentation](https://docs.microsoft.com/en-us/windows/win32/wmisdk/wmi-start-page)
            pub mod WMI;

            /// WS-Management Service
            pub mod WSMAN;
        }

        /// [COM Events](https://docs.microsoft.com/en-us/windows/win32/api/eventsys/)
        pub mod EVENT;

        pub mod FA;

        /// Full Volume Encryption / [Bitlocker](https://en.wikipedia.org/wiki/BitLocker)
        pub mod FVE;

        /// Windows Filtering Platform
        pub mod FWP; // https://docs.microsoft.com/en-us/windows/win32/fwp/wfp-error-codes

        /// [Host Computer Network](https://docs.microsoft.com/en-us/windows-server/networking/technologies/hcn/hcn-top)
        pub mod GCN;

        /// [Host Computer Network](https://docs.microsoft.com/en-us/windows-server/networking/technologies/hcn/hcn-top)
        pub mod HCN;

        /// [Host Compute System](https://docs.microsoft.com/en-us/virtualization/api/hcs/overview)
        pub mod HCS;

        /// [WinINet / WinHTTP](https://docs.microsoft.com/en-us/windows/win32/wininet/wininet-vs-winhttp)
        pub mod HTTP;

        /// [WinINet](https://docs.microsoft.com/en-us/windows/win32/wininet/about-wininet)
        pub mod INET;

        pub mod INPLACE;

        pub mod INPUT;
        pub mod JSCRIPT;
        pub mod MEM;
        pub mod MK;
        pub mod MSDTC;
        pub mod MSSIPOTF;
        pub mod NAP;

        /// "Object Linking and Embedding"
        pub mod OLE;

        /// "Object Linking and Embedding"
        pub mod OLEOBJ;

        pub mod ONL;
        pub mod PEER;

        /// [Peer Distribution](https://docs.microsoft.com/en-us/windows/win32/p2psdk/peer-distribution)
        pub mod PEERDIST;

        pub mod PERSIST;

        /// [Performance Logs and Alerts](https://docs.microsoft.com/en-us/previous-versions/windows/desktop/pla/using-performance-logs-and-alerts)
        pub mod PLA;

        /// COM+ registration database
        pub mod REGDB;

        /// WinRT COM
        pub mod RO;

        /// DO NOT EXPOSE THIS MESS AS IS.  See [doc/ept-and-rpc-codes-are-evil.md](https://github.com/MaulingMonkey/winresult/blob/5094a8a5568392ef855babd8bc62458f29153e46/crates/winresult/doc/ept-and-rpc-codes-are-evil.md) for details.
        ///
        /// Remote Procedure Call
        #[allow(dead_code)] mod RPC {}

        /// [Smart Card](https://docs.microsoft.com/en-us/windows/security/identity-protection/smart-cards/smart-card-windows-smart-card-technical-reference)
        pub mod SCARD;

        /// Task Scheduler
        pub mod SCHED;

        pub mod SDIAG;
        pub mod SEC;
        pub mod SPAPI;

        /// [SQLite](https://www.sqlite.org/index.html)
        pub mod SQLITE;

        pub mod STATEREPOSITORY;

        /// [Structured Storage](https://docs.microsoft.com/en-us/windows/win32/stg/functions)?
        pub mod STG;

        /// [Microsoft Store](https://en.wikipedia.org/wiki/Microsoft_Store)?
        pub mod STORE;

        pub mod TBS;
        pub mod TBSIMP;

        /// Tablet PC
        pub mod TPC;

        /// [Trusted Platform Module 2.0](https://docs.microsoft.com/en-us/windows-hardware/design/device-experiences/oem-tpm)
        pub mod TPM_20;

        /// Trusted Platform Module (1.2)
        pub mod TPM;

        /// Trusted Platform Module
        pub mod TPMAPI;

        /// Certificate Trust
        pub mod TRUST;

        /// COM Type Libraries
        pub mod TYPE;

        pub mod UI;

        /// Universal Telemetry Client (UTC) data in Event Tracing for Windows (ETW) traces.
        pub mod UTC;

        pub mod VIEW;

        pub mod VM_SAVED_STATE_DUMP;

        /// [WinINet / WinHTTP](https://docs.microsoft.com/en-us/windows/win32/wininet/wininet-vs-winhttp)
        pub mod WEB;

        /// Wired Equivalent Privacy
        pub mod WEP;

        /// Windows Error Reporting
        pub mod WER;

        /// Windows Hypervisor Platform
        pub mod WHV;

        /// [WinINet](https://docs.microsoft.com/en-us/windows/win32/wininet/about-wininet)
        pub mod WININET;

        /// Windows Push Notifications?
        pub mod WPN;

        /// [Windows Web Services](https://docs.microsoft.com/en-us/windows/win32/wsw/portal)
        pub mod WS;

        /// WinSock
        pub mod WSA;

        /// [Cross-platform Audio Creation Tool (XACT)](https://en.wikipedia.org/wiki/Cross-platform_Audio_Creation_Tool)
        pub mod XACT;

        /// Pre-Vista [Certificate Enrollment Control](https://docs.microsoft.com/en-us/windows/win32/seccertenroll/mapping-xenroll-dll-to-certenroll-dll)
        pub mod XENROLL;
    }
}
