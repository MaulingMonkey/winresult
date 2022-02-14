// WARNING: this file is auto-generated by xtask gen and may be overwritten

use super::*;


/// ### Desktop composition is disabled
/// The operation could not be completed because desktop composition is disabled.
pub const E_COMPOSITIONDISABLED : HRESULT = HRESULT::from_constant(0x80263001); // DWM_E_COMPOSITIONDISABLED

/// ### Some desktop composition APIs are not supported while remoting
/// The operation is not supported while running in a remote session.
pub const E_REMOTING_NOT_SUPPORTED : HRESULT = HRESULT::from_constant(0x80263002); // DWM_E_REMOTING_NOT_SUPPORTED

/// ### No DWM redirection surface is available
/// The DWM was unable to provide a redirection surface to complete the DirectX present.
pub const E_NO_REDIRECTION_SURFACE_AVAILABLE : HRESULT = HRESULT::from_constant(0x80263003); // DWM_E_NO_REDIRECTION_SURFACE_AVAILABLE

/// ### DWM is not queuing presents for the specified window
/// The window specified is not currently using queued presents.
pub const E_NOT_QUEUING_PRESENTS : HRESULT = HRESULT::from_constant(0x80263004); // DWM_E_NOT_QUEUING_PRESENTS

/// ### The adapter specified by the LUID is not found
/// DWM can not find the adapter specified by the LUID.
pub const E_ADAPTER_NOT_FOUND : HRESULT = HRESULT::from_constant(0x80263005); // DWM_E_ADAPTER_NOT_FOUND

/// ### GDI redirection surface was returned
/// GDI redirection surface of the top level window was returned.
pub const S_GDI_REDIRECTION_SURFACE : HRESULT = HRESULT::from_constant(0x00263005); // DWM_S_GDI_REDIRECTION_SURFACE

/// ### Redirection surface can not be created.  The size of the surface is larger than what is supported on this machine
/// Redirection surface can not be created.  The size of the surface is larger than what is supported on this machine.
pub const E_TEXTURE_TOO_LARGE : HRESULT = HRESULT::from_constant(0x80263007); // DWM_E_TEXTURE_TOO_LARGE

/// ### GDI redirection surface is either on a different adapter or in system memory. Perform blt via GDI
/// GDI redirection surface is either on a different adapter or in system memory. Perform blt via GDI.
pub const S_GDI_REDIRECTION_SURFACE_BLT_VIA_GDI : HRESULT = HRESULT::from_constant(0x00263008); // DWM_S_GDI_REDIRECTION_SURFACE_BLT_VIA_GDI
