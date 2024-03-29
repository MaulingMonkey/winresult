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
//! |          0 | 0xFFFFFFFF | [`ErrorHResultOrCode`]                      | [`ErrorCode`] \| [`HResultError`]
//!
//! ### Modules of Note
//!
//! | mod                   | types         |
//! | --------------------- | ------------- |
//! | [ERROR]               | [ErrorCode], [HResultError], and [HResultSuccess]\(!\)
//! | [FACILITY]            | [HResultFacilityMicrosoft], [NtStatusFacilityMicrosoft]
//! | [STATUS]              | [NtStatus]
//! | [STATUS::SEVERITY]    | [NtStatusSeverity]
//! | [WAIT]                | [WaitCode]
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

/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/synchapi/nf-synchapi-waitformultipleobjectsex#return-value)\]
/// WAIT_\* values returned by various WaitFor\* and other win32 functions.
#[allow(non_snake_case)]
pub mod WAIT {
    use super::*;

    pub const OBJECT_0      : WaitCode = WaitCode::from_constant(0); // STATUS_WAIT_0

    // DO NOT INCLUDE: CHILD, GRANDCHILD (these are for _cwait, not WaitFor*, and ignored to boot according to process.h!

    pub const ABANDONED_0   : WaitCode = WaitCode::from_constant(0x80); // STATUS_ABANDONED_WAIT_0

    /// The wait was ended by one or more user-mode [asynchronous procedure calls](https://learn.microsoft.com/en-us/windows/desktop/Sync/asynchronous-procedure-calls) (APC) queued to the thread.
    pub const IO_COMPLETION : WaitCode = WaitCode::from_constant(0xC0); // STATUS_USER_APC

    /// The time-out interval elapsed.
    pub const TIMEOUT       : WaitCode = WaitCode::from_constant(258); // = 0x102 = STATUS_TIMEOUT

    // PENDING = 0x103 = 259 (STATUS_*, but no WAIT_*?)

    /// The function has failed. To get extended error information, generally call [GetLastError](https://learn.microsoft.com/en-us/windows/desktop/api/errhandlingapi/nf-errhandlingapi-getlasterror).
    pub const FAILED        : WaitCode = WaitCode::from_constant(0xFFFFFFFF);

    /// WAIT_OBJECT_0 + n
    ///
    /// ### Returns
    /// *   [None]                  if `n >= 64` (MAXIMUM_WAIT_OBJECTS)
    /// *   [Some]\([WaitCode]\)    otherwise
    pub const fn OBJECT(n: u32) -> Option<WaitCode> {
        if n >= MAXIMUM_WAIT_OBJECTS { return None }
        Some(WaitCode::from_constant(WAIT::OBJECT_0.to_u32() + n))
    }

    /// WAIT_ABANDONED_0 + n
    ///
    /// ### Returns
    /// *   [None]                  if `n >= 64` (MAXIMUM_WAIT_OBJECTS)
    /// *   [Some]\([WaitCode]\)    otherwise
    pub const fn ABANDONED(n: u32) -> Option<WaitCode> {
        if n >= MAXIMUM_WAIT_OBJECTS { return None }
        Some(WaitCode::from_constant(WAIT::ABANDONED_0.to_u32() + n))
    }

    /// Maximum number of wait objects
    const MAXIMUM_WAIT_OBJECTS : u32 = 64;
}



mod gen {
    pub mod codes {
        #![allow(non_snake_case)]
        #![allow(non_upper_case_globals)]
        use types::{ErrorCode, HResultSuccess, HResultError, NtStatus, NtStatusSeverity};



        #[path = "STATUS.rs"] mod _STATUS;
        /// \[[microsoft.com](https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-erref/87fba13e-bf06-450e-83b1-9241dc81e781)\]
        /// [NtStatus](types::NtStatus) errors, warnings, and other codes (for use in e.g. Kernel / Drivers)
        pub mod STATUS {
            use super::*;

