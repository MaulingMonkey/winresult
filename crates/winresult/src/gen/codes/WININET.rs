// WARNING: this file is auto-generated by xtask gen and may be overwritten

use super::*;


/// No more Internet handles can be allocated
pub const E_OUT_OF_HANDLES : HResultError = HResultError::from_constant(0x80072EE1); // WININET_E_OUT_OF_HANDLES

/// The operation timed out
pub const E_TIMEOUT : HResultError = HResultError::from_constant(0x80072EE2); // WININET_E_TIMEOUT

/// The server returned extended information
pub const E_EXTENDED_ERROR : HResultError = HResultError::from_constant(0x80072EE3); // WININET_E_EXTENDED_ERROR

/// An internal error occurred in the Microsoft Internet extensions
pub const E_INTERNAL_ERROR : HResultError = HResultError::from_constant(0x80072EE4); // WININET_E_INTERNAL_ERROR

/// The URL is invalid
pub const E_INVALID_URL : HResultError = HResultError::from_constant(0x80072EE5); // WININET_E_INVALID_URL

/// The URL does not use a recognized protocol
pub const E_UNRECOGNIZED_SCHEME : HResultError = HResultError::from_constant(0x80072EE6); // WININET_E_UNRECOGNIZED_SCHEME

/// The server name or address could not be resolved
pub const E_NAME_NOT_RESOLVED : HResultError = HResultError::from_constant(0x80072EE7); // WININET_E_NAME_NOT_RESOLVED

/// A protocol with the required capabilities was not found
pub const E_PROTOCOL_NOT_FOUND : HResultError = HResultError::from_constant(0x80072EE8); // WININET_E_PROTOCOL_NOT_FOUND

/// The option is invalid
pub const E_INVALID_OPTION : HResultError = HResultError::from_constant(0x80072EE9); // WININET_E_INVALID_OPTION

/// The length is incorrect for the option type
pub const E_BAD_OPTION_LENGTH : HResultError = HResultError::from_constant(0x80072EEA); // WININET_E_BAD_OPTION_LENGTH

/// The option value cannot be set
pub const E_OPTION_NOT_SETTABLE : HResultError = HResultError::from_constant(0x80072EEB); // WININET_E_OPTION_NOT_SETTABLE

/// Microsoft Internet Extension support has been shut down
pub const E_SHUTDOWN : HResultError = HResultError::from_constant(0x80072EEC); // WININET_E_SHUTDOWN

/// The user name was not allowed
pub const E_INCORRECT_USER_NAME : HResultError = HResultError::from_constant(0x80072EED); // WININET_E_INCORRECT_USER_NAME

/// The password was not allowed
pub const E_INCORRECT_PASSWORD : HResultError = HResultError::from_constant(0x80072EEE); // WININET_E_INCORRECT_PASSWORD

/// The login request was denied
pub const E_LOGIN_FAILURE : HResultError = HResultError::from_constant(0x80072EEF); // WININET_E_LOGIN_FAILURE

/// The requested operation is invalid
pub const E_INVALID_OPERATION : HResultError = HResultError::from_constant(0x80072EF0); // WININET_E_INVALID_OPERATION

/// The operation has been canceled
pub const E_OPERATION_CANCELLED : HResultError = HResultError::from_constant(0x80072EF1); // WININET_E_OPERATION_CANCELLED

/// The supplied handle is the wrong type for the requested operation
pub const E_INCORRECT_HANDLE_TYPE : HResultError = HResultError::from_constant(0x80072EF2); // WININET_E_INCORRECT_HANDLE_TYPE

/// The handle is in the wrong state for the requested operation
pub const E_INCORRECT_HANDLE_STATE : HResultError = HResultError::from_constant(0x80072EF3); // WININET_E_INCORRECT_HANDLE_STATE

/// The request cannot be made on a Proxy session
pub const E_NOT_PROXY_REQUEST : HResultError = HResultError::from_constant(0x80072EF4); // WININET_E_NOT_PROXY_REQUEST

