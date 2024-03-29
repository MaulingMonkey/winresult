// WARNING: this file is auto-generated by xtask gen and may be overwritten

use super::*;


/// The virtual hard disk is corrupted. The virtual hard disk drive footer is missing.
pub const DRIVE_FOOTER_MISSING : HResultError = HResultError::from_constant(0xC03A0001); // ERROR_VHD_DRIVE_FOOTER_MISSING

/// The virtual hard disk is corrupted. The virtual hard disk drive footer checksum does not match the on-disk checksum.
pub const DRIVE_FOOTER_CHECKSUM_MISMATCH : HResultError = HResultError::from_constant(0xC03A0002); // ERROR_VHD_DRIVE_FOOTER_CHECKSUM_MISMATCH

/// The virtual hard disk is corrupted. The virtual hard disk drive footer in the virtual hard disk is corrupted.
pub const DRIVE_FOOTER_CORRUPT : HResultError = HResultError::from_constant(0xC03A0003); // ERROR_VHD_DRIVE_FOOTER_CORRUPT

/// The system does not recognize the file format of this virtual hard disk.
pub const FORMAT_UNKNOWN : HResultError = HResultError::from_constant(0xC03A0004); // ERROR_VHD_FORMAT_UNKNOWN

/// The version does not support this version of the file format.
pub const FORMAT_UNSUPPORTED_VERSION : HResultError = HResultError::from_constant(0xC03A0005); // ERROR_VHD_FORMAT_UNSUPPORTED_VERSION

/// The virtual hard disk is corrupted. The sparse header checksum does not match the on-disk checksum.
pub const SPARSE_HEADER_CHECKSUM_MISMATCH : HResultError = HResultError::from_constant(0xC03A0006); // ERROR_VHD_SPARSE_HEADER_CHECKSUM_MISMATCH

/// The system does not support this version of the virtual hard disk.This version of the sparse header is not supported.
pub const SPARSE_HEADER_UNSUPPORTED_VERSION : HResultError = HResultError::from_constant(0xC03A0007); // ERROR_VHD_SPARSE_HEADER_UNSUPPORTED_VERSION

/// The virtual hard disk is corrupted. The sparse header in the virtual hard disk is corrupt.
pub const SPARSE_HEADER_CORRUPT : HResultError = HResultError::from_constant(0xC03A0008); // ERROR_VHD_SPARSE_HEADER_CORRUPT

/// Failed to write to the virtual hard disk failed because the system failed to allocate a new block in the virtual hard disk.
pub const BLOCK_ALLOCATION_FAILURE : HResultError = HResultError::from_constant(0xC03A0009); // ERROR_VHD_BLOCK_ALLOCATION_FAILURE

/// The virtual hard disk is corrupted. The block allocation table in the virtual hard disk is corrupt.
pub const BLOCK_ALLOCATION_TABLE_CORRUPT : HResultError = HResultError::from_constant(0xC03A000A); // ERROR_VHD_BLOCK_ALLOCATION_TABLE_CORRUPT

/// The system does not support this version of the virtual hard disk. The block size is invalid.
pub const INVALID_BLOCK_SIZE : HResultError = HResultError::from_constant(0xC03A000B); // ERROR_VHD_INVALID_BLOCK_SIZE

/// The virtual hard disk is corrupted. The block bitmap does not match with the block data present in the virtual hard disk.
pub const BITMAP_MISMATCH : HResultError = HResultError::from_constant(0xC03A000C); // ERROR_VHD_BITMAP_MISMATCH

/// The chain of virtual hard disks is broken. The system cannot locate the parent virtual hard disk for the differencing disk.
pub const PARENT_VHD_NOT_FOUND : HResultError = HResultError::from_constant(0xC03A000D); // ERROR_VHD_PARENT_VHD_NOT_FOUND

/// The chain of virtual hard disks is corrupted. There is a mismatch in the identifiers of the parent virtual hard disk and differencing disk.
pub const CHILD_PARENT_ID_MISMATCH : HResultError = HResultError::from_constant(0xC03A000E); // ERROR_VHD_CHILD_PARENT_ID_MISMATCH

/// The chain of virtual hard disks is corrupted. The time stamp of the parent virtual hard disk does not match the time stamp of the differencing disk.
pub const CHILD_PARENT_TIMESTAMP_MISMATCH : HResultError = HResultError::from_constant(0xC03A000F); // ERROR_VHD_CHILD_PARENT_TIMESTAMP_MISMATCH

/// Failed to read the metadata of the virtual hard disk.
pub const METADATA_READ_FAILURE : HResultError = HResultError::from_constant(0xC03A0010); // ERROR_VHD_METADATA_READ_FAILURE

