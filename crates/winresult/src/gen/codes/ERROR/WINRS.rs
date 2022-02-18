// WARNING: this file is auto-generated by xtask gen and may be overwritten

use super::*;


/// The WinRS client cannot process the request. An invalid flag was specified for this request.
/// Remove or change the invalid flag and try the request again.
pub const CLIENT_INVALID_FLAG : ErrorHResult = ErrorHResult::from_constant(0x80338120); // ERROR_WINRS_CLIENT_INVALID_FLAG

/// The WinRS client cannot process the request. One of the parameters required is null or zero.
/// Change the request to include the missing parameter and try again.
pub const CLIENT_NULL_PARAM : ErrorHResult = ErrorHResult::from_constant(0x80338121); // ERROR_WINRS_CLIENT_NULL_PARAM

/// Not used. To be removed.
pub const RECEIVE_IN_PROGRESS : ErrorHResult = ErrorHResult::from_constant(0x80338127); // ERROR_WINRS_RECEIVE_IN_PROGRESS

/// The WinRS client cannot process the Receive request because the shell plugin returned an empty response to the request.
pub const RECEIVE_NO_RESPONSE_DATA : ErrorHResult = ErrorHResult::from_constant(0x80338128); // ERROR_WINRS_RECEIVE_NO_RESPONSE_DATA

/// The WinRM Shell client cannot process the request. One of the parameters required for the WSManCreateShell function is null or zero.
/// Change the request to include the missing parameter and try again.
pub const CLIENT_CREATESHELL_NULL_PARAM : ErrorHResult = ErrorHResult::from_constant(0x80338129); // ERROR_WINRS_CLIENT_CREATESHELL_NULL_PARAM

/// The WinRM Shell client cannot process the request. One of the parameters required for the WinrsCloseShell function is null or zero.
/// Change the request to include the missing parameter and try again.
pub const CLIENT_CLOSESHELL_NULL_PARAM : ErrorHResult = ErrorHResult::from_constant(0x8033812A); // ERROR_WINRS_CLIENT_CLOSESHELL_NULL_PARAM

/// The WinRS client cannot process the request. The parameter required for the WinrsFreeCreateShellResult function is null or zero.
/// Change the request to include the missing parameter and try again.
pub const CLIENT_FREECREATESHELLRESULT_NULL_PARAM : ErrorHResult = ErrorHResult::from_constant(0x8033812B); // ERROR_WINRS_CLIENT_FREECREATESHELLRESULT_NULL_PARAM

/// The WinRM Shell client cannot process the request. One of the parameters required for the WSManRunShellCommand function is null or zero.
/// Change the request to include the missing parameter and try again.
pub const CLIENT_RUNCOMMAND_NULL_PARAM : ErrorHResult = ErrorHResult::from_constant(0x8033812C); // ERROR_WINRS_CLIENT_RUNCOMMAND_NULL_PARAM

/// The WinRS client cannot process the request. The parameter required for the WinrsFreeRunCommandResult function is null or zero.
/// Change the request to include the missing parameter and try again.
pub const CLIENT_FREERUNCOMMANDRESULT_NULL_PARAM : ErrorHResult = ErrorHResult::from_constant(0x8033812D); // ERROR_WINRS_CLIENT_FREERUNCOMMANDRESULT_NULL_PARAM

/// The WinRM Shell client cannot process the request. One of the parameters required for the WSManSignalShell function is null or zero.
/// Change the request to include the missing parameter and try again.
pub const CLIENT_SIGNAL_NULL_PARAM : ErrorHResult = ErrorHResult::from_constant(0x8033812E); // ERROR_WINRS_CLIENT_SIGNAL_NULL_PARAM

/// The WinRM Shell client cannot process the request. One of the parameters required for the WSMansReceiveShellOutput function is null or zero.
/// Change the request to include the missing parameter and try again.
pub const CLIENT_RECEIVE_NULL_PARAM : ErrorHResult = ErrorHResult::from_constant(0x8033812F); // ERROR_WINRS_CLIENT_RECEIVE_NULL_PARAM