/// The registry value could not be found
pub const E_REGISTRY_VALUE_NOT_FOUND : HResultError = HResultError::from_constant(0x80072EF5); // WININET_E_REGISTRY_VALUE_NOT_FOUND

/// The registry parameter is incorrect
pub const E_BAD_REGISTRY_PARAMETER : HResultError = HResultError::from_constant(0x80072EF6); // WININET_E_BAD_REGISTRY_PARAMETER

/// Direct Internet access is not available
pub const E_NO_DIRECT_ACCESS : HResultError = HResultError::from_constant(0x80072EF7); // WININET_E_NO_DIRECT_ACCESS

/// No context value was supplied
pub const E_NO_CONTEXT : HResultError = HResultError::from_constant(0x80072EF8); // WININET_E_NO_CONTEXT

/// No status callback was supplied
pub const E_NO_CALLBACK : HResultError = HResultError::from_constant(0x80072EF9); // WININET_E_NO_CALLBACK

/// There are outstanding requests
pub const E_REQUEST_PENDING : HResultError = HResultError::from_constant(0x80072EFA); // WININET_E_REQUEST_PENDING

/// The information format is incorrect
pub const E_INCORRECT_FORMAT : HResultError = HResultError::from_constant(0x80072EFB); // WININET_E_INCORRECT_FORMAT

/// The requested item could not be found
pub const E_ITEM_NOT_FOUND : HResultError = HResultError::from_constant(0x80072EFC); // WININET_E_ITEM_NOT_FOUND

/// A connection with the server could not be established
pub const E_CANNOT_CONNECT : HResultError = HResultError::from_constant(0x80072EFD); // WININET_E_CANNOT_CONNECT

/// The connection with the server was terminated abnormally
pub const E_CONNECTION_ABORTED : HResultError = HResultError::from_constant(0x80072EFE); // WININET_E_CONNECTION_ABORTED

/// The connection with the server was reset
pub const E_CONNECTION_RESET : HResultError = HResultError::from_constant(0x80072EFF); // WININET_E_CONNECTION_RESET

/// The action must be retried
pub const E_FORCE_RETRY : HResultError = HResultError::from_constant(0x80072F00); // WININET_E_FORCE_RETRY

/// The proxy request is invalid
pub const E_INVALID_PROXY_REQUEST : HResultError = HResultError::from_constant(0x80072F01); // WININET_E_INVALID_PROXY_REQUEST

/// User interaction is required to complete the operation
pub const E_NEED_UI : HResultError = HResultError::from_constant(0x80072F02); // WININET_E_NEED_UI

/// The handle already exists
pub const E_HANDLE_EXISTS : HResultError = HResultError::from_constant(0x80072F04); // WININET_E_HANDLE_EXISTS

/// The date in the certificate is invalid or has expired
pub const E_SEC_CERT_DATE_INVALID : HResultError = HResultError::from_constant(0x80072F05); // WININET_E_SEC_CERT_DATE_INVALID

/// The host name in the certificate is invalid or does not match
pub const E_SEC_CERT_CN_INVALID : HResultError = HResultError::from_constant(0x80072F06); // WININET_E_SEC_CERT_CN_INVALID

/// A redirect request will change a non-secure to a secure connection
pub const E_HTTP_TO_HTTPS_ON_REDIR : HResultError = HResultError::from_constant(0x80072F07); // WININET_E_HTTP_TO_HTTPS_ON_REDIR

/// A redirect request will change a secure to a non-secure connection
pub const E_HTTPS_TO_HTTP_ON_REDIR : HResultError = HResultError::from_constant(0x80072F08); // WININET_E_HTTPS_TO_HTTP_ON_REDIR

/// Mixed secure and non-secure connections
pub const E_MIXED_SECURITY : HResultError = HResultError::from_constant(0x80072F09); // WININET_E_MIXED_SECURITY

