// WARNING: this file is auto-generated by xtask gen and may be overwritten

use super::*;


/// Catastrophic failure
pub const UNEXPECTED : HRESULT = HRESULT::from_constant(0x8000FFFF); // E_UNEXPECTED

/// Not implemented
pub const NOTIMPL : HRESULT = HRESULT::from_constant(0x80004001); // E_NOTIMPL

/// Ran out of memory
pub const OUTOFMEMORY : HRESULT = HRESULT::from_constant(0x8007000E); // E_OUTOFMEMORY

/// One or more arguments are invalid
pub const INVALIDARG : HRESULT = HRESULT::from_constant(0x80070057); // E_INVALIDARG

/// No such interface supported
pub const NOINTERFACE : HRESULT = HRESULT::from_constant(0x80004002); // E_NOINTERFACE

/// Invalid pointer
pub const POINTER : HRESULT = HRESULT::from_constant(0x80004003); // E_POINTER

/// Invalid handle
pub const HANDLE : HRESULT = HRESULT::from_constant(0x80070006); // E_HANDLE

/// Operation aborted
pub const ABORT : HRESULT = HRESULT::from_constant(0x80004004); // E_ABORT

/// Unspecified error
pub const FAIL : HRESULT = HRESULT::from_constant(0x80004005); // E_FAIL

/// General access denied error
pub const ACCESSDENIED : HRESULT = HRESULT::from_constant(0x80070005); // E_ACCESSDENIED

/// The data necessary to complete this operation is not yet available.
pub const PENDING : HRESULT = HRESULT::from_constant(0x8000000A); // E_PENDING

/// The operation attempted to access data outside the valid range
pub const BOUNDS : HRESULT = HRESULT::from_constant(0x8000000B); // E_BOUNDS

/// A concurrent or interleaved operation changed the state of the object, invalidating this operation.
pub const CHANGED_STATE : HRESULT = HRESULT::from_constant(0x8000000C); // E_CHANGED_STATE

/// An illegal state change was requested.
pub const ILLEGAL_STATE_CHANGE : HRESULT = HRESULT::from_constant(0x8000000D); // E_ILLEGAL_STATE_CHANGE

/// A method was called at an unexpected time.
pub const ILLEGAL_METHOD_CALL : HRESULT = HRESULT::from_constant(0x8000000E); // E_ILLEGAL_METHOD_CALL

/// String not null terminated.
pub const STRING_NOT_NULL_TERMINATED : HRESULT = HRESULT::from_constant(0x80000017); // E_STRING_NOT_NULL_TERMINATED

/// A delegate was assigned when not allowed.
pub const ILLEGAL_DELEGATE_ASSIGNMENT : HRESULT = HRESULT::from_constant(0x80000018); // E_ILLEGAL_DELEGATE_ASSIGNMENT

/// An async operation was not properly started.
pub const ASYNC_OPERATION_NOT_STARTED : HRESULT = HRESULT::from_constant(0x80000019); // E_ASYNC_OPERATION_NOT_STARTED

/// The application is exiting and cannot service this request
pub const APPLICATION_EXITING : HRESULT = HRESULT::from_constant(0x8000001A); // E_APPLICATION_EXITING

/// The application view is exiting and cannot service this request
pub const APPLICATION_VIEW_EXITING : HRESULT = HRESULT::from_constant(0x8000001B); // E_APPLICATION_VIEW_EXITING

/// Context is not activated.
pub const MBN_CONTEXT_NOT_ACTIVATED : HRESULT = HRESULT::from_constant(0x80548201); // E_MBN_CONTEXT_NOT_ACTIVATED

/// Bad SIM is inserted.
pub const MBN_BAD_SIM : HRESULT = HRESULT::from_constant(0x80548202); // E_MBN_BAD_SIM

/// Requested data class is not available.
pub const MBN_DATA_CLASS_NOT_AVAILABLE : HRESULT = HRESULT::from_constant(0x80548203); // E_MBN_DATA_CLASS_NOT_AVAILABLE