/// The WinRS client cannot process the request. The parameter required for the WinrsFreePullResult function is null or zero.
/// Change the request to include the missing parameter and try again.
pub const CLIENT_FREEPULLRESULT_NULL_PARAM : ErrorHResult = ErrorHResult::from_constant(0x80338130); // ERROR_WINRS_CLIENT_FREEPULLRESULT_NULL_PARAM

/// The WinRS client cannot process the request. One of the parameters required for the WinrsPull function is null or zero.
/// Change the request to include the missing parameter and try again.
pub const CLIENT_PULL_NULL_PARAM : ErrorHResult = ErrorHResult::from_constant(0x80338131); // ERROR_WINRS_CLIENT_PULL_NULL_PARAM

/// The WinRS client cannot process the request. The parameter required for the WinrsCloseReceiveHandle function is null or zero.
/// Change the request to include the missing parameter and try again.
pub const CLIENT_CLOSERECEIVEHANDLE_NULL_PARAM : ErrorHResult = ErrorHResult::from_constant(0x80338132); // ERROR_WINRS_CLIENT_CLOSERECEIVEHANDLE_NULL_PARAM

/// The WinRM Shell client cannot process the request. One of the parameters required for the WSManSendShellInput function is null or zero.
/// Change the request to include the missing parameter and try again.
pub const CLIENT_SEND_NULL_PARAM : ErrorHResult = ErrorHResult::from_constant(0x80338133); // ERROR_WINRS_CLIENT_SEND_NULL_PARAM

/// The WinRS client cannot process the request. One of the parameters required for the WinrsPush function is null or zero.
/// Change the request to include the missing parameter and try again.
pub const CLIENT_PUSH_NULL_PARAM : ErrorHResult = ErrorHResult::from_constant(0x80338134); // ERROR_WINRS_CLIENT_PUSH_NULL_PARAM

/// The WinRS client cannot process the request. The parameter required for the WinrsCloseSendHandle function is null or zero.
/// Change the request to include the missing parameter and try again.
pub const CLIENT_CLOSESENDHANDLE_NULL_PARAM : ErrorHResult = ErrorHResult::from_constant(0x80338135); // ERROR_WINRS_CLIENT_CLOSESENDHANDLE_NULL_PARAM

/// The WinRS client cannot process the request. One of the parameters required for the WinrsGet function is null or zero.
/// Change the request to include the missing parameter and try again.
pub const CLIENT_GET_NULL_PARAM : ErrorHResult = ErrorHResult::from_constant(0x80338136); // ERROR_WINRS_CLIENT_GET_NULL_PARAM

/// The WinRS client cannot process the request. The server cannot set Code Page.
/// You may want to use the CHCP command to change the client Code Page to 437 and receive the results in English.
pub const CODE_PAGE_NOT_SUPPORTED : ErrorHResult = ErrorHResult::from_constant(0x80338140); // ERROR_WINRS_CODE_PAGE_NOT_SUPPORTED

/// Not used. To be removed.
pub const SHELL_URI_INVALID : ErrorHResult = ErrorHResult::from_constant(0x8033815B); // ERROR_WINRS_SHELL_URI_INVALID

/// The WinRM service cannot process the request because the WinRS shell instance is currently disconnected.
pub const SHELL_DISCONNECTED : ErrorHResult = ErrorHResult::from_constant(0x803381C4); // ERROR_WINRS_SHELL_DISCONNECTED

/// The WinRM service cannot process the request. This WinRS shell instance does not support disconnect and reconnect operations
/// because it was created by an older WinRS client or its provider does not support the disconnect operation.
pub const SHELL_DISCONNECT_NOT_SUPPORTED : ErrorHResult = ErrorHResult::from_constant(0x803381C5); // ERROR_WINRS_SHELL_DISCONNECT_NOT_SUPPORTED

/// The WinRM service cannot process the request because the WinRS shell instance is connected to a different client.
pub const SHELL_CLIENTSESSIONID_MISMATCH : ErrorHResult = ErrorHResult::from_constant(0x803381C6); // ERROR_WINRS_SHELL_CLIENTSESSIONID_MISMATCH

