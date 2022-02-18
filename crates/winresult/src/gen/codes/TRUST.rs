// WARNING: this file is auto-generated by xtask gen and may be overwritten

use super::*;


/// A system-level error occurred while verifying trust.
pub const E_SYSTEM_ERROR : ErrorHResult = ErrorHResult::from_constant(0x80096001); // TRUST_E_SYSTEM_ERROR

/// The certificate for the signer of the message is invalid or not found.
pub const E_NO_SIGNER_CERT : ErrorHResult = ErrorHResult::from_constant(0x80096002); // TRUST_E_NO_SIGNER_CERT

/// One of the counter signatures was invalid.
pub const E_COUNTER_SIGNER : ErrorHResult = ErrorHResult::from_constant(0x80096003); // TRUST_E_COUNTER_SIGNER

/// The signature of the certificate cannot be verified.
pub const E_CERT_SIGNATURE : ErrorHResult = ErrorHResult::from_constant(0x80096004); // TRUST_E_CERT_SIGNATURE

/// The timestamp signature and/or certificate could not be verified or is malformed.
pub const E_TIME_STAMP : ErrorHResult = ErrorHResult::from_constant(0x80096005); // TRUST_E_TIME_STAMP

/// The digital signature of the object did not verify.
pub const E_BAD_DIGEST : ErrorHResult = ErrorHResult::from_constant(0x80096010); // TRUST_E_BAD_DIGEST

/// The digital signature of the object is malformed. For technical detail, see security bulletin MS13-098.
pub const E_MALFORMED_SIGNATURE : ErrorHResult = ErrorHResult::from_constant(0x80096011); // TRUST_E_MALFORMED_SIGNATURE

/// A certificate's basic constraint extension has not been observed.
pub const E_BASIC_CONSTRAINTS : ErrorHResult = ErrorHResult::from_constant(0x80096019); // TRUST_E_BASIC_CONSTRAINTS

/// The certificate does not meet or contain the Authenticode(tm) financial extensions.
pub const E_FINANCIAL_CRITERIA : ErrorHResult = ErrorHResult::from_constant(0x8009601E); // TRUST_E_FINANCIAL_CRITERIA

/// Unknown trust provider.
pub const E_PROVIDER_UNKNOWN : ErrorHResult = ErrorHResult::from_constant(0x800B0001); // TRUST_E_PROVIDER_UNKNOWN

/// The trust verification action specified is not supported by the specified trust provider.
pub const E_ACTION_UNKNOWN : ErrorHResult = ErrorHResult::from_constant(0x800B0002); // TRUST_E_ACTION_UNKNOWN

/// The form specified for the subject is not one supported or known by the specified trust provider.
pub const E_SUBJECT_FORM_UNKNOWN : ErrorHResult = ErrorHResult::from_constant(0x800B0003); // TRUST_E_SUBJECT_FORM_UNKNOWN

/// The subject is not trusted for the specified action.
pub const E_SUBJECT_NOT_TRUSTED : ErrorHResult = ErrorHResult::from_constant(0x800B0004); // TRUST_E_SUBJECT_NOT_TRUSTED

/// No signature was present in the subject.
pub const E_NOSIGNATURE : ErrorHResult = ErrorHResult::from_constant(0x800B0100); // TRUST_E_NOSIGNATURE

/// Generic trust failure.
pub const E_FAIL : ErrorHResult = ErrorHResult::from_constant(0x800B010B); // TRUST_E_FAIL

/// The certificate was explicitly marked as untrusted by the user.
pub const E_EXPLICIT_DISTRUST : ErrorHResult = ErrorHResult::from_constant(0x800B0111); // TRUST_E_EXPLICIT_DISTRUST