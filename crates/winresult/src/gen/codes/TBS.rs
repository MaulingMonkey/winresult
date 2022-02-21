// WARNING: this file is auto-generated by xtask gen and may be overwritten

use super::*;


/// An internal error has occurred within the Trusted Platform Module support program.
pub const E_INTERNAL_ERROR : HResultError = HResultError::from_constant(0x80284001); // TBS_E_INTERNAL_ERROR

/// One or more input parameters is bad.
pub const E_BAD_PARAMETER : HResultError = HResultError::from_constant(0x80284002); // TBS_E_BAD_PARAMETER

/// A specified output pointer is bad.
pub const E_INVALID_OUTPUT_POINTER : HResultError = HResultError::from_constant(0x80284003); // TBS_E_INVALID_OUTPUT_POINTER

/// The specified context handle does not refer to a valid context.
pub const E_INVALID_CONTEXT : HResultError = HResultError::from_constant(0x80284004); // TBS_E_INVALID_CONTEXT

/// A specified output buffer is too small.
pub const E_INSUFFICIENT_BUFFER : HResultError = HResultError::from_constant(0x80284005); // TBS_E_INSUFFICIENT_BUFFER

/// An error occurred while communicating with the TPM.
pub const E_IOERROR : HResultError = HResultError::from_constant(0x80284006); // TBS_E_IOERROR

/// One or more context parameters is invalid.
pub const E_INVALID_CONTEXT_PARAM : HResultError = HResultError::from_constant(0x80284007); // TBS_E_INVALID_CONTEXT_PARAM

/// The TBS service is not running and could not be started.
pub const E_SERVICE_NOT_RUNNING : HResultError = HResultError::from_constant(0x80284008); // TBS_E_SERVICE_NOT_RUNNING

/// A new context could not be created because there are too many open contexts.
pub const E_TOO_MANY_TBS_CONTEXTS : HResultError = HResultError::from_constant(0x80284009); // TBS_E_TOO_MANY_TBS_CONTEXTS

/// A new virtual resource could not be created because there are too many open virtual resources.
pub const E_TOO_MANY_RESOURCES : HResultError = HResultError::from_constant(0x8028400A); // TBS_E_TOO_MANY_RESOURCES

/// The TBS service has been started but is not yet running.
pub const E_SERVICE_START_PENDING : HResultError = HResultError::from_constant(0x8028400B); // TBS_E_SERVICE_START_PENDING

/// The physical presence interface is not supported.
pub const E_PPI_NOT_SUPPORTED : HResultError = HResultError::from_constant(0x8028400C); // TBS_E_PPI_NOT_SUPPORTED

/// The command was canceled.
pub const E_COMMAND_CANCELED : HResultError = HResultError::from_constant(0x8028400D); // TBS_E_COMMAND_CANCELED

/// The input or output buffer is too large.
pub const E_BUFFER_TOO_LARGE : HResultError = HResultError::from_constant(0x8028400E); // TBS_E_BUFFER_TOO_LARGE

/// A compatible Trusted Platform Module (TPM) Security Device cannot be found on this computer.
pub const E_TPM_NOT_FOUND : HResultError = HResultError::from_constant(0x8028400F); // TBS_E_TPM_NOT_FOUND

/// The TBS service has been disabled.
pub const E_SERVICE_DISABLED : HResultError = HResultError::from_constant(0x80284010); // TBS_E_SERVICE_DISABLED

/// No TCG event log is available.
pub const E_NO_EVENT_LOG : HResultError = HResultError::from_constant(0x80284011); // TBS_E_NO_EVENT_LOG

/// The caller does not have the appropriate rights to perform the requested operation.
pub const E_ACCESS_DENIED : HResultError = HResultError::from_constant(0x80284012); // TBS_E_ACCESS_DENIED

/// The TPM provisioning action is not allowed by the specified flags.  For provisioning to be successful, one of several actions may be required.  The TPM management console (tpm.msc) action to make the TPM Ready may help.  For further information, see the documentation for the Win32_Tpm WMI method 'Provision'.  (The actions that may be required include importing the TPM Owner Authorization value into the system, calling the Win32_Tpm WMI method for provisioning the TPM and specifying TRUE for either 'ForceClear_Allowed' or 'PhysicalPresencePrompts_Allowed' (as indicated by the value returned in the Additional Information), or enabling the TPM in the system BIOS.)
pub const E_PROVISIONING_NOT_ALLOWED : HResultError = HResultError::from_constant(0x80284013); // TBS_E_PROVISIONING_NOT_ALLOWED

/// The Physical Presence Interface of this firmware does not support the requested method.
pub const E_PPI_FUNCTION_UNSUPPORTED : HResultError = HResultError::from_constant(0x80284014); // TBS_E_PPI_FUNCTION_UNSUPPORTED

/// The requested TPM OwnerAuth value was not found.
pub const E_OWNERAUTH_NOT_FOUND : HResultError = HResultError::from_constant(0x80284015); // TBS_E_OWNERAUTH_NOT_FOUND

/// The TPM provisioning did not complete.  For more information on completing the provisioning, call the Win32_Tpm WMI method for provisioning the TPM ('Provision') and check the returned Information.
pub const E_PROVISIONING_INCOMPLETE : HResultError = HResultError::from_constant(0x80284016); // TBS_E_PROVISIONING_INCOMPLETE