/// The WinRM client cannot process the request. The body response is not a valid connect request response.
pub const CONNECT_RESPONSE_BAD_BODY : ErrorHResult = ErrorHResult::from_constant(0x803381CB); // ERROR_WINRS_CONNECT_RESPONSE_BAD_BODY

/// The WinRM service cannot process the request. The WinRS shell instance is currently connected to a different client.
pub const SHELL_CONNECTED_TO_DIFFERENT_CLIENT : ErrorHResult = ErrorHResult::from_constant(0x803381CD); // ERROR_WINRS_SHELL_CONNECTED_TO_DIFFERENT_CLIENT

/// The WinRM client encountered an error while communicating with the WinRM service during the disconnect operation.
/// The shell has been disconnected and the streams were possibly suspended abruptly.
pub const SHELL_DISCONNECT_OPERATION_NOT_GRACEFUL : ErrorHResult = ErrorHResult::from_constant(0x803381CE); // ERROR_WINRS_SHELL_DISCONNECT_OPERATION_NOT_GRACEFUL

/// The WinRM client cannot process the request. A disconnect operation cannot be performed on a WinRS shell instance that is already disconnected.
pub const SHELL_DISCONNECT_OPERATION_NOT_VALID : ErrorHResult = ErrorHResult::from_constant(0x803381CF); // ERROR_WINRS_SHELL_DISCONNECT_OPERATION_NOT_VALID

/// The WinRM client cannot process the request. A reconnect operation cannot be performed on a WinRS shell instance that is currently connected.
pub const SHELL_RECONNECT_OPERATION_NOT_VALID : ErrorHResult = ErrorHResult::from_constant(0x803381D0); // ERROR_WINRS_SHELL_RECONNECT_OPERATION_NOT_VALID

/// The WinRM client cannot process the request. A reconnect operation cannot be performed on a WinRS shell command instance that is currently connected.
pub const SHELLCOMMAND_RECONNECT_OPERATION_NOT_VALID : ErrorHResult = ErrorHResult::from_constant(0x803381D3); // ERROR_WINRS_SHELLCOMMAND_RECONNECT_OPERATION_NOT_VALID

/// The WinRM service cannot process the request because the command ID specified by the client is not a valid GUID. Modify the request and retry the request.
pub const SHELLCOMMAND_CLIENTID_NOT_VALID : ErrorHResult = ErrorHResult::from_constant(0x803381D4); // ERROR_WINRS_SHELLCOMMAND_CLIENTID_NOT_VALID

/// The WinRM service cannot process the request because the shell ID specified by the client is not a valid GUID. Provide a valid ID and try again.
pub const SHELL_CLIENTID_NOT_VALID : ErrorHResult = ErrorHResult::from_constant(0x803381D5); // ERROR_WINRS_SHELL_CLIENTID_NOT_VALID

/// The WinRM service cannot process the request. A command already exists with the command ID specified by the client.
pub const SHELLCOMMAND_CLIENTID_RESOURCE_CONFLICT : ErrorHResult = ErrorHResult::from_constant(0x803381D6); // ERROR_WINRS_SHELLCOMMAND_CLIENTID_RESOURCE_CONFLICT

/// The WinRM service cannot process the request. A resource already exists with the shell ID specified by the client.
pub const SHELL_CLIENTID_RESOURCE_CONFLICT : ErrorHResult = ErrorHResult::from_constant(0x803381D7); // ERROR_WINRS_SHELL_CLIENTID_RESOURCE_CONFLICT

/// The WinRM client cannot process the request. A disconnect operation cannot be performed on a WinRS shell command instance that is disconnected.
pub const SHELLCOMMAND_DISCONNECT_OPERATION_NOT_VALID : ErrorHResult = ErrorHResult::from_constant(0x803381D8); // ERROR_WINRS_SHELLCOMMAND_DISCONNECT_OPERATION_NOT_VALID

/// The WS-Management service cannot process the request. The requested IdleTimeout is outside the allowed range.
pub const IDLETIMEOUT_OUTOFBOUNDS : ErrorHResult = ErrorHResult::from_constant(0x803381F2); // ERROR_WINRS_IDLETIMEOUT_OUTOFBOUNDS