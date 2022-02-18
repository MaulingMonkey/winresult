// WARNING: this file is auto-generated by xtask gen and may be overwritten

use super::*;


/// The application made a call that is invalid. Either the parameters of the call or the state of some object was incorrect.
/// Enable the D3D debug layer in order to see details via debug messages.
pub const ERROR_INVALID_CALL : ErrorHResult = ErrorHResult::from_constant(0x887A0001); // DXGI_ERROR_INVALID_CALL

/// The object was not found. If calling IDXGIFactory::EnumAdaptes, there is no adapter with the specified ordinal.
pub const ERROR_NOT_FOUND : ErrorHResult = ErrorHResult::from_constant(0x887A0002); // DXGI_ERROR_NOT_FOUND

/// The caller did not supply a sufficiently large buffer.
pub const ERROR_MORE_DATA : ErrorHResult = ErrorHResult::from_constant(0x887A0003); // DXGI_ERROR_MORE_DATA

/// The specified device interface or feature level is not supported on this system.
pub const ERROR_UNSUPPORTED : ErrorHResult = ErrorHResult::from_constant(0x887A0004); // DXGI_ERROR_UNSUPPORTED

/// The GPU device instance has been suspended. Use GetDeviceRemovedReason to determine the appropriate action.
pub const ERROR_DEVICE_REMOVED : ErrorHResult = ErrorHResult::from_constant(0x887A0005); // DXGI_ERROR_DEVICE_REMOVED

/// The GPU will not respond to more commands, most likely because of an invalid command passed by the calling application.
pub const ERROR_DEVICE_HUNG : ErrorHResult = ErrorHResult::from_constant(0x887A0006); // DXGI_ERROR_DEVICE_HUNG

/// The GPU will not respond to more commands, most likely because some other application submitted invalid commands.
/// The calling application should re-create the device and continue.
pub const ERROR_DEVICE_RESET : ErrorHResult = ErrorHResult::from_constant(0x887A0007); // DXGI_ERROR_DEVICE_RESET

/// The GPU was busy at the moment when the call was made, and the call was neither executed nor scheduled.
pub const ERROR_WAS_STILL_DRAWING : ErrorHResult = ErrorHResult::from_constant(0x887A000A); // DXGI_ERROR_WAS_STILL_DRAWING

/// An event (such as power cycle) interrupted the gathering of presentation statistics. Any previous statistics should be
/// considered invalid.
pub const ERROR_FRAME_STATISTICS_DISJOINT : ErrorHResult = ErrorHResult::from_constant(0x887A000B); // DXGI_ERROR_FRAME_STATISTICS_DISJOINT

/// Fullscreen mode could not be achieved because the specified output was already in use.
pub const ERROR_GRAPHICS_VIDPN_SOURCE_IN_USE : ErrorHResult = ErrorHResult::from_constant(0x887A000C); // DXGI_ERROR_GRAPHICS_VIDPN_SOURCE_IN_USE

/// An internal issue prevented the driver from carrying out the specified operation. The driver's state is probably suspect,
/// and the application should not continue.
pub const ERROR_DRIVER_INTERNAL_ERROR : ErrorHResult = ErrorHResult::from_constant(0x887A0020); // DXGI_ERROR_DRIVER_INTERNAL_ERROR

/// A global counter resource was in use, and the specified counter cannot be used by this Direct3D device at this time.
pub const ERROR_NONEXCLUSIVE : ErrorHResult = ErrorHResult::from_constant(0x887A0021); // DXGI_ERROR_NONEXCLUSIVE

/// A resource is not available at the time of the call, but may become available later.
pub const ERROR_NOT_CURRENTLY_AVAILABLE : ErrorHResult = ErrorHResult::from_constant(0x887A0022); // DXGI_ERROR_NOT_CURRENTLY_AVAILABLE

/// The application's remote device has been removed due to session disconnect or network disconnect.
/// The application should call IDXGIFactory1::IsCurrent to find out when the remote device becomes available again.
pub const ERROR_REMOTE_CLIENT_DISCONNECTED : ErrorHResult = ErrorHResult::from_constant(0x887A0023); // DXGI_ERROR_REMOTE_CLIENT_DISCONNECTED

/// The device has been removed during a remote session because the remote computer ran out of memory.
pub const ERROR_REMOTE_OUTOFMEMORY : ErrorHResult = ErrorHResult::from_constant(0x887A0024); // DXGI_ERROR_REMOTE_OUTOFMEMORY

