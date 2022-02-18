// WARNING: this file is auto-generated by xtask gen and may be overwritten

use super::*;


/// The URL is invalid.
pub const E_INVALID_URL : ErrorHResult = ErrorHResult::from_constant(0x800C0002); // INET_E_INVALID_URL

/// No Internet session has been established.
pub const E_NO_SESSION : ErrorHResult = ErrorHResult::from_constant(0x800C0003); // INET_E_NO_SESSION

/// Unable to connect to the target server.
pub const E_CANNOT_CONNECT : ErrorHResult = ErrorHResult::from_constant(0x800C0004); // INET_E_CANNOT_CONNECT

/// The system cannot locate the resource specified.
pub const E_RESOURCE_NOT_FOUND : ErrorHResult = ErrorHResult::from_constant(0x800C0005); // INET_E_RESOURCE_NOT_FOUND

/// The system cannot locate the object specified.
pub const E_OBJECT_NOT_FOUND : ErrorHResult = ErrorHResult::from_constant(0x800C0006); // INET_E_OBJECT_NOT_FOUND

/// No data is available for the requested resource.
pub const E_DATA_NOT_AVAILABLE : ErrorHResult = ErrorHResult::from_constant(0x800C0007); // INET_E_DATA_NOT_AVAILABLE

/// The download of the specified resource has failed.
pub const E_DOWNLOAD_FAILURE : ErrorHResult = ErrorHResult::from_constant(0x800C0008); // INET_E_DOWNLOAD_FAILURE

/// Authentication is required to access this resource.
pub const E_AUTHENTICATION_REQUIRED : ErrorHResult = ErrorHResult::from_constant(0x800C0009); // INET_E_AUTHENTICATION_REQUIRED

/// The server could not recognize the provided mime type.
pub const E_NO_VALID_MEDIA : ErrorHResult = ErrorHResult::from_constant(0x800C000A); // INET_E_NO_VALID_MEDIA

/// The operation was timed out.
pub const E_CONNECTION_TIMEOUT : ErrorHResult = ErrorHResult::from_constant(0x800C000B); // INET_E_CONNECTION_TIMEOUT

/// The server did not understand the request, or the request was invalid.
pub const E_INVALID_REQUEST : ErrorHResult = ErrorHResult::from_constant(0x800C000C); // INET_E_INVALID_REQUEST

/// The specified protocol is unknown.
pub const E_UNKNOWN_PROTOCOL : ErrorHResult = ErrorHResult::from_constant(0x800C000D); // INET_E_UNKNOWN_PROTOCOL

/// A security problem occurred.
pub const E_SECURITY_PROBLEM : ErrorHResult = ErrorHResult::from_constant(0x800C000E); // INET_E_SECURITY_PROBLEM

/// The system could not load the persisted data.
pub const E_CANNOT_LOAD_DATA : ErrorHResult = ErrorHResult::from_constant(0x800C000F); // INET_E_CANNOT_LOAD_DATA

/// Unable to instantiate the object.
pub const E_CANNOT_INSTANTIATE_OBJECT : ErrorHResult = ErrorHResult::from_constant(0x800C0010); // INET_E_CANNOT_INSTANTIATE_OBJECT

/// Security certificate required to access this resource is invalid.
pub const E_INVALID_CERTIFICATE : ErrorHResult = ErrorHResult::from_constant(0x800C0019); // INET_E_INVALID_CERTIFICATE

/// A redirection problem occurred.
pub const E_REDIRECT_FAILED : ErrorHResult = ErrorHResult::from_constant(0x800C0014); // INET_E_REDIRECT_FAILED

/// The requested resource is a directory, not a file.
pub const E_REDIRECT_TO_DIR : ErrorHResult = ErrorHResult::from_constant(0x800C0015); // INET_E_REDIRECT_TO_DIR