/// Access point name (APN) or Access string is incorrect.
pub const MBN_INVALID_ACCESS_STRING : HRESULT = HRESULT::from_constant(0x80548204); // E_MBN_INVALID_ACCESS_STRING

/// Max activated contexts have reached.
pub const MBN_MAX_ACTIVATED_CONTEXTS : HRESULT = HRESULT::from_constant(0x80548205); // E_MBN_MAX_ACTIVATED_CONTEXTS

/// Device is in packet detach state.
pub const MBN_PACKET_SVC_DETACHED : HRESULT = HRESULT::from_constant(0x80548206); // E_MBN_PACKET_SVC_DETACHED

/// Provider is not visible.
pub const MBN_PROVIDER_NOT_VISIBLE : HRESULT = HRESULT::from_constant(0x80548207); // E_MBN_PROVIDER_NOT_VISIBLE

/// Radio is powered off.
pub const MBN_RADIO_POWER_OFF : HRESULT = HRESULT::from_constant(0x80548208); // E_MBN_RADIO_POWER_OFF

/// MBN subscription is not activated.
pub const MBN_SERVICE_NOT_ACTIVATED : HRESULT = HRESULT::from_constant(0x80548209); // E_MBN_SERVICE_NOT_ACTIVATED

/// SIM is not inserted.
pub const MBN_SIM_NOT_INSERTED : HRESULT = HRESULT::from_constant(0x8054820A); // E_MBN_SIM_NOT_INSERTED

/// Voice call in progress.
pub const MBN_VOICE_CALL_IN_PROGRESS : HRESULT = HRESULT::from_constant(0x8054820B); // E_MBN_VOICE_CALL_IN_PROGRESS

/// Visible provider cache is invalid.
pub const MBN_INVALID_CACHE : HRESULT = HRESULT::from_constant(0x8054820C); // E_MBN_INVALID_CACHE

/// Device is not registered.
pub const MBN_NOT_REGISTERED : HRESULT = HRESULT::from_constant(0x8054820D); // E_MBN_NOT_REGISTERED

/// Providers not found.
pub const MBN_PROVIDERS_NOT_FOUND : HRESULT = HRESULT::from_constant(0x8054820E); // E_MBN_PROVIDERS_NOT_FOUND

/// Pin is not supported.
pub const MBN_PIN_NOT_SUPPORTED : HRESULT = HRESULT::from_constant(0x8054820F); // E_MBN_PIN_NOT_SUPPORTED

/// Pin is required.
pub const MBN_PIN_REQUIRED : HRESULT = HRESULT::from_constant(0x80548210); // E_MBN_PIN_REQUIRED

/// PIN is disabled.
pub const MBN_PIN_DISABLED : HRESULT = HRESULT::from_constant(0x80548211); // E_MBN_PIN_DISABLED

/// Generic Failure.
pub const MBN_FAILURE : HRESULT = HRESULT::from_constant(0x80548212); // E_MBN_FAILURE

/// Profile is invalid.
pub const MBN_INVALID_PROFILE : HRESULT = HRESULT::from_constant(0x80548218); // E_MBN_INVALID_PROFILE

/// Default profile exist.
pub const MBN_DEFAULT_PROFILE_EXIST : HRESULT = HRESULT::from_constant(0x80548219); // E_MBN_DEFAULT_PROFILE_EXIST

/// SMS encoding is not supported.
pub const MBN_SMS_ENCODING_NOT_SUPPORTED : HRESULT = HRESULT::from_constant(0x80548220); // E_MBN_SMS_ENCODING_NOT_SUPPORTED

/// SMS filter is not supported.
pub const MBN_SMS_FILTER_NOT_SUPPORTED : HRESULT = HRESULT::from_constant(0x80548221); // E_MBN_SMS_FILTER_NOT_SUPPORTED

/// Invalid SMS memory index is used.
pub const MBN_SMS_INVALID_MEMORY_INDEX : HRESULT = HRESULT::from_constant(0x80548222); // E_MBN_SMS_INVALID_MEMORY_INDEX

