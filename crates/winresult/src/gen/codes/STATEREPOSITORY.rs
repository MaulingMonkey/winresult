// WARNING: this file is auto-generated by xtask gen and may be overwritten

use super::*;


/// Optimistic locking failure. Data cannot be updated if it has changed since it was read.
pub const E_CONCURRENCY_LOCKING_FAILURE : HResultError = HResultError::from_constant(0x80670001); // STATEREPOSITORY_E_CONCURRENCY_LOCKING_FAILURE

/// A prepared statement has been stepped at least once but not run to completion or reset. This may result in busy waits.
pub const E_STATEMENT_INPROGRESS : HResultError = HResultError::from_constant(0x80670002); // STATEREPOSITORY_E_STATEMENT_INPROGRESS

/// The StateRepository configuration is not valid.
pub const E_CONFIGURATION_INVALID : HResultError = HResultError::from_constant(0x80670003); // STATEREPOSITORY_E_CONFIGURATION_INVALID

/// The StateRepository schema version is not known.
pub const E_UNKNOWN_SCHEMA_VERSION : HResultError = HResultError::from_constant(0x80670004); // STATEREPOSITORY_E_UNKNOWN_SCHEMA_VERSION

/// A StateRepository dictionary is not valid.
pub const ERROR_DICTIONARY_CORRUPTED : HResultError = HResultError::from_constant(0x80670005); // STATEREPOSITORY_ERROR_DICTIONARY_CORRUPTED

/// The request failed because the StateRepository is actively blocking requests.
pub const E_BLOCKED : HResultError = HResultError::from_constant(0x80670006); // STATEREPOSITORY_E_BLOCKED

/// The database file is locked. The request will be retried.
pub const E_BUSY_RETRY : HResultError = HResultError::from_constant(0x80670007); // STATEREPOSITORY_E_BUSY_RETRY

/// The database file is locked because another process is busy recovering the database. The request will be retried.
pub const E_BUSY_RECOVERY_RETRY : HResultError = HResultError::from_constant(0x80670008); // STATEREPOSITORY_E_BUSY_RECOVERY_RETRY

/// A table in the database is locked. The request will be retried.
pub const E_LOCKED_RETRY : HResultError = HResultError::from_constant(0x80670009); // STATEREPOSITORY_E_LOCKED_RETRY

/// The shared cache for the database is locked by another connection. The request will be retried.
pub const E_LOCKED_SHAREDCACHE_RETRY : HResultError = HResultError::from_constant(0x8067000A); // STATEREPOSITORY_E_LOCKED_SHAREDCACHE_RETRY

/// A transaction is required to perform the request operation.
pub const E_TRANSACTION_REQUIRED : HResultError = HResultError::from_constant(0x8067000B); // STATEREPOSITORY_E_TRANSACTION_REQUIRED

/// The database file is locked. The request has exceeded the allowed threshold.
pub const E_BUSY_TIMEOUT_EXCEEDED : HResultError = HResultError::from_constant(0x8067000C); // STATEREPOSITORY_E_BUSY_TIMEOUT_EXCEEDED

/// The database file is locked because another process is busy recovering the database. The request has exceeded the allowed threshold.
pub const E_BUSY_RECOVERY_TIMEOUT_EXCEEDED : HResultError = HResultError::from_constant(0x8067000D); // STATEREPOSITORY_E_BUSY_RECOVERY_TIMEOUT_EXCEEDED

/// A table in the database is locked. The request has exceeded the allowed threshold.
pub const E_LOCKED_TIMEOUT_EXCEEDED : HResultError = HResultError::from_constant(0x8067000E); // STATEREPOSITORY_E_LOCKED_TIMEOUT_EXCEEDED

/// The shared cache for the database is locked by another connection. The request has exceeded the allowed threshold.
pub const E_LOCKED_SHAREDCACHE_TIMEOUT_EXCEEDED : HResultError = HResultError::from_constant(0x8067000F); // STATEREPOSITORY_E_LOCKED_SHAREDCACHE_TIMEOUT_EXCEEDED

/// The StateRepository service Stop event is in progress.
pub const E_SERVICE_STOP_IN_PROGRESS : HResultError = HResultError::from_constant(0x80670010); // STATEREPOSITORY_E_SERVICE_STOP_IN_PROGRESS

/// Nested transactions are not supported.
pub const E_NESTED_TRANSACTION_NOT_SUPPORTED : HResultError = HResultError::from_constant(0x80670011); // STATEREPOSTORY_E_NESTED_TRANSACTION_NOT_SUPPORTED

/// The StateRepository cache is not valid.
pub const ERROR_CACHE_CORRUPTED : HResultError = HResultError::from_constant(0x80670012); // STATEREPOSITORY_ERROR_CACHE_CORRUPTED

/// The StateRepository cache is not initialized.
pub const E_CACHE_NOT_INIITALIZED : HResultError = HResultError::from_constant(0x80670015); // STATEREPOSITORY_E_CACHE_NOT_INIITALIZED

/// Package dependency criteria could not be resolved.
pub const E_DEPENDENCY_NOT_RESOLVED : HResultError = HResultError::from_constant(0x80670016); // STATEREPOSITORY_E_DEPENDENCY_NOT_RESOLVED
