// WARNING: this file is auto-generated by xtask gen and may be overwritten

use super::*;


/// Error due to problem in ASN.1 encoding process.
pub const E_ENCODE : HResultError = HResultError::from_constant(0x800B0005); // DIGSIG_E_ENCODE

/// Error due to problem in ASN.1 decoding process.
pub const E_DECODE : HResultError = HResultError::from_constant(0x800B0006); // DIGSIG_E_DECODE

/// Reading / writing Extensions where Attributes are appropriate, and vice versa.
pub const E_EXTENSIBILITY : HResultError = HResultError::from_constant(0x800B0007); // DIGSIG_E_EXTENSIBILITY

/// Unspecified cryptographic failure.
pub const E_CRYPTO : HResultError = HResultError::from_constant(0x800B0008); // DIGSIG_E_CRYPTO