/// SMS language is not supported.
pub const MBN_SMS_LANG_NOT_SUPPORTED : HRESULT = HRESULT::from_constant(0x80548223); // E_MBN_SMS_LANG_NOT_SUPPORTED

/// SMS memory failure occurred.
pub const MBN_SMS_MEMORY_FAILURE : HRESULT = HRESULT::from_constant(0x80548224); // E_MBN_SMS_MEMORY_FAILURE

/// SMS network timeout happened.
pub const MBN_SMS_NETWORK_TIMEOUT : HRESULT = HRESULT::from_constant(0x80548225); // E_MBN_SMS_NETWORK_TIMEOUT

/// Unknown SMSC address is used.
pub const MBN_SMS_UNKNOWN_SMSC_ADDRESS : HRESULT = HRESULT::from_constant(0x80548226); // E_MBN_SMS_UNKNOWN_SMSC_ADDRESS

/// SMS format is not supported.
pub const MBN_SMS_FORMAT_NOT_SUPPORTED : HRESULT = HRESULT::from_constant(0x80548227); // E_MBN_SMS_FORMAT_NOT_SUPPORTED

/// SMS operation is not allowed.
pub const MBN_SMS_OPERATION_NOT_ALLOWED : HRESULT = HRESULT::from_constant(0x80548228); // E_MBN_SMS_OPERATION_NOT_ALLOWED

/// Device SMS memory is full.
pub const MBN_SMS_MEMORY_FULL : HRESULT = HRESULT::from_constant(0x80548229); // E_MBN_SMS_MEMORY_FULL

/// The attribute handle given was not valid on this server.
pub const BLUETOOTH_ATT_INVALID_HANDLE : HRESULT = HRESULT::from_constant(0x80650001); // E_BLUETOOTH_ATT_INVALID_HANDLE

/// The attribute cannot be read.
pub const BLUETOOTH_ATT_READ_NOT_PERMITTED : HRESULT = HRESULT::from_constant(0x80650002); // E_BLUETOOTH_ATT_READ_NOT_PERMITTED

/// The attribute cannot be written.
pub const BLUETOOTH_ATT_WRITE_NOT_PERMITTED : HRESULT = HRESULT::from_constant(0x80650003); // E_BLUETOOTH_ATT_WRITE_NOT_PERMITTED

/// The attribute PDU was invalid.
pub const BLUETOOTH_ATT_INVALID_PDU : HRESULT = HRESULT::from_constant(0x80650004); // E_BLUETOOTH_ATT_INVALID_PDU

/// The attribute requires authentication before it can be read or written.
pub const BLUETOOTH_ATT_INSUFFICIENT_AUTHENTICATION : HRESULT = HRESULT::from_constant(0x80650005); // E_BLUETOOTH_ATT_INSUFFICIENT_AUTHENTICATION

/// Attribute server does not support the request received from the client.
pub const BLUETOOTH_ATT_REQUEST_NOT_SUPPORTED : HRESULT = HRESULT::from_constant(0x80650006); // E_BLUETOOTH_ATT_REQUEST_NOT_SUPPORTED

/// Offset specified was past the end of the attribute.
pub const BLUETOOTH_ATT_INVALID_OFFSET : HRESULT = HRESULT::from_constant(0x80650007); // E_BLUETOOTH_ATT_INVALID_OFFSET

/// The attribute requires authorization before it can be read or written.
pub const BLUETOOTH_ATT_INSUFFICIENT_AUTHORIZATION : HRESULT = HRESULT::from_constant(0x80650008); // E_BLUETOOTH_ATT_INSUFFICIENT_AUTHORIZATION

/// Too many prepare writes have been queued.
pub const BLUETOOTH_ATT_PREPARE_QUEUE_FULL : HRESULT = HRESULT::from_constant(0x80650009); // E_BLUETOOTH_ATT_PREPARE_QUEUE_FULL

/// No attribute found within the given attribute handle range.
pub const BLUETOOTH_ATT_ATTRIBUTE_NOT_FOUND : HRESULT = HRESULT::from_constant(0x8065000A); // E_BLUETOOTH_ATT_ATTRIBUTE_NOT_FOUND