/// Changing to non-secure post
pub const E_CHG_POST_IS_NON_SECURE : HResultError = HResultError::from_constant(0x80072F0A); // WININET_E_CHG_POST_IS_NON_SECURE

/// Data is being posted on a non-secure connection
pub const E_POST_IS_NON_SECURE : HResultError = HResultError::from_constant(0x80072F0B); // WININET_E_POST_IS_NON_SECURE

/// A certificate is required to complete client authentication
pub const E_CLIENT_AUTH_CERT_NEEDED : HResultError = HResultError::from_constant(0x80072F0C); // WININET_E_CLIENT_AUTH_CERT_NEEDED

/// The certificate authority is invalid or incorrect
pub const E_INVALID_CA : HResultError = HResultError::from_constant(0x80072F0D); // WININET_E_INVALID_CA

/// Client authentication has not been correctly installed
pub const E_CLIENT_AUTH_NOT_SETUP : HResultError = HResultError::from_constant(0x80072F0E); // WININET_E_CLIENT_AUTH_NOT_SETUP

/// An error has occurred in a Wininet asynchronous thread. You may need to restart
pub const E_ASYNC_THREAD_FAILED : HResultError = HResultError::from_constant(0x80072F0F); // WININET_E_ASYNC_THREAD_FAILED

/// The protocol scheme has changed during a redirect operation
pub const E_REDIRECT_SCHEME_CHANGE : HResultError = HResultError::from_constant(0x80072F10); // WININET_E_REDIRECT_SCHEME_CHANGE

/// There are operations awaiting retry
pub const E_DIALOG_PENDING : HResultError = HResultError::from_constant(0x80072F11); // WININET_E_DIALOG_PENDING

/// The operation must be retried
pub const E_RETRY_DIALOG : HResultError = HResultError::from_constant(0x80072F12); // WININET_E_RETRY_DIALOG

/// There are no new cache containers
pub const E_NO_NEW_CONTAINERS : HResultError = HResultError::from_constant(0x80072F13); // WININET_E_NO_NEW_CONTAINERS

/// A security zone check indicates the operation must be retried
pub const E_HTTPS_HTTP_SUBMIT_REDIR : HResultError = HResultError::from_constant(0x80072F14); // WININET_E_HTTPS_HTTP_SUBMIT_REDIR

/// The SSL certificate contains errors.
pub const E_SEC_CERT_ERRORS : HResultError = HResultError::from_constant(0x80072F17); // WININET_E_SEC_CERT_ERRORS

/// It was not possible to connect to the revocation server or a definitive response could not be obtained.
pub const E_SEC_CERT_REV_FAILED : HResultError = HResultError::from_constant(0x80072F19); // WININET_E_SEC_CERT_REV_FAILED

/// The requested header was not found
pub const E_HEADER_NOT_FOUND : HResultError = HResultError::from_constant(0x80072F76); // WININET_E_HEADER_NOT_FOUND

/// The server does not support the requested protocol level
pub const E_DOWNLEVEL_SERVER : HResultError = HResultError::from_constant(0x80072F77); // WININET_E_DOWNLEVEL_SERVER

/// The server returned an invalid or unrecognized response
pub const E_INVALID_SERVER_RESPONSE : HResultError = HResultError::from_constant(0x80072F78); // WININET_E_INVALID_SERVER_RESPONSE

/// The supplied HTTP header is invalid
pub const E_INVALID_HEADER : HResultError = HResultError::from_constant(0x80072F79); // WININET_E_INVALID_HEADER

/// The request for a HTTP header is invalid
pub const E_INVALID_QUERY_REQUEST : HResultError = HResultError::from_constant(0x80072F7A); // WININET_E_INVALID_QUERY_REQUEST

/// The HTTP header already exists
pub const E_HEADER_ALREADY_EXISTS : HResultError = HResultError::from_constant(0x80072F7B); // WININET_E_HEADER_ALREADY_EXISTS

/// The HTTP redirect request failed
pub const E_REDIRECT_FAILED : HResultError = HResultError::from_constant(0x80072F7C); // WININET_E_REDIRECT_FAILED