            /// \[[microsoft.com](https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-erref/87fba13e-bf06-450e-83b1-9241dc81e781)\]
            /// [NtStatus::sev]\(\):
            /// [SUCCESS](Self::SUCCESS)
            /// [INFORMATIONAL](Self::INFORMATIONAL)
            /// [WARNING](Self::WARNING)
            /// [ERROR](Self::ERROR)
            pub mod SEVERITY {
                use super::*;

                pub const SUCCESS       : NtStatusSeverity = NtStatusSeverity::from_constant(0);
                pub const INFORMATIONAL : NtStatusSeverity = NtStatusSeverity::from_constant(1);
                pub const WARNING       : NtStatusSeverity = NtStatusSeverity::from_constant(2);
                pub const ERROR         : NtStatusSeverity = NtStatusSeverity::from_constant(3);
            }

            // TODO: SUCCESS = 0 ?

            pub use super::_STATUS::*;
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

        /// [COM Categories](https://learn.microsoft.com/en-us/windows/win32/api/comcat/)
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

        /// [COM Admin / Catalog](https://learn.microsoft.com/en-us/windows/win32/api/comadmin/)
        pub mod COMADMIN;

        /// [COM+ Queued Components Protocol](https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-wpo/55d601ed-c63b-485b-8648-53866b3e8e21)
        pub mod COMQC;

        /// [DCOM Context](https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-dcom/94a587a3-826a-4bac-969b-ae0bbfc9a663)?
        pub mod CONTEXT;
        pub mod CONVERT10;
        pub mod CRYPT;
        pub mod CS;

        /// [Direct3D](https://learn.microsoft.com/en-us/windows/win32/direct3d)
        pub mod D3D { use super::*; pub const OK : HResultSuccess = HResultSuccess::from_constant(0); }

        /// [Direct3D](https://learn.microsoft.com/en-us/windows/win32/direct3d) 10+
        pub mod D3D10;

        /// [Direct3D 11](https://learn.microsoft.com/en-us/windows/win32/direct3d11/atoc-dx-graphics-direct3d-11)+
        pub mod D3D11;

        /// [Direct3D 12](https://learn.microsoft.com/en-us/windows/win32/direct3d12/direct3d-12-graphics)+
        pub mod D3D12;

        /// [Direct3D](https://learn.microsoft.com/en-us/windows/win32/direct3d) Errors
        pub mod D3DERR;

        /// [D3DX](https://en.wikipedia.org/wiki/D3DX) Errors
        pub mod D3DXERR;

        /// [D3DX](https://en.wikipedia.org/wiki/D3DX) `.X` file type errors
        pub mod D3DXFERR;

        /// [Direct3D](https://learn.microsoft.com/en-us/windows/win32/direct3d)
        pub mod D3DOK;

        /// OLE / Clipboard Stuff?
        pub mod DATA;

        /// [DirectComposition](https://learn.microsoft.com/en-us/windows/win32/directcomp/directcomposition-portal)
        pub mod DCOMPOSITION;

        /// Digital Signature
        pub mod DIGSIG;

        /// [IDispatch](https://learn.microsoft.com/en-us/windows/win32/api/oaidl/nf-oaidl-idispatch-invoke)
        pub mod DISP;

        /// Domain Name Services
        pub mod DNS;

        /// [Drag and Drop](https://learn.microsoft.com/en-us/windows/win32/com/drag-and-drop)
        pub mod DRAGDROP;

        /// OLE / Data Values / Clipboard Stuff?
        pub mod DV;

        /// Display Window Manager (desktop rendering composition)
        pub mod DWM;

        /// [DirectWrite](https://learn.microsoft.com/en-us/windows/win32/directwrite/direct-write-portal)
        pub mod DWRITE;

        /// DirectX
        pub mod DXCORE;

        /// [DXGI](https://learn.microsoft.com/en-us/windows/win32/direct3ddxgi/dx-graphics-dxgi)
        pub mod DXGI;

        /// **Errors Codes**.  Typically [HResultError]s.
        pub mod E;

        /// Exchange ActiveSync
        pub mod EAS;