/// The attribute cannot be read or written using the Read Blob Request.
pub const BLUETOOTH_ATT_ATTRIBUTE_NOT_LONG : HRESULT = HRESULT::from_constant(0x8065000B); // E_BLUETOOTH_ATT_ATTRIBUTE_NOT_LONG

/// The Encryption Key Size used for encrypting this link is insufficient.
pub const BLUETOOTH_ATT_INSUFFICIENT_ENCRYPTION_KEY_SIZE : HRESULT = HRESULT::from_constant(0x8065000C); // E_BLUETOOTH_ATT_INSUFFICIENT_ENCRYPTION_KEY_SIZE

/// The attribute value length is invalid for the operation.
pub const BLUETOOTH_ATT_INVALID_ATTRIBUTE_VALUE_LENGTH : HRESULT = HRESULT::from_constant(0x8065000D); // E_BLUETOOTH_ATT_INVALID_ATTRIBUTE_VALUE_LENGTH

/// The attribute request that was requested has encountered an error that was unlikely, and therefore could not be completed as requested.
pub const BLUETOOTH_ATT_UNLIKELY : HRESULT = HRESULT::from_constant(0x8065000E); // E_BLUETOOTH_ATT_UNLIKELY

/// The attribute requires encryption before it can be read or written.
pub const BLUETOOTH_ATT_INSUFFICIENT_ENCRYPTION : HRESULT = HRESULT::from_constant(0x8065000F); // E_BLUETOOTH_ATT_INSUFFICIENT_ENCRYPTION

/// The attribute type is not a supported grouping attribute as defined by a higher layer specification.
pub const BLUETOOTH_ATT_UNSUPPORTED_GROUP_TYPE : HRESULT = HRESULT::from_constant(0x80650010); // E_BLUETOOTH_ATT_UNSUPPORTED_GROUP_TYPE

/// Insufficient Resources to complete the request.
pub const BLUETOOTH_ATT_INSUFFICIENT_RESOURCES : HRESULT = HRESULT::from_constant(0x80650011); // E_BLUETOOTH_ATT_INSUFFICIENT_RESOURCES

/// An error that lies in the reserved range has been received.
pub const BLUETOOTH_ATT_UNKNOWN_ERROR : HRESULT = HRESULT::from_constant(0x80651000); // E_BLUETOOTH_ATT_UNKNOWN_ERROR

/// PortCls could not find an audio engine node exposed by a miniport driver claiming support for IMiniportAudioEngineNode.
pub const AUDIO_ENGINE_NODE_NOT_FOUND : HRESULT = HRESULT::from_constant(0x80660001); // E_AUDIO_ENGINE_NODE_NOT_FOUND

/// HD Audio widget encountered an unexpected empty connection list.
pub const HDAUDIO_EMPTY_CONNECTION_LIST : HRESULT = HRESULT::from_constant(0x80660002); // E_HDAUDIO_EMPTY_CONNECTION_LIST

/// HD Audio widget does not support the connection list parameter.
pub const HDAUDIO_CONNECTION_LIST_NOT_SUPPORTED : HRESULT = HRESULT::from_constant(0x80660003); // E_HDAUDIO_CONNECTION_LIST_NOT_SUPPORTED

/// No HD Audio subdevices were successfully created.
pub const HDAUDIO_NO_LOGICAL_DEVICES_CREATED : HRESULT = HRESULT::from_constant(0x80660004); // E_HDAUDIO_NO_LOGICAL_DEVICES_CREATED

/// An unexpected NULL pointer was encountered in a linked list.
pub const HDAUDIO_NULL_LINKED_LIST_ENTRY : HRESULT = HRESULT::from_constant(0x80660005); // E_HDAUDIO_NULL_LINKED_LIST_ENTRY

/// This app can't start because the screen resolution is below 1024x768. Choose a higher screen resolution and then try again.
pub const MONITOR_RESOLUTION_TOO_LOW : HRESULT = HRESULT::from_constant(0x80270250); // E_MONITOR_RESOLUTION_TOO_LOW