/// An error occurred in the secure channel support
pub const E_SECURITY_CHANNEL_ERROR : HResultError = HResultError::from_constant(0x80072F7D); // WININET_E_SECURITY_CHANNEL_ERROR

/// The file could not be written to the cache
pub const E_UNABLE_TO_CACHE_FILE : HResultError = HResultError::from_constant(0x80072F7E); // WININET_E_UNABLE_TO_CACHE_FILE

/// The TCP/IP protocol is not installed properly
pub const E_TCPIP_NOT_INSTALLED : HResultError = HResultError::from_constant(0x80072F7F); // WININET_E_TCPIP_NOT_INSTALLED

/// The computer is disconnected from the network
pub const E_DISCONNECTED : HResultError = HResultError::from_constant(0x80072F83); // WININET_E_DISCONNECTED

/// The server is unreachable
pub const E_SERVER_UNREACHABLE : HResultError = HResultError::from_constant(0x80072F84); // WININET_E_SERVER_UNREACHABLE

/// The proxy server is unreachable
pub const E_PROXY_SERVER_UNREACHABLE : HResultError = HResultError::from_constant(0x80072F85); // WININET_E_PROXY_SERVER_UNREACHABLE

/// The proxy auto-configuration script is in error
pub const E_BAD_AUTO_PROXY_SCRIPT : HResultError = HResultError::from_constant(0x80072F86); // WININET_E_BAD_AUTO_PROXY_SCRIPT

/// Could not download the proxy auto-configuration script file
pub const E_UNABLE_TO_DOWNLOAD_SCRIPT : HResultError = HResultError::from_constant(0x80072F87); // WININET_E_UNABLE_TO_DOWNLOAD_SCRIPT

/// The supplied certificate is invalid
pub const E_SEC_INVALID_CERT : HResultError = HResultError::from_constant(0x80072F89); // WININET_E_SEC_INVALID_CERT

/// The supplied certificate has been revoked
pub const E_SEC_CERT_REVOKED : HResultError = HResultError::from_constant(0x80072F8A); // WININET_E_SEC_CERT_REVOKED

/// The Dialup failed because file sharing was turned on and a failure was requested if security check was needed
pub const E_FAILED_DUETOSECURITYCHECK : HResultError = HResultError::from_constant(0x80072F8B); // WININET_E_FAILED_DUETOSECURITYCHECK

/// Initialization of the WinINet API has not occurred
pub const E_NOT_INITIALIZED : HResultError = HResultError::from_constant(0x80072F8C); // WININET_E_NOT_INITIALIZED

/// Login failed and the client should display the entity body to the user
pub const E_LOGIN_FAILURE_DISPLAY_ENTITY_BODY : HResultError = HResultError::from_constant(0x80072F8E); // WININET_E_LOGIN_FAILURE_DISPLAY_ENTITY_BODY

/// Content decoding has failed
pub const E_DECODING_FAILED : HResultError = HResultError::from_constant(0x80072F8F); // WININET_E_DECODING_FAILED

/// The HTTP request was not redirected
pub const E_NOT_REDIRECTED : HResultError = HResultError::from_constant(0x80072F80); // WININET_E_NOT_REDIRECTED

/// A cookie from the server must be confirmed by the user
pub const E_COOKIE_NEEDS_CONFIRMATION : HResultError = HResultError::from_constant(0x80072F81); // WININET_E_COOKIE_NEEDS_CONFIRMATION

/// A cookie from the server has been declined acceptance
pub const E_COOKIE_DECLINED : HResultError = HResultError::from_constant(0x80072F82); // WININET_E_COOKIE_DECLINED

/// The HTTP redirect request must be confirmed by the user
pub const E_REDIRECT_NEEDS_CONFIRMATION : HResultError = HResultError::from_constant(0x80072F88); // WININET_E_REDIRECT_NEEDS_CONFIRMATION