        /// DO NOT EXPOSE THIS MESS AS IS.  See [doc/ept-and-rpc-codes-are-evil.md](https://github.com/MaulingMonkey/winresult/blob/5094a8a5568392ef855babd8bc62458f29153e46/crates/winresult/doc/ept-and-rpc-codes-are-evil.md) for details.
        ///
        /// **E**ntry **P**oin**t** for Remote Procedure Calls
        #[allow(dead_code)] mod EPT {}

        #[path = "ERROR.rs"] mod _ERROR;
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

            pub use super::_ERROR::*;

            /// WinSpool / Printer related
            pub mod BIDI;

            /// [OneDrive](https://en.wikipedia.org/wiki/OneDrive) / [Cloud Filter API](https://learn.microsoft.com/en-us/windows/win32/api/_cloudapi/)
            pub mod CLOUD_FILE;

            /// [Windows Clustering](https://learn.microsoft.com/en-us/windows/win32/api/_mscs/)
            pub mod CLUSTER;

            /// [Debugging](https://learn.microsoft.com/en-us/windows/win32/debug/debugging-functions)
            pub mod DBG;

            /// DHCP-related?
            pub mod DDS;

            /// DHCP
            pub mod DHCP;

            /// [Domain Services](https://en.wikipedia.org/wiki/Active_Directory#Domain_Services)
            pub mod DS;

            /// [Windows Event Log](https://learn.microsoft.com/en-us/windows/win32/wes/windows-event-log-error-constants)
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

            /// [Package Resource Indexing](https://learn.microsoft.com/en-us/windows/win32/menurc/pri-indexing-reference)
            pub mod MRM;

            /// [Multilingual User Interface](https://en.wikipedia.org/wiki/Multilingual_User_Interface)
            pub mod MUI;

            /// Network Driver Interface Services
            pub mod NDIS;

            pub mod PATCH;

            /// PatchWiz
            pub mod PCW;

            /// [Package Resource Indexing](https://learn.microsoft.com/en-us/windows/win32/menurc/pri-indexing-reference)
            pub mod PRI_MERGE;

            /// [Secure Boot](https://learn.microsoft.com/en-us/windows-hardware/design/device-experiences/oem-secure-boot)
            pub mod SECUREBOOT;

            pub mod SERVER;

            /// [Service Application](https://learn.microsoft.com/en-us/windows/win32/services/services)
            pub mod SERVICE;

            /// [Shared Virtual Hard Disk](https://learn.microsoft.com/en-us/previous-versions/windows/it-pro/windows-server-2012-r2-and-2012/dn281956(v=ws.11))s (w/ \*.vhdx)
            pub mod SVHDX;

            #[path = "SXS.rs"] mod _SXS;
            #[path = "../ERROR_SXS_XML.rs"] mod _SXS_XML;

            /// [Side-by-side assembly](https://en.wikipedia.org/wiki/Side-by-side_assembly)
            pub mod SXS {
                pub use super::_SXS::*;

                /// Manifest parsing errors
                pub mod XML {
                    pub use super::super::_SXS_XML::*;
                }
            }

            /// Virtual Hard Disk (\*.vhd)
            pub mod VHD;

            pub mod WINHTTP;

            /// WinRS / WinRM shell/client for WS-Management Service?
            pub mod WINRS;

            /// [Windows Management Instrumentation](https://learn.microsoft.com/en-us/windows/win32/wmisdk/wmi-start-page)
            pub mod WMI;

            /// WS-Management Service
            pub mod WSMAN;
        }

        /// [COM Events](https://learn.microsoft.com/en-us/windows/win32/api/eventsys/)
        pub mod EVENT;

        pub mod FA;

        /// Full Volume Encryption / [Bitlocker](https://en.wikipedia.org/wiki/BitLocker)
        pub mod FVE;

        /// Windows Filtering Platform
        pub mod FWP; // https://learn.microsoft.com/en-us/windows/win32/fwp/wfp-error-codes

        /// [Host Computer Network](https://learn.microsoft.com/en-us/windows-server/networking/technologies/hcn/hcn-top)
        pub mod GCN;

        /// [Host Computer Network](https://learn.microsoft.com/en-us/windows-server/networking/technologies/hcn/hcn-top)
        pub mod HCN;