/// This app can't be activated from an elevated context.
pub const ELEVATED_ACTIVATION_NOT_SUPPORTED : HRESULT = HRESULT::from_constant(0x80270251); // E_ELEVATED_ACTIVATION_NOT_SUPPORTED

/// This app can't be activated when UAC is disabled.
pub const UAC_DISABLED : HRESULT = HRESULT::from_constant(0x80270252); // E_UAC_DISABLED

/// This app can't be activated by the Built-in Administrator.
pub const FULL_ADMIN_NOT_SUPPORTED : HRESULT = HRESULT::from_constant(0x80270253); // E_FULL_ADMIN_NOT_SUPPORTED

/// This app does not support the contract specified or is not installed.
pub const APPLICATION_NOT_REGISTERED : HRESULT = HRESULT::from_constant(0x80270254); // E_APPLICATION_NOT_REGISTERED

/// This app has multiple extensions registered to support the specified contract. Activation by AppUserModelId is ambiguous.
pub const MULTIPLE_EXTENSIONS_FOR_APPLICATION : HRESULT = HRESULT::from_constant(0x80270255); // E_MULTIPLE_EXTENSIONS_FOR_APPLICATION

/// This app's package family has more than one package installed. This is not supported.
pub const MULTIPLE_PACKAGES_FOR_FAMILY : HRESULT = HRESULT::from_constant(0x80270256); // E_MULTIPLE_PACKAGES_FOR_FAMILY

/// The app manager is required to activate applications, but is not running.
pub const APPLICATION_MANAGER_NOT_RUNNING : HRESULT = HRESULT::from_constant(0x80270257); // E_APPLICATION_MANAGER_NOT_RUNNING

/// The app didn't start in the required time.
pub const APPLICATION_ACTIVATION_TIMED_OUT : HRESULT = HRESULT::from_constant(0x8027025A); // E_APPLICATION_ACTIVATION_TIMED_OUT

/// The app didn't start.
pub const APPLICATION_ACTIVATION_EXEC_FAILURE : HRESULT = HRESULT::from_constant(0x8027025B); // E_APPLICATION_ACTIVATION_EXEC_FAILURE

/// This app failed to launch because of an issue with its license. Please try again in a moment.
pub const APPLICATION_TEMPORARY_LICENSE_ERROR : HRESULT = HRESULT::from_constant(0x8027025C); // E_APPLICATION_TEMPORARY_LICENSE_ERROR

/// This app failed to launch because its trial license has expired.
pub const APPLICATION_TRIAL_LICENSE_EXPIRED : HRESULT = HRESULT::from_constant(0x8027025D); // E_APPLICATION_TRIAL_LICENSE_EXPIRED

/// Please choose a folder on a drive that's formatted with the NTFS file system.
pub const SKYDRIVE_ROOT_TARGET_FILE_SYSTEM_NOT_SUPPORTED : HRESULT = HRESULT::from_constant(0x80270260); // E_SKYDRIVE_ROOT_TARGET_FILE_SYSTEM_NOT_SUPPORTED

/// This location is already being used. Please choose a different location.
pub const SKYDRIVE_ROOT_TARGET_OVERLAP : HRESULT = HRESULT::from_constant(0x80270261); // E_SKYDRIVE_ROOT_TARGET_OVERLAP

/// This location cannot be indexed. Please choose a different location.
pub const SKYDRIVE_ROOT_TARGET_CANNOT_INDEX : HRESULT = HRESULT::from_constant(0x80270262); // E_SKYDRIVE_ROOT_TARGET_CANNOT_INDEX

/// Sorry, the action couldn't be completed because the file hasn't finished uploading. Try again later.
pub const SKYDRIVE_FILE_NOT_UPLOADED : HRESULT = HRESULT::from_constant(0x80270263); // E_SKYDRIVE_FILE_NOT_UPLOADED

/// Sorry, the action couldn't be completed.
pub const SKYDRIVE_UPDATE_AVAILABILITY_FAIL : HRESULT = HRESULT::from_constant(0x80270264); // E_SKYDRIVE_UPDATE_AVAILABILITY_FAIL

