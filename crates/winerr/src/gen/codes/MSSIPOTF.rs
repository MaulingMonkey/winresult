// WARNING: this file is auto-generated by xtask gen and may be overwritten

use super::*;


/// Tried to reference a part of the file outside the proper range.
pub const E_OUTOFMEMRANGE : HRESULT = HRESULT::from_constant(0x80097001); // MSSIPOTF_E_OUTOFMEMRANGE

/// Could not retrieve an object from the file.
pub const E_CANTGETOBJECT : HRESULT = HRESULT::from_constant(0x80097002); // MSSIPOTF_E_CANTGETOBJECT

/// Could not find the head table in the file.
pub const E_NOHEADTABLE : HRESULT = HRESULT::from_constant(0x80097003); // MSSIPOTF_E_NOHEADTABLE

/// The magic number in the head table is incorrect.
pub const E_BAD_MAGICNUMBER : HRESULT = HRESULT::from_constant(0x80097004); // MSSIPOTF_E_BAD_MAGICNUMBER

/// The offset table has incorrect values.
pub const E_BAD_OFFSET_TABLE : HRESULT = HRESULT::from_constant(0x80097005); // MSSIPOTF_E_BAD_OFFSET_TABLE

/// Duplicate table tags or tags out of alphabetical order.
pub const E_TABLE_TAGORDER : HRESULT = HRESULT::from_constant(0x80097006); // MSSIPOTF_E_TABLE_TAGORDER

/// A table does not start on a long word boundary.
pub const E_TABLE_LONGWORD : HRESULT = HRESULT::from_constant(0x80097007); // MSSIPOTF_E_TABLE_LONGWORD

/// First table does not appear after header information.
pub const E_BAD_FIRST_TABLE_PLACEMENT : HRESULT = HRESULT::from_constant(0x80097008); // MSSIPOTF_E_BAD_FIRST_TABLE_PLACEMENT

/// Two or more tables overlap.
pub const E_TABLES_OVERLAP : HRESULT = HRESULT::from_constant(0x80097009); // MSSIPOTF_E_TABLES_OVERLAP

/// Too many pad bytes between tables or pad bytes are not 0.
pub const E_TABLE_PADBYTES : HRESULT = HRESULT::from_constant(0x8009700A); // MSSIPOTF_E_TABLE_PADBYTES

/// File is too small to contain the last table.
pub const E_FILETOOSMALL : HRESULT = HRESULT::from_constant(0x8009700B); // MSSIPOTF_E_FILETOOSMALL

/// A table checksum is incorrect.
pub const E_TABLE_CHECKSUM : HRESULT = HRESULT::from_constant(0x8009700C); // MSSIPOTF_E_TABLE_CHECKSUM

/// The file checksum is incorrect.
pub const E_FILE_CHECKSUM : HRESULT = HRESULT::from_constant(0x8009700D); // MSSIPOTF_E_FILE_CHECKSUM

/// The signature does not have the correct attributes for the policy.
pub const E_FAILED_POLICY : HRESULT = HRESULT::from_constant(0x80097010); // MSSIPOTF_E_FAILED_POLICY

/// The file did not pass the hints check.
pub const E_FAILED_HINTS_CHECK : HRESULT = HRESULT::from_constant(0x80097011); // MSSIPOTF_E_FAILED_HINTS_CHECK

/// The file is not an OpenType file.
pub const E_NOT_OPENTYPE : HRESULT = HRESULT::from_constant(0x80097012); // MSSIPOTF_E_NOT_OPENTYPE

/// Failed on a file operation (open, map, read, write).
pub const E_FILE : HRESULT = HRESULT::from_constant(0x80097013); // MSSIPOTF_E_FILE

/// A call to a CryptoAPI function failed.
pub const E_CRYPT : HRESULT = HRESULT::from_constant(0x80097014); // MSSIPOTF_E_CRYPT

/// There is a bad version number in the file.
pub const E_BADVERSION : HRESULT = HRESULT::from_constant(0x80097015); // MSSIPOTF_E_BADVERSION

/// The structure of the DSIG table is incorrect.
pub const E_DSIG_STRUCTURE : HRESULT = HRESULT::from_constant(0x80097016); // MSSIPOTF_E_DSIG_STRUCTURE

/// A check failed in a partially constant table.
pub const E_PCONST_CHECK : HRESULT = HRESULT::from_constant(0x80097017); // MSSIPOTF_E_PCONST_CHECK

/// Some kind of structural error.
pub const E_STRUCTURE : HRESULT = HRESULT::from_constant(0x80097018); // MSSIPOTF_E_STRUCTURE
