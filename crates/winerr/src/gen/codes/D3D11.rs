// WARNING: this file is auto-generated by xtask gen and may be overwritten

use super::*;


/// The application has exceeded the maximum number of unique state objects per Direct3D device.
/// The limit is 4096 for feature levels up to 11.1.
pub const ERROR_TOO_MANY_UNIQUE_STATE_OBJECTS : ErrorHResult = ErrorHResult::from_constant(0x887C0001); // D3D11_ERROR_TOO_MANY_UNIQUE_STATE_OBJECTS

/// The specified file was not found.
pub const ERROR_FILE_NOT_FOUND : ErrorHResult = ErrorHResult::from_constant(0x887C0002); // D3D11_ERROR_FILE_NOT_FOUND

/// The application has exceeded the maximum number of unique view objects per Direct3D device.
/// The limit is 2^20 for feature levels up to 11.1.
pub const ERROR_TOO_MANY_UNIQUE_VIEW_OBJECTS : ErrorHResult = ErrorHResult::from_constant(0x887C0003); // D3D11_ERROR_TOO_MANY_UNIQUE_VIEW_OBJECTS

/// The application's first call per command list to Map on a deferred context did not use D3D11_MAP_WRITE_DISCARD.
pub const ERROR_DEFERRED_CONTEXT_MAP_WITHOUT_INITIAL_DISCARD : ErrorHResult = ErrorHResult::from_constant(0x887C0004); // D3D11_ERROR_DEFERRED_CONTEXT_MAP_WITHOUT_INITIAL_DISCARD
