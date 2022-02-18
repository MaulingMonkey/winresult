// WARNING: this file is auto-generated by xtask gen and may be overwritten

use super::*;


/// No package in the software installation data in the Active Directory meets this criteria.
pub const E_PACKAGE_NOTFOUND : ErrorHResult = ErrorHResult::from_constant(0x80040164); // CS_E_PACKAGE_NOTFOUND

/// Deleting this will break the referential integrity of the software installation data in the Active Directory.
pub const E_NOT_DELETABLE : ErrorHResult = ErrorHResult::from_constant(0x80040165); // CS_E_NOT_DELETABLE

/// The CLSID was not found in the software installation data in the Active Directory.
pub const E_CLASS_NOTFOUND : ErrorHResult = ErrorHResult::from_constant(0x80040166); // CS_E_CLASS_NOTFOUND

/// The software installation data in the Active Directory is corrupt.
pub const E_INVALID_VERSION : ErrorHResult = ErrorHResult::from_constant(0x80040167); // CS_E_INVALID_VERSION

/// There is no software installation data in the Active Directory.
pub const E_NO_CLASSSTORE : ErrorHResult = ErrorHResult::from_constant(0x80040168); // CS_E_NO_CLASSSTORE

/// There is no software installation data object in the Active Directory.
pub const E_OBJECT_NOTFOUND : ErrorHResult = ErrorHResult::from_constant(0x80040169); // CS_E_OBJECT_NOTFOUND

/// The software installation data object in the Active Directory already exists.
pub const E_OBJECT_ALREADY_EXISTS : ErrorHResult = ErrorHResult::from_constant(0x8004016A); // CS_E_OBJECT_ALREADY_EXISTS

/// The path to the software installation data in the Active Directory is not correct.
pub const E_INVALID_PATH : ErrorHResult = ErrorHResult::from_constant(0x8004016B); // CS_E_INVALID_PATH

/// A network error interrupted the operation.
pub const E_NETWORK_ERROR : ErrorHResult = ErrorHResult::from_constant(0x8004016C); // CS_E_NETWORK_ERROR

/// The size of this object exceeds the maximum size set by the Administrator.
pub const E_ADMIN_LIMIT_EXCEEDED : ErrorHResult = ErrorHResult::from_constant(0x8004016D); // CS_E_ADMIN_LIMIT_EXCEEDED

/// The schema for the software installation data in the Active Directory does not match the required schema.
pub const E_SCHEMA_MISMATCH : ErrorHResult = ErrorHResult::from_constant(0x8004016E); // CS_E_SCHEMA_MISMATCH

/// An error occurred in the software installation data in the Active Directory.
pub const E_INTERNAL_ERROR : ErrorHResult = ErrorHResult::from_constant(0x8004016F); // CS_E_INTERNAL_ERROR