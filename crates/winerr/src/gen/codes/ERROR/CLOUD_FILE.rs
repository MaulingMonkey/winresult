// WARNING: this file is auto-generated by xtask gen and may be overwritten

use super::*;


/// The cloud sync root metadata is corrupted.
pub const SYNC_ROOT_METADATA_CORRUPT : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(358); // ERROR_CLOUD_FILE_SYNC_ROOT_METADATA_CORRUPT

/// The cloud file provider is not running.
pub const PROVIDER_NOT_RUNNING : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(362); // ERROR_CLOUD_FILE_PROVIDER_NOT_RUNNING

/// The cloud file metadata is corrupt and unreadable.
pub const METADATA_CORRUPT : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(363); // ERROR_CLOUD_FILE_METADATA_CORRUPT

/// The cloud file metadata is too large.
pub const METADATA_TOO_LARGE : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(364); // ERROR_CLOUD_FILE_METADATA_TOO_LARGE

/// The cloud file property is too large.
pub const PROPERTY_BLOB_TOO_LARGE : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(365); // ERROR_CLOUD_FILE_PROPERTY_BLOB_TOO_LARGE

/// The cloud file property is possibly corrupt. The on-disk checksum does not match the computed checksum.
pub const PROPERTY_BLOB_CHECKSUM_MISMATCH : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(366); // ERROR_CLOUD_FILE_PROPERTY_BLOB_CHECKSUM_MISMATCH

/// The maximum number of cloud file properties has been reached.
pub const TOO_MANY_PROPERTY_BLOBS : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(374); // ERROR_CLOUD_FILE_TOO_MANY_PROPERTY_BLOBS

/// The version of the cloud file property store is not supported.
pub const PROPERTY_VERSION_NOT_SUPPORTED : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(375); // ERROR_CLOUD_FILE_PROPERTY_VERSION_NOT_SUPPORTED

/// The file is not in sync with the cloud.
pub const NOT_IN_SYNC : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(377); // ERROR_CLOUD_FILE_NOT_IN_SYNC

/// The cloud sync root is already connected with another cloud sync provider.
pub const ALREADY_CONNECTED : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(378); // ERROR_CLOUD_FILE_ALREADY_CONNECTED

/// The operation is not supported by the cloud sync provider.
pub const NOT_SUPPORTED : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(379); // ERROR_CLOUD_FILE_NOT_SUPPORTED

/// The cloud operation is invalid.
pub const INVALID_REQUEST : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(380); // ERROR_CLOUD_FILE_INVALID_REQUEST

/// The cloud operation is not supported on a read-only volume.
pub const READ_ONLY_VOLUME : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(381); // ERROR_CLOUD_FILE_READ_ONLY_VOLUME

/// The operation is reserved for a connected cloud sync provider.
pub const CONNECTED_PROVIDER_ONLY : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(382); // ERROR_CLOUD_FILE_CONNECTED_PROVIDER_ONLY

/// The cloud sync provider failed to validate the downloaded data.
pub const VALIDATION_FAILED : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(383); // ERROR_CLOUD_FILE_VALIDATION_FAILED

/// The cloud sync provider failed user authentication.
pub const AUTHENTICATION_FAILED : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(386); // ERROR_CLOUD_FILE_AUTHENTICATION_FAILED

/// The cloud sync provider failed to perform the operation due to low system resources.
pub const INSUFFICIENT_RESOURCES : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(387); // ERROR_CLOUD_FILE_INSUFFICIENT_RESOURCES

/// The cloud sync provider failed to perform the operation due to network being unavailable.
pub const NETWORK_UNAVAILABLE : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(388); // ERROR_CLOUD_FILE_NETWORK_UNAVAILABLE

/// The cloud operation was unsuccessful.
pub const UNSUCCESSFUL : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(389); // ERROR_CLOUD_FILE_UNSUCCESSFUL

/// The operation is only supported on files under a cloud sync root.
pub const NOT_UNDER_SYNC_ROOT : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(390); // ERROR_CLOUD_FILE_NOT_UNDER_SYNC_ROOT

/// The operation cannot be performed on cloud files in use.
pub const IN_USE : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(391); // ERROR_CLOUD_FILE_IN_USE

/// The operation cannot be performed on pinned cloud files.
pub const PINNED : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(392); // ERROR_CLOUD_FILE_PINNED

/// The cloud operation was aborted.
pub const REQUEST_ABORTED : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(393); // ERROR_CLOUD_FILE_REQUEST_ABORTED

/// The cloud file's property store is corrupt.
pub const PROPERTY_CORRUPT : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(394); // ERROR_CLOUD_FILE_PROPERTY_CORRUPT

/// Access to the cloud file is denied.
pub const ACCESS_DENIED : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(395); // ERROR_CLOUD_FILE_ACCESS_DENIED

/// The cloud operation cannot be performed on a file with incompatible hardlinks.
pub const INCOMPATIBLE_HARDLINKS : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(396); // ERROR_CLOUD_FILE_INCOMPATIBLE_HARDLINKS

/// The operation failed due to a conflicting cloud file property lock.
pub const PROPERTY_LOCK_CONFLICT : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(397); // ERROR_CLOUD_FILE_PROPERTY_LOCK_CONFLICT

/// The cloud operation was canceled by user.
pub const REQUEST_CANCELED : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(398); // ERROR_CLOUD_FILE_REQUEST_CANCELED

/// The cloud file provider exited unexpectedly.
pub const PROVIDER_TERMINATED : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(404); // ERROR_CLOUD_FILE_PROVIDER_TERMINATED

/// The cloud operation was not completed before the time-out period expired.
pub const REQUEST_TIMEOUT : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(426); // ERROR_CLOUD_FILE_REQUEST_TIMEOUT

/// Dehydration of the cloud file is disallowed by the cloud sync provider.
pub const DEHYDRATION_DISALLOWED : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(434); // ERROR_CLOUD_FILE_DEHYDRATION_DISALLOWED