/// The keyed mutex was abandoned.
pub const ERROR_ACCESS_LOST : ErrorHResult = ErrorHResult::from_constant(0x887A0026); // DXGI_ERROR_ACCESS_LOST

/// The timeout value has elapsed and the resource is not yet available.
pub const ERROR_WAIT_TIMEOUT : ErrorHResult = ErrorHResult::from_constant(0x887A0027); // DXGI_ERROR_WAIT_TIMEOUT

/// The output duplication has been turned off because the Windows session ended or was disconnected.
/// This happens when a remote user disconnects, or when "switch user" is used locally.
pub const ERROR_SESSION_DISCONNECTED : ErrorHResult = ErrorHResult::from_constant(0x887A0028); // DXGI_ERROR_SESSION_DISCONNECTED

/// The DXGI output (monitor) to which the swapchain content was restricted, has been disconnected or changed.
pub const ERROR_RESTRICT_TO_OUTPUT_STALE : ErrorHResult = ErrorHResult::from_constant(0x887A0029); // DXGI_ERROR_RESTRICT_TO_OUTPUT_STALE

/// DXGI is unable to provide content protection on the swapchain. This is typically caused by an older driver,
/// or by the application using a swapchain that is incompatible with content protection.
pub const ERROR_CANNOT_PROTECT_CONTENT : ErrorHResult = ErrorHResult::from_constant(0x887A002A); // DXGI_ERROR_CANNOT_PROTECT_CONTENT

/// The application is trying to use a resource to which it does not have the required access privileges.
/// This is most commonly caused by writing to a shared resource with read-only access.
pub const ERROR_ACCESS_DENIED : ErrorHResult = ErrorHResult::from_constant(0x887A002B); // DXGI_ERROR_ACCESS_DENIED

/// The application is trying to create a shared handle using a name that is already associated with some other resource.
pub const ERROR_NAME_ALREADY_EXISTS : ErrorHResult = ErrorHResult::from_constant(0x887A002C); // DXGI_ERROR_NAME_ALREADY_EXISTS

/// The application requested an operation that depends on an SDK component that is missing or mismatched.
pub const ERROR_SDK_COMPONENT_MISSING : ErrorHResult = ErrorHResult::from_constant(0x887A002D); // DXGI_ERROR_SDK_COMPONENT_MISSING

/// The DXGI objects that the application has created are no longer current & need to be recreated for this operation to be performed.
pub const ERROR_NOT_CURRENT : ErrorHResult = ErrorHResult::from_constant(0x887A002E); // DXGI_ERROR_NOT_CURRENT

/// Insufficient HW protected memory exits for proper function.
pub const ERROR_HW_PROTECTION_OUTOFMEMORY : ErrorHResult = ErrorHResult::from_constant(0x887A0030); // DXGI_ERROR_HW_PROTECTION_OUTOFMEMORY

/// Creating this device would violate the process's dynamic code policy.
pub const ERROR_DYNAMIC_CODE_POLICY_VIOLATION : ErrorHResult = ErrorHResult::from_constant(0x887A0031); // DXGI_ERROR_DYNAMIC_CODE_POLICY_VIOLATION

/// The operation failed because the compositor is not in control of the output.
pub const ERROR_NON_COMPOSITED_UI : ErrorHResult = ErrorHResult::from_constant(0x887A0032); // DXGI_ERROR_NON_COMPOSITED_UI

/// An on-going mode change prevented completion of the call. The call may succeed if attempted later.
pub const ERROR_MODE_CHANGE_IN_PROGRESS : ErrorHResult = ErrorHResult::from_constant(0x887A0025); // DXGI_ERROR_MODE_CHANGE_IN_PROGRESS

/// The cache is corrupt and either could not be opened or could not be reset.
pub const ERROR_CACHE_CORRUPT : ErrorHResult = ErrorHResult::from_constant(0x887A0033); // DXGI_ERROR_CACHE_CORRUPT

/// This entry would cause the cache to exceed its quota. On a load operation, this may indicate exceeding the maximum in-memory size.
pub const ERROR_CACHE_FULL : ErrorHResult = ErrorHResult::from_constant(0x887A0034); // DXGI_ERROR_CACHE_FULL

/// A cache entry was found, but the key provided does not match the key stored in the entry.
pub const ERROR_CACHE_HASH_COLLISION : ErrorHResult = ErrorHResult::from_constant(0x887A0035); // DXGI_ERROR_CACHE_HASH_COLLISION

/// The desired element already exists.
pub const ERROR_ALREADY_EXISTS : ErrorHResult = ErrorHResult::from_constant(0x887A0036); // DXGI_ERROR_ALREADY_EXISTS