/// Failed to write to the metadata of the virtual hard disk.
pub const METADATA_WRITE_FAILURE : HResultError = HResultError::from_constant(0xC03A0011); // ERROR_VHD_METADATA_WRITE_FAILURE

/// The size of the virtual hard disk is not valid.
pub const INVALID_SIZE : HResultError = HResultError::from_constant(0xC03A0012); // ERROR_VHD_INVALID_SIZE

/// The file size of this virtual hard disk is not valid.
pub const INVALID_FILE_SIZE : HResultError = HResultError::from_constant(0xC03A0013); // ERROR_VHD_INVALID_FILE_SIZE

/// The chain of virtual hard disks is inaccessible. The process has not been granted access rights to the parent virtual hard disk for the differencing disk.
pub const PARENT_VHD_ACCESS_DENIED : HResultError = HResultError::from_constant(0xC03A0016); // ERROR_VHD_PARENT_VHD_ACCESS_DENIED

/// The chain of virtual hard disks is corrupted. There is a mismatch in the virtual sizes of the parent virtual hard disk and differencing disk.
pub const CHILD_PARENT_SIZE_MISMATCH : HResultError = HResultError::from_constant(0xC03A0017); // ERROR_VHD_CHILD_PARENT_SIZE_MISMATCH

/// The chain of virtual hard disks is corrupted. A differencing disk is indicated in its own parent chain.
pub const DIFFERENCING_CHAIN_CYCLE_DETECTED : HResultError = HResultError::from_constant(0xC03A0018); // ERROR_VHD_DIFFERENCING_CHAIN_CYCLE_DETECTED

/// The chain of virtual hard disks is inaccessible. There was an error opening a virtual hard disk further up the chain.
pub const DIFFERENCING_CHAIN_ERROR_IN_PARENT : HResultError = HResultError::from_constant(0xC03A0019); // ERROR_VHD_DIFFERENCING_CHAIN_ERROR_IN_PARENT

/// The requested operation cannot be performed on a virtual disk of this type.
pub const INVALID_TYPE : HResultError = HResultError::from_constant(0xC03A001B); // ERROR_VHD_INVALID_TYPE

/// The requested operation cannot be performed on the virtual disk in its current state.
pub const INVALID_STATE : HResultError = HResultError::from_constant(0xC03A001C); // ERROR_VHD_INVALID_STATE

/// The requested resize operation could not be completed because it might truncate user data residing on the virtual disk.
pub const RESIZE_WOULD_TRUNCATE_DATA : HResultError = HResultError::from_constant(0xC03A0025); // ERROR_VHD_RESIZE_WOULD_TRUNCATE_DATA

/// The requested operation could not be completed because the virtual disk's minimum safe size could not be determined.
/// This may be due to a missing or corrupt partition table.
pub const COULD_NOT_COMPUTE_MINIMUM_VIRTUAL_SIZE : HResultError = HResultError::from_constant(0xC03A0026); // ERROR_VHD_COULD_NOT_COMPUTE_MINIMUM_VIRTUAL_SIZE

/// The requested operation could not be completed because the virtual disk's size cannot be safely reduced further.
pub const ALREADY_AT_OR_BELOW_MINIMUM_VIRTUAL_SIZE : HResultError = HResultError::from_constant(0xC03A0027); // ERROR_VHD_ALREADY_AT_OR_BELOW_MINIMUM_VIRTUAL_SIZE

/// There is not enough space in the virtual disk file for the provided metadata item.
pub const METADATA_FULL : HResultError = HResultError::from_constant(0xC03A0028); // ERROR_VHD_METADATA_FULL

/// The specified change tracking identifier is not valid.
pub const INVALID_CHANGE_TRACKING_ID : HResultError = HResultError::from_constant(0xC03A0029); // ERROR_VHD_INVALID_CHANGE_TRACKING_ID

/// Change tracking is disabled for the specified virtual hard disk, so no change tracking information is available.
pub const CHANGE_TRACKING_DISABLED : HResultError = HResultError::from_constant(0xC03A002A); // ERROR_VHD_CHANGE_TRACKING_DISABLED

/// There is no change tracking data available associated with the specified change tracking identifier.
pub const MISSING_CHANGE_TRACKING_INFORMATION : HResultError = HResultError::from_constant(0xC03A0030); // ERROR_VHD_MISSING_CHANGE_TRACKING_INFORMATION

/// The requested operation cannot be performed on the virtual disk as it is currently used in shared mode.
pub const SHARED : HResultError = HResultError::from_constant(0xC05CFF0A); // ERROR_VHD_SHARED
