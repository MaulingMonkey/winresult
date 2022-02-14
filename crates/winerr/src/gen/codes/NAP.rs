// WARNING: this file is auto-generated by xtask gen and may be overwritten

use super::*;


/// The NAP SoH packet is invalid.
pub const E_INVALID_PACKET : HRESULT = HRESULT::from_constant(0x80270001); // NAP_E_INVALID_PACKET

/// An SoH was missing from the NAP packet.
pub const E_MISSING_SOH : HRESULT = HRESULT::from_constant(0x80270002); // NAP_E_MISSING_SOH

/// The entity ID conflicts with an already registered id.
pub const E_CONFLICTING_ID : HRESULT = HRESULT::from_constant(0x80270003); // NAP_E_CONFLICTING_ID

/// No cached SoH is present.
pub const E_NO_CACHED_SOH : HRESULT = HRESULT::from_constant(0x80270004); // NAP_E_NO_CACHED_SOH

/// The entity is still bound to the NAP system.
pub const E_STILL_BOUND : HRESULT = HRESULT::from_constant(0x80270005); // NAP_E_STILL_BOUND

/// The entity is not registered with the NAP system.
pub const E_NOT_REGISTERED : HRESULT = HRESULT::from_constant(0x80270006); // NAP_E_NOT_REGISTERED

/// The entity is not initialized with the NAP system.
pub const E_NOT_INITIALIZED : HRESULT = HRESULT::from_constant(0x80270007); // NAP_E_NOT_INITIALIZED

/// The correlation id in the SoH-Request and SoH-Response do not match up.
pub const E_MISMATCHED_ID : HRESULT = HRESULT::from_constant(0x80270008); // NAP_E_MISMATCHED_ID

/// Completion was indicated on a request that is not currently pending.
pub const E_NOT_PENDING : HRESULT = HRESULT::from_constant(0x80270009); // NAP_E_NOT_PENDING

/// The NAP component's id was not found.
pub const E_ID_NOT_FOUND : HRESULT = HRESULT::from_constant(0x8027000A); // NAP_E_ID_NOT_FOUND

/// The maximum size of the connection is too small for an SoH packet.
pub const E_MAXSIZE_TOO_SMALL : HRESULT = HRESULT::from_constant(0x8027000B); // NAP_E_MAXSIZE_TOO_SMALL

/// The NapAgent service is not running.
pub const E_SERVICE_NOT_RUNNING : HRESULT = HRESULT::from_constant(0x8027000C); // NAP_E_SERVICE_NOT_RUNNING

/// A certificate is already present in the cert store.
pub const S_CERT_ALREADY_PRESENT : HRESULT = HRESULT::from_constant(0x0027000D); // NAP_S_CERT_ALREADY_PRESENT

/// The entity is disabled with the NapAgent service.
pub const E_ENTITY_DISABLED : HRESULT = HRESULT::from_constant(0x8027000E); // NAP_E_ENTITY_DISABLED

/// Group Policy is not configured.
pub const E_NETSH_GROUPPOLICY_ERROR : HRESULT = HRESULT::from_constant(0x8027000F); // NAP_E_NETSH_GROUPPOLICY_ERROR

/// Too many simultaneous calls.
pub const E_TOO_MANY_CALLS : HRESULT = HRESULT::from_constant(0x80270010); // NAP_E_TOO_MANY_CALLS

/// SHV configuration already existed.
pub const E_SHV_CONFIG_EXISTED : HRESULT = HRESULT::from_constant(0x80270011); // NAP_E_SHV_CONFIG_EXISTED

/// SHV configuration is not found.
pub const E_SHV_CONFIG_NOT_FOUND : HRESULT = HRESULT::from_constant(0x80270012); // NAP_E_SHV_CONFIG_NOT_FOUND

/// SHV timed out on the request.
pub const E_SHV_TIMEOUT : HRESULT = HRESULT::from_constant(0x80270013); // NAP_E_SHV_TIMEOUT
