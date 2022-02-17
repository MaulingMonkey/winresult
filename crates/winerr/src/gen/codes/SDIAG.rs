// WARNING: this file is auto-generated by xtask gen and may be overwritten

use super::*;


/// The operation was cancelled.
pub const E_CANCELLED : ErrorHResult = ErrorHResult::from_constant(0x803C0100); // SDIAG_E_CANCELLED

/// An error occurred when running a PowerShell script.
pub const E_SCRIPT : ErrorHResult = ErrorHResult::from_constant(0x803C0101); // SDIAG_E_SCRIPT

/// An error occurred when interacting with PowerShell runtime.
pub const E_POWERSHELL : ErrorHResult = ErrorHResult::from_constant(0x803C0102); // SDIAG_E_POWERSHELL

/// An error occurred in the Scripted Diagnostic Managed Host.
pub const E_MANAGEDHOST : ErrorHResult = ErrorHResult::from_constant(0x803C0103); // SDIAG_E_MANAGEDHOST

/// The troubleshooting pack does not contain a required verifier to complete the verification.
pub const E_NOVERIFIER : ErrorHResult = ErrorHResult::from_constant(0x803C0104); // SDIAG_E_NOVERIFIER

/// The troubleshooting pack cannot be executed on this system.
pub const S_CANNOTRUN : SuccessHResult = SuccessHResult::from_constant(0x003C0105); // SDIAG_S_CANNOTRUN

/// Scripted diagnostics is disabled by group policy.
pub const E_DISABLED : ErrorHResult = ErrorHResult::from_constant(0x803C0106); // SDIAG_E_DISABLED

/// Trust validation of the troubleshooting pack failed.
pub const E_TRUST : ErrorHResult = ErrorHResult::from_constant(0x803C0107); // SDIAG_E_TRUST

/// The troubleshooting pack cannot be executed on this system.
pub const E_CANNOTRUN : ErrorHResult = ErrorHResult::from_constant(0x803C0108); // SDIAG_E_CANNOTRUN

/// This version of the troubleshooting pack is not supported.
pub const E_VERSION : ErrorHResult = ErrorHResult::from_constant(0x803C0109); // SDIAG_E_VERSION

/// A required resource cannot be loaded.
pub const E_RESOURCE : ErrorHResult = ErrorHResult::from_constant(0x803C010A); // SDIAG_E_RESOURCE

/// The troubleshooting pack reported information for a root cause without adding the root cause.
pub const E_ROOTCAUSE : ErrorHResult = ErrorHResult::from_constant(0x803C010B); // SDIAG_E_ROOTCAUSE
