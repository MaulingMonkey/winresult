// WARNING: this file is auto-generated by xtask gen and may be overwritten

use super::*;


/// The specified capability does not exist.
pub const E_UNKNOWN_CAPABILITY : HResultError = HResultError::from_constant(0x80370300); // WHV_E_UNKNOWN_CAPABILITY

/// The specified buffer is too small for the requested data.
pub const E_INSUFFICIENT_BUFFER : HResultError = HResultError::from_constant(0x80370301); // WHV_E_INSUFFICIENT_BUFFER

/// The specified property does not exist.
pub const E_UNKNOWN_PROPERTY : HResultError = HResultError::from_constant(0x80370302); // WHV_E_UNKNOWN_PROPERTY

/// The configuration of the hypervisor on this system is not supported.
pub const E_UNSUPPORTED_HYPERVISOR_CONFIG : HResultError = HResultError::from_constant(0x80370303); // WHV_E_UNSUPPORTED_HYPERVISOR_CONFIG

/// The configuration of the partition is not valid.
pub const E_INVALID_PARTITION_CONFIG : HResultError = HResultError::from_constant(0x80370304); // WHV_E_INVALID_PARTITION_CONFIG

/// The specified GPA range was not found.
pub const E_GPA_RANGE_NOT_FOUND : HResultError = HResultError::from_constant(0x80370305); // WHV_E_GPA_RANGE_NOT_FOUND

/// A virtual processor with the specified index already exists.
pub const E_VP_ALREADY_EXISTS : HResultError = HResultError::from_constant(0x80370306); // WHV_E_VP_ALREADY_EXISTS

/// A virtual processor with the specified index does not exist.
pub const E_VP_DOES_NOT_EXIST : HResultError = HResultError::from_constant(0x80370307); // WHV_E_VP_DOES_NOT_EXIST

/// The virtual processor is not in the correct state to perform the requested operation.
pub const E_INVALID_VP_STATE : HResultError = HResultError::from_constant(0x80370308); // WHV_E_INVALID_VP_STATE

/// A virtual processor register with the specified name does not exist.
pub const E_INVALID_VP_REGISTER_NAME : HResultError = HResultError::from_constant(0x80370309); // WHV_E_INVALID_VP_REGISTER_NAME

/// The Windows Hypervisor Platform is not supported due to a processor limitation.
pub const E_UNSUPPORTED_PROCESSOR_CONFIG : HResultError = HResultError::from_constant(0x80370310); // WHV_E_UNSUPPORTED_PROCESSOR_CONFIG