/// This content can only be moved to a folder. To move the content to this drive, please choose or create a folder.
pub const SKYDRIVE_ROOT_TARGET_VOLUME_ROOT_NOT_SUPPORTED : HRESULT = HRESULT::from_constant(0x80270265); // E_SKYDRIVE_ROOT_TARGET_VOLUME_ROOT_NOT_SUPPORTED

/// The file size is larger than supported by the sync engine.
pub const SYNCENGINE_FILE_SIZE_OVER_LIMIT : HRESULT = HRESULT::from_constant(0x8802B001); // E_SYNCENGINE_FILE_SIZE_OVER_LIMIT

/// The file cannot be uploaded because it doesn't fit in the user's available service provided storage space.
pub const SYNCENGINE_FILE_SIZE_EXCEEDS_REMAINING_QUOTA : HRESULT = HRESULT::from_constant(0x8802B002); // E_SYNCENGINE_FILE_SIZE_EXCEEDS_REMAINING_QUOTA

/// The file name contains invalid characters.
pub const SYNCENGINE_UNSUPPORTED_FILE_NAME : HRESULT = HRESULT::from_constant(0x8802B003); // E_SYNCENGINE_UNSUPPORTED_FILE_NAME

/// The maximum file count has been reached for this folder in the sync engine.
pub const SYNCENGINE_FOLDER_ITEM_COUNT_LIMIT_EXCEEDED : HRESULT = HRESULT::from_constant(0x8802B004); // E_SYNCENGINE_FOLDER_ITEM_COUNT_LIMIT_EXCEEDED

/// The file sync has been delegated to another program and has run into an issue.
pub const SYNCENGINE_FILE_SYNC_PARTNER_ERROR : HRESULT = HRESULT::from_constant(0x8802B005); // E_SYNCENGINE_FILE_SYNC_PARTNER_ERROR

/// Sync has been delayed due to a throttling request from the service.
pub const SYNCENGINE_SYNC_PAUSED_BY_SERVICE : HRESULT = HRESULT::from_constant(0x8802B006); // E_SYNCENGINE_SYNC_PAUSED_BY_SERVICE

/// We can't seem to find that file. Please try again later.
pub const SYNCENGINE_FILE_IDENTIFIER_UNKNOWN : HRESULT = HRESULT::from_constant(0x8802C002); // E_SYNCENGINE_FILE_IDENTIFIER_UNKNOWN

/// The account you're signed in with doesn't have permission to open this file.
pub const SYNCENGINE_SERVICE_AUTHENTICATION_FAILED : HRESULT = HRESULT::from_constant(0x8802C003); // E_SYNCENGINE_SERVICE_AUTHENTICATION_FAILED

/// There was a problem connecting to the service. Please try again later.
pub const SYNCENGINE_UNKNOWN_SERVICE_ERROR : HRESULT = HRESULT::from_constant(0x8802C004); // E_SYNCENGINE_UNKNOWN_SERVICE_ERROR

/// Sorry, there was a problem downloading the file.
pub const SYNCENGINE_SERVICE_RETURNED_UNEXPECTED_SIZE : HRESULT = HRESULT::from_constant(0x8802C005); // E_SYNCENGINE_SERVICE_RETURNED_UNEXPECTED_SIZE

/// We're having trouble downloading the file right now. Please try again later.
pub const SYNCENGINE_REQUEST_BLOCKED_BY_SERVICE : HRESULT = HRESULT::from_constant(0x8802C006); // E_SYNCENGINE_REQUEST_BLOCKED_BY_SERVICE

/// We're having trouble downloading the file right now. Please try again later.
pub const SYNCENGINE_REQUEST_BLOCKED_DUE_TO_CLIENT_ERROR : HRESULT = HRESULT::from_constant(0x8802C007); // E_SYNCENGINE_REQUEST_BLOCKED_DUE_TO_CLIENT_ERROR

/// The sync engine does not have permissions to access a local folder under the sync root.
pub const SYNCENGINE_FOLDER_INACCESSIBLE : HRESULT = HRESULT::from_constant(0x8802D001); // E_SYNCENGINE_FOLDER_INACCESSIBLE

