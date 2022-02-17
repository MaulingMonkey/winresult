// WARNING: this file is auto-generated by xtask gen and may be overwritten

use super::*;


/// Typename or Namespace was not found in metadata file.
pub const E_METADATA_NAME_NOT_FOUND : ErrorHResult = ErrorHResult::from_constant(0x8000000F); // RO_E_METADATA_NAME_NOT_FOUND

/// Name is an existing namespace rather than a typename.
pub const E_METADATA_NAME_IS_NAMESPACE : ErrorHResult = ErrorHResult::from_constant(0x80000010); // RO_E_METADATA_NAME_IS_NAMESPACE

/// Typename has an invalid format.
pub const E_METADATA_INVALID_TYPE_FORMAT : ErrorHResult = ErrorHResult::from_constant(0x80000011); // RO_E_METADATA_INVALID_TYPE_FORMAT

/// Metadata file is invalid or corrupted.
pub const E_INVALID_METADATA_FILE : ErrorHResult = ErrorHResult::from_constant(0x80000012); // RO_E_INVALID_METADATA_FILE

/// The object has been closed.
pub const E_CLOSED : ErrorHResult = ErrorHResult::from_constant(0x80000013); // RO_E_CLOSED

/// Only one thread may access the object during a write operation.
pub const E_EXCLUSIVE_WRITE : ErrorHResult = ErrorHResult::from_constant(0x80000014); // RO_E_EXCLUSIVE_WRITE

/// Operation is prohibited during change notification.
pub const E_CHANGE_NOTIFICATION_IN_PROGRESS : ErrorHResult = ErrorHResult::from_constant(0x80000015); // RO_E_CHANGE_NOTIFICATION_IN_PROGRESS

/// The text associated with this error code could not be found.
pub const E_ERROR_STRING_NOT_FOUND : ErrorHResult = ErrorHResult::from_constant(0x80000016); // RO_E_ERROR_STRING_NOT_FOUND

/// The object must support the IAgileObject interface
pub const E_MUST_BE_AGILE : ErrorHResult = ErrorHResult::from_constant(0x8000001C); // RO_E_MUST_BE_AGILE

/// Activating a single-threaded class from MTA is not supported
pub const E_UNSUPPORTED_FROM_MTA : ErrorHResult = ErrorHResult::from_constant(0x8000001D); // RO_E_UNSUPPORTED_FROM_MTA

/// The object has been committed.
pub const E_COMMITTED : ErrorHResult = ErrorHResult::from_constant(0x8000001E); // RO_E_COMMITTED

/// A COM call to an ASTA was blocked because the call chain originated in or passed through another ASTA. This call pattern is deadlock-prone and disallowed by apartment call control.
pub const E_BLOCKED_CROSS_ASTA_CALL : ErrorHResult = ErrorHResult::from_constant(0x8000001F); // RO_E_BLOCKED_CROSS_ASTA_CALL

/// A universal application process cannot activate a packaged WinRT server that is declared to run full trust.
pub const E_CANNOT_ACTIVATE_FULL_TRUST_SERVER : ErrorHResult = ErrorHResult::from_constant(0x80000020); // RO_E_CANNOT_ACTIVATE_FULL_TRUST_SERVER

/// A full trust packaged application process cannot activate a packaged WinRT server unless it is also declared to run full trust.
pub const E_CANNOT_ACTIVATE_UNIVERSAL_APPLICATION_SERVER : ErrorHResult = ErrorHResult::from_constant(0x80000021); // RO_E_CANNOT_ACTIVATE_UNIVERSAL_APPLICATION_SERVER