        /// [Host Compute System](https://learn.microsoft.com/en-us/virtualization/api/hcs/overview)
        pub mod HCS;

        pub mod HSP;

        /// [WinINet / WinHTTP](https://learn.microsoft.com/en-us/windows/win32/wininet/wininet-vs-winhttp)
        pub mod HTTP;

        /// [WinINet](https://learn.microsoft.com/en-us/windows/win32/wininet/about-wininet)
        pub mod INET;

        pub mod INPLACE;

        pub mod INPUT;

        /// [I/O Ring](https://learn.microsoft.com/en-us/windows/win32/api/ioringapi/)
        pub mod IORING;

        pub mod JSCRIPT;
        pub mod MEM;

        /// [**M**obile Device Management (MDM)](https://learn.microsoft.com/en-us/windows/client-management/mdm/mdm-overview) **Enroll**ment
        pub mod MENROLL;

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

        /// [Peer Distribution](https://learn.microsoft.com/en-us/windows/win32/p2psdk/peer-distribution)
        pub mod PEERDIST;

        pub mod PERSIST;

        /// [Performance Logs and Alerts](https://learn.microsoft.com/en-us/previous-versions/windows/desktop/pla/using-performance-logs-and-alerts)
        pub mod PLA;

        pub mod PRESENTATION;

        /// COM+ registration database
        pub mod REGDB;

        /// WinRT COM
        pub mod RO;

        /// DO NOT EXPOSE THIS MESS AS IS.  See [doc/ept-and-rpc-codes-are-evil.md](https://github.com/MaulingMonkey/winresult/blob/5094a8a5568392ef855babd8bc62458f29153e46/crates/winresult/doc/ept-and-rpc-codes-are-evil.md) for details.
        ///
        /// Remote Procedure Call
        #[allow(dead_code)] mod RPC {}

        /// [Smart Card](https://learn.microsoft.com/en-us/windows/security/identity-protection/smart-cards/smart-card-windows-smart-card-technical-reference)
        pub mod SCARD;

        /// Task Scheduler
        pub mod SCHED;

        pub mod SDIAG;
        pub mod SEC;
        pub mod SPAPI;

        /// [SQLite](https://www.sqlite.org/index.html)
        pub mod SQLITE;

        pub mod STATEREPOSITORY;

        /// [Structured Storage](https://learn.microsoft.com/en-us/windows/win32/stg/functions)?
        pub mod STG;

        /// [Microsoft Store](https://en.wikipedia.org/wiki/Microsoft_Store)?
        pub mod STORE;

        pub mod TBS;
        pub mod TBSIMP;

        /// Tablet PC
        pub mod TPC;

        /// [Trusted Platform Module 2.0](https://learn.microsoft.com/en-us/windows-hardware/design/device-experiences/oem-tpm)
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

        /// [WinINet / WinHTTP](https://learn.microsoft.com/en-us/windows/win32/wininet/wininet-vs-winhttp)
        pub mod WEB;

        /// Wired Equivalent Privacy
        pub mod WEP;

        /// Windows Error Reporting
        pub mod WER;

        /// Windows Hypervisor Platform
        pub mod WHV;

        /// [WinINet](https://learn.microsoft.com/en-us/windows/win32/wininet/about-wininet)
        pub mod WININET;

        /// Windows Push Notifications?
        pub mod WPN;

        /// [Windows Web Services](https://learn.microsoft.com/en-us/windows/win32/wsw/portal)
        pub mod WS;

        /// WinSock
        pub mod WSA;

        /// [Cross-platform Audio Creation Tool (XACT)](https://en.wikipedia.org/wiki/Cross-platform_Audio_Creation_Tool)
        pub mod XACT;

        /// [XAudio 2](https://learn.microsoft.com/en-us/windows/win32/xaudio2/xaudio2-introduction)
        pub mod XAUDIO2;

        /// Pre-Vista [Certificate Enrollment Control](https://learn.microsoft.com/en-us/windows/win32/seccertenroll/mapping-xenroll-dll-to-certenroll-dll)
        pub mod XENROLL;
    }
}