/// The folder name contains invalid characters.
pub const SYNCENGINE_UNSUPPORTED_FOLDER_NAME : HRESULT = HRESULT::from_constant(0x8802D002); // E_SYNCENGINE_UNSUPPORTED_FOLDER_NAME

/// The sync engine is not allowed to run in your current market.
pub const SYNCENGINE_UNSUPPORTED_MARKET : HRESULT = HRESULT::from_constant(0x8802D003); // E_SYNCENGINE_UNSUPPORTED_MARKET

/// All files and folders can't be uploaded because a path of a file or folder is too long.
pub const SYNCENGINE_PATH_LENGTH_LIMIT_EXCEEDED : HRESULT = HRESULT::from_constant(0x8802D004); // E_SYNCENGINE_PATH_LENGTH_LIMIT_EXCEEDED

/// All file and folders cannot be synchronized because a path of a file or folder would exceed the local path limit.
pub const SYNCENGINE_REMOTE_PATH_LENGTH_LIMIT_EXCEEDED : HRESULT = HRESULT::from_constant(0x8802D005); // E_SYNCENGINE_REMOTE_PATH_LENGTH_LIMIT_EXCEEDED

/// Updates are needed in order to use the sync engine.
pub const SYNCENGINE_CLIENT_UPDATE_NEEDED : HRESULT = HRESULT::from_constant(0x8802D006); // E_SYNCENGINE_CLIENT_UPDATE_NEEDED

/// The sync engine needs to authenticate with a proxy server.
pub const SYNCENGINE_PROXY_AUTHENTICATION_REQUIRED : HRESULT = HRESULT::from_constant(0x8802D007); // E_SYNCENGINE_PROXY_AUTHENTICATION_REQUIRED

/// There was a problem setting up the storage services for the account.
pub const SYNCENGINE_STORAGE_SERVICE_PROVISIONING_FAILED : HRESULT = HRESULT::from_constant(0x8802D008); // E_SYNCENGINE_STORAGE_SERVICE_PROVISIONING_FAILED

/// Files can't be uploaded because there's an unsupported reparse point.
pub const SYNCENGINE_UNSUPPORTED_REPARSE_POINT : HRESULT = HRESULT::from_constant(0x8802D009); // E_SYNCENGINE_UNSUPPORTED_REPARSE_POINT

/// The service has blocked your account from accessing the storage service.
pub const SYNCENGINE_STORAGE_SERVICE_BLOCKED : HRESULT = HRESULT::from_constant(0x8802D00A); // E_SYNCENGINE_STORAGE_SERVICE_BLOCKED

/// The action can't be performed right now because this folder is being moved. Please try again later.
pub const SYNCENGINE_FOLDER_IN_REDIRECTION : HRESULT = HRESULT::from_constant(0x8802D00B); // E_SYNCENGINE_FOLDER_IN_REDIRECTION

/// Invalid operation performed by the protocol.
pub const INVALID_PROTOCOL_OPERATION : HRESULT = HRESULT::from_constant(0x83760001); // E_INVALID_PROTOCOL_OPERATION

/// Invalid data format for the specific protocol operation.
pub const INVALID_PROTOCOL_FORMAT : HRESULT = HRESULT::from_constant(0x83760002); // E_INVALID_PROTOCOL_FORMAT

/// Protocol extensions are not supported.
pub const PROTOCOL_EXTENSIONS_NOT_SUPPORTED : HRESULT = HRESULT::from_constant(0x83760003); // E_PROTOCOL_EXTENSIONS_NOT_SUPPORTED

/// Subprotocol is not supported.
pub const SUBPROTOCOL_NOT_SUPPORTED : HRESULT = HRESULT::from_constant(0x83760004); // E_SUBPROTOCOL_NOT_SUPPORTED

/// Incorrect protocol version.
pub const PROTOCOL_VERSION_NOT_SUPPORTED : HRESULT = HRESULT::from_constant(0x83760005); // E_PROTOCOL_VERSION_NOT_SUPPORTED
