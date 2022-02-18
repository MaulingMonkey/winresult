// WARNING: this file is auto-generated by xtask gen and may be overwritten

use super::*;


/// Secure Boot detected that rollback of protected data has been attempted.
pub const ROLLBACK_DETECTED : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(4420); // ERROR_SECUREBOOT_ROLLBACK_DETECTED

/// The value is protected by Secure Boot policy and cannot be modified or deleted.
pub const POLICY_VIOLATION : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(4421); // ERROR_SECUREBOOT_POLICY_VIOLATION

/// The Secure Boot policy is invalid.
pub const INVALID_POLICY : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(4422); // ERROR_SECUREBOOT_INVALID_POLICY

/// A new Secure Boot policy did not contain the current publisher on its update list.
pub const POLICY_PUBLISHER_NOT_FOUND : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(4423); // ERROR_SECUREBOOT_POLICY_PUBLISHER_NOT_FOUND

/// The Secure Boot policy is either not signed or is signed by a non-trusted signer.
pub const POLICY_NOT_SIGNED : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(4424); // ERROR_SECUREBOOT_POLICY_NOT_SIGNED

/// Secure Boot is not enabled on this machine.
pub const NOT_ENABLED : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(4425); // ERROR_SECUREBOOT_NOT_ENABLED

/// Secure Boot requires that certain files and drivers are not replaced by other files or drivers.
pub const FILE_REPLACED : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(4426); // ERROR_SECUREBOOT_FILE_REPLACED

/// The Secure Boot Supplemental Policy file was not authorized on this machine.
pub const POLICY_NOT_AUTHORIZED : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(4427); // ERROR_SECUREBOOT_POLICY_NOT_AUTHORIZED

/// The Supplemental Policy is not recognized on this device.
pub const POLICY_UNKNOWN : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(4428); // ERROR_SECUREBOOT_POLICY_UNKNOWN

/// The Antirollback version was not found in the Secure Boot Policy.
pub const POLICY_MISSING_ANTIROLLBACKVERSION : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(4429); // ERROR_SECUREBOOT_POLICY_MISSING_ANTIROLLBACKVERSION

/// The Platform ID specified in the Secure Boot policy does not match the Platform ID on this device.
pub const PLATFORM_ID_MISMATCH : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(4430); // ERROR_SECUREBOOT_PLATFORM_ID_MISMATCH

/// The Secure Boot policy file has an older Antirollback Version than this device.
pub const POLICY_ROLLBACK_DETECTED : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(4431); // ERROR_SECUREBOOT_POLICY_ROLLBACK_DETECTED

/// The Secure Boot policy file does not match the upgraded legacy policy.
pub const POLICY_UPGRADE_MISMATCH : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(4432); // ERROR_SECUREBOOT_POLICY_UPGRADE_MISMATCH

/// The Secure Boot policy file is required but could not be found.
pub const REQUIRED_POLICY_FILE_MISSING : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(4433); // ERROR_SECUREBOOT_REQUIRED_POLICY_FILE_MISSING

/// Supplemental Secure Boot policy file can not be loaded as a base Secure Boot policy.
pub const NOT_BASE_POLICY : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(4434); // ERROR_SECUREBOOT_NOT_BASE_POLICY

/// Base Secure Boot policy file can not be loaded as a Supplemental Secure Boot policy.
pub const NOT_SUPPLEMENTAL_POLICY : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(4435); // ERROR_SECUREBOOT_NOT_SUPPLEMENTAL_POLICY