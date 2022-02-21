// WARNING: this file is auto-generated by xtask gen and may be overwritten

use super::*;


/// A non-empty line was encountered in the INF before the start of a section.
pub const E_EXPECTED_SECTION_NAME : HResultError = HResultError::from_constant(0x800F0000); // SPAPI_E_EXPECTED_SECTION_NAME

/// A section name marker in the INF is not complete, or does not exist on a line by itself.
pub const E_BAD_SECTION_NAME_LINE : HResultError = HResultError::from_constant(0x800F0001); // SPAPI_E_BAD_SECTION_NAME_LINE

/// An INF section was encountered whose name exceeds the maximum section name length.
pub const E_SECTION_NAME_TOO_LONG : HResultError = HResultError::from_constant(0x800F0002); // SPAPI_E_SECTION_NAME_TOO_LONG

/// The syntax of the INF is invalid.
pub const E_GENERAL_SYNTAX : HResultError = HResultError::from_constant(0x800F0003); // SPAPI_E_GENERAL_SYNTAX

/// The style of the INF is different than what was requested.
pub const E_WRONG_INF_STYLE : HResultError = HResultError::from_constant(0x800F0100); // SPAPI_E_WRONG_INF_STYLE

/// The required section was not found in the INF.
pub const E_SECTION_NOT_FOUND : HResultError = HResultError::from_constant(0x800F0101); // SPAPI_E_SECTION_NOT_FOUND

/// The required line was not found in the INF.
pub const E_LINE_NOT_FOUND : HResultError = HResultError::from_constant(0x800F0102); // SPAPI_E_LINE_NOT_FOUND

/// The files affected by the installation of this file queue have not been backed up for uninstall.
pub const E_NO_BACKUP : HResultError = HResultError::from_constant(0x800F0103); // SPAPI_E_NO_BACKUP

/// The INF or the device information set or element does not have an associated install class.
pub const E_NO_ASSOCIATED_CLASS : HResultError = HResultError::from_constant(0x800F0200); // SPAPI_E_NO_ASSOCIATED_CLASS

/// The INF or the device information set or element does not match the specified install class.
pub const E_CLASS_MISMATCH : HResultError = HResultError::from_constant(0x800F0201); // SPAPI_E_CLASS_MISMATCH

/// An existing device was found that is a duplicate of the device being manually installed.
pub const E_DUPLICATE_FOUND : HResultError = HResultError::from_constant(0x800F0202); // SPAPI_E_DUPLICATE_FOUND

/// There is no driver selected for the device information set or element.
pub const E_NO_DRIVER_SELECTED : HResultError = HResultError::from_constant(0x800F0203); // SPAPI_E_NO_DRIVER_SELECTED

/// The requested device registry key does not exist.
pub const E_KEY_DOES_NOT_EXIST : HResultError = HResultError::from_constant(0x800F0204); // SPAPI_E_KEY_DOES_NOT_EXIST

/// The device instance name is invalid.
pub const E_INVALID_DEVINST_NAME : HResultError = HResultError::from_constant(0x800F0205); // SPAPI_E_INVALID_DEVINST_NAME

/// The install class is not present or is invalid.
pub const E_INVALID_CLASS : HResultError = HResultError::from_constant(0x800F0206); // SPAPI_E_INVALID_CLASS

/// The device instance cannot be created because it already exists.
pub const E_DEVINST_ALREADY_EXISTS : HResultError = HResultError::from_constant(0x800F0207); // SPAPI_E_DEVINST_ALREADY_EXISTS

/// The operation cannot be performed on a device information element that has not been registered.
pub const E_DEVINFO_NOT_REGISTERED : HResultError = HResultError::from_constant(0x800F0208); // SPAPI_E_DEVINFO_NOT_REGISTERED

/// The device property code is invalid.
pub const E_INVALID_REG_PROPERTY : HResultError = HResultError::from_constant(0x800F0209); // SPAPI_E_INVALID_REG_PROPERTY

/// The INF from which a driver list is to be built does not exist.
pub const E_NO_INF : HResultError = HResultError::from_constant(0x800F020A); // SPAPI_E_NO_INF

/// The device instance does not exist in the hardware tree.
pub const E_NO_SUCH_DEVINST : HResultError = HResultError::from_constant(0x800F020B); // SPAPI_E_NO_SUCH_DEVINST

/// The icon representing this install class cannot be loaded.
pub const E_CANT_LOAD_CLASS_ICON : HResultError = HResultError::from_constant(0x800F020C); // SPAPI_E_CANT_LOAD_CLASS_ICON

/// The class installer registry entry is invalid.
pub const E_INVALID_CLASS_INSTALLER : HResultError = HResultError::from_constant(0x800F020D); // SPAPI_E_INVALID_CLASS_INSTALLER

/// The class installer has indicated that the default action should be performed for this installation request.
pub const E_DI_DO_DEFAULT : HResultError = HResultError::from_constant(0x800F020E); // SPAPI_E_DI_DO_DEFAULT

/// The operation does not require any files to be copied.
pub const E_DI_NOFILECOPY : HResultError = HResultError::from_constant(0x800F020F); // SPAPI_E_DI_NOFILECOPY

/// The specified hardware profile does not exist.
pub const E_INVALID_HWPROFILE : HResultError = HResultError::from_constant(0x800F0210); // SPAPI_E_INVALID_HWPROFILE

/// There is no device information element currently selected for this device information set.
pub const E_NO_DEVICE_SELECTED : HResultError = HResultError::from_constant(0x800F0211); // SPAPI_E_NO_DEVICE_SELECTED

/// The operation cannot be performed because the device information set is locked.
pub const E_DEVINFO_LIST_LOCKED : HResultError = HResultError::from_constant(0x800F0212); // SPAPI_E_DEVINFO_LIST_LOCKED

/// The operation cannot be performed because the device information element is locked.
pub const E_DEVINFO_DATA_LOCKED : HResultError = HResultError::from_constant(0x800F0213); // SPAPI_E_DEVINFO_DATA_LOCKED

/// The specified path does not contain any applicable device INFs.
pub const E_DI_BAD_PATH : HResultError = HResultError::from_constant(0x800F0214); // SPAPI_E_DI_BAD_PATH

/// No class installer parameters have been set for the device information set or element.
pub const E_NO_CLASSINSTALL_PARAMS : HResultError = HResultError::from_constant(0x800F0215); // SPAPI_E_NO_CLASSINSTALL_PARAMS

/// The operation cannot be performed because the file queue is locked.
pub const E_FILEQUEUE_LOCKED : HResultError = HResultError::from_constant(0x800F0216); // SPAPI_E_FILEQUEUE_LOCKED

/// A service installation section in this INF is invalid.
pub const E_BAD_SERVICE_INSTALLSECT : HResultError = HResultError::from_constant(0x800F0217); // SPAPI_E_BAD_SERVICE_INSTALLSECT

/// There is no class driver list for the device information element.
pub const E_NO_CLASS_DRIVER_LIST : HResultError = HResultError::from_constant(0x800F0218); // SPAPI_E_NO_CLASS_DRIVER_LIST

/// The installation failed because a function driver was not specified for this device instance.
pub const E_NO_ASSOCIATED_SERVICE : HResultError = HResultError::from_constant(0x800F0219); // SPAPI_E_NO_ASSOCIATED_SERVICE

/// There is presently no default device interface designated for this interface class.
pub const E_NO_DEFAULT_DEVICE_INTERFACE : HResultError = HResultError::from_constant(0x800F021A); // SPAPI_E_NO_DEFAULT_DEVICE_INTERFACE

/// The operation cannot be performed because the device interface is currently active.
pub const E_DEVICE_INTERFACE_ACTIVE : HResultError = HResultError::from_constant(0x800F021B); // SPAPI_E_DEVICE_INTERFACE_ACTIVE

/// The operation cannot be performed because the device interface has been removed from the system.
pub const E_DEVICE_INTERFACE_REMOVED : HResultError = HResultError::from_constant(0x800F021C); // SPAPI_E_DEVICE_INTERFACE_REMOVED

/// An interface installation section in this INF is invalid.
pub const E_BAD_INTERFACE_INSTALLSECT : HResultError = HResultError::from_constant(0x800F021D); // SPAPI_E_BAD_INTERFACE_INSTALLSECT

/// This interface class does not exist in the system.
pub const E_NO_SUCH_INTERFACE_CLASS : HResultError = HResultError::from_constant(0x800F021E); // SPAPI_E_NO_SUCH_INTERFACE_CLASS

/// The reference string supplied for this interface device is invalid.
pub const E_INVALID_REFERENCE_STRING : HResultError = HResultError::from_constant(0x800F021F); // SPAPI_E_INVALID_REFERENCE_STRING

/// The specified machine name does not conform to UNC naming conventions.
pub const E_INVALID_MACHINENAME : HResultError = HResultError::from_constant(0x800F0220); // SPAPI_E_INVALID_MACHINENAME

/// A general remote communication error occurred.
pub const E_REMOTE_COMM_FAILURE : HResultError = HResultError::from_constant(0x800F0221); // SPAPI_E_REMOTE_COMM_FAILURE

/// The machine selected for remote communication is not available at this time.
pub const E_MACHINE_UNAVAILABLE : HResultError = HResultError::from_constant(0x800F0222); // SPAPI_E_MACHINE_UNAVAILABLE

/// The Plug and Play service is not available on the remote machine.
pub const E_NO_CONFIGMGR_SERVICES : HResultError = HResultError::from_constant(0x800F0223); // SPAPI_E_NO_CONFIGMGR_SERVICES

/// The property page provider registry entry is invalid.
pub const E_INVALID_PROPPAGE_PROVIDER : HResultError = HResultError::from_constant(0x800F0224); // SPAPI_E_INVALID_PROPPAGE_PROVIDER

/// The requested device interface is not present in the system.
pub const E_NO_SUCH_DEVICE_INTERFACE : HResultError = HResultError::from_constant(0x800F0225); // SPAPI_E_NO_SUCH_DEVICE_INTERFACE

/// The device's co-installer has additional work to perform after installation is complete.
pub const E_DI_POSTPROCESSING_REQUIRED : HResultError = HResultError::from_constant(0x800F0226); // SPAPI_E_DI_POSTPROCESSING_REQUIRED

/// The device's co-installer is invalid.
pub const E_INVALID_COINSTALLER : HResultError = HResultError::from_constant(0x800F0227); // SPAPI_E_INVALID_COINSTALLER

/// There are no compatible drivers for this device.
pub const E_NO_COMPAT_DRIVERS : HResultError = HResultError::from_constant(0x800F0228); // SPAPI_E_NO_COMPAT_DRIVERS

/// There is no icon that represents this device or device type.
pub const E_NO_DEVICE_ICON : HResultError = HResultError::from_constant(0x800F0229); // SPAPI_E_NO_DEVICE_ICON

/// A logical configuration specified in this INF is invalid.
pub const E_INVALID_INF_LOGCONFIG : HResultError = HResultError::from_constant(0x800F022A); // SPAPI_E_INVALID_INF_LOGCONFIG

/// The class installer has denied the request to install or upgrade this device.
pub const E_DI_DONT_INSTALL : HResultError = HResultError::from_constant(0x800F022B); // SPAPI_E_DI_DONT_INSTALL

/// One of the filter drivers installed for this device is invalid.
pub const E_INVALID_FILTER_DRIVER : HResultError = HResultError::from_constant(0x800F022C); // SPAPI_E_INVALID_FILTER_DRIVER

/// The driver selected for this device does not support this version of Windows.
pub const E_NON_WINDOWS_NT_DRIVER : HResultError = HResultError::from_constant(0x800F022D); // SPAPI_E_NON_WINDOWS_NT_DRIVER

/// The driver selected for this device does not support Windows.
pub const E_NON_WINDOWS_DRIVER : HResultError = HResultError::from_constant(0x800F022E); // SPAPI_E_NON_WINDOWS_DRIVER

/// The third-party INF does not contain digital signature information.
pub const E_NO_CATALOG_FOR_OEM_INF : HResultError = HResultError::from_constant(0x800F022F); // SPAPI_E_NO_CATALOG_FOR_OEM_INF

/// An invalid attempt was made to use a device installation file queue for verification of digital signatures relative to other platforms.
pub const E_DEVINSTALL_QUEUE_NONNATIVE : HResultError = HResultError::from_constant(0x800F0230); // SPAPI_E_DEVINSTALL_QUEUE_NONNATIVE

/// The device cannot be disabled.
pub const E_NOT_DISABLEABLE : HResultError = HResultError::from_constant(0x800F0231); // SPAPI_E_NOT_DISABLEABLE

/// The device could not be dynamically removed.
pub const E_CANT_REMOVE_DEVINST : HResultError = HResultError::from_constant(0x800F0232); // SPAPI_E_CANT_REMOVE_DEVINST

/// Cannot copy to specified target.
pub const E_INVALID_TARGET : HResultError = HResultError::from_constant(0x800F0233); // SPAPI_E_INVALID_TARGET

/// Driver is not intended for this platform.
pub const E_DRIVER_NONNATIVE : HResultError = HResultError::from_constant(0x800F0234); // SPAPI_E_DRIVER_NONNATIVE

/// Operation not allowed in WOW64.
pub const E_IN_WOW64 : HResultError = HResultError::from_constant(0x800F0235); // SPAPI_E_IN_WOW64

/// The operation involving unsigned file copying was rolled back, so that a system restore point could be set.
pub const E_SET_SYSTEM_RESTORE_POINT : HResultError = HResultError::from_constant(0x800F0236); // SPAPI_E_SET_SYSTEM_RESTORE_POINT

/// An INF was copied into the Windows INF directory in an improper manner.
pub const E_INCORRECTLY_COPIED_INF : HResultError = HResultError::from_constant(0x800F0237); // SPAPI_E_INCORRECTLY_COPIED_INF

/// The Security Configuration Editor (SCE) APIs have been disabled on this Embedded product.
pub const E_SCE_DISABLED : HResultError = HResultError::from_constant(0x800F0238); // SPAPI_E_SCE_DISABLED

/// An unknown exception was encountered.
pub const E_UNKNOWN_EXCEPTION : HResultError = HResultError::from_constant(0x800F0239); // SPAPI_E_UNKNOWN_EXCEPTION

/// A problem was encountered when accessing the Plug and Play registry database.
pub const E_PNP_REGISTRY_ERROR : HResultError = HResultError::from_constant(0x800F023A); // SPAPI_E_PNP_REGISTRY_ERROR

/// The requested operation is not supported for a remote machine.
pub const E_REMOTE_REQUEST_UNSUPPORTED : HResultError = HResultError::from_constant(0x800F023B); // SPAPI_E_REMOTE_REQUEST_UNSUPPORTED

/// The specified file is not an installed OEM INF.
pub const E_NOT_AN_INSTALLED_OEM_INF : HResultError = HResultError::from_constant(0x800F023C); // SPAPI_E_NOT_AN_INSTALLED_OEM_INF

/// One or more devices are presently installed using the specified INF.
pub const E_INF_IN_USE_BY_DEVICES : HResultError = HResultError::from_constant(0x800F023D); // SPAPI_E_INF_IN_USE_BY_DEVICES

/// The requested device install operation is obsolete.
pub const E_DI_FUNCTION_OBSOLETE : HResultError = HResultError::from_constant(0x800F023E); // SPAPI_E_DI_FUNCTION_OBSOLETE

/// A file could not be verified because it does not have an associated catalog signed via Authenticode(tm).
pub const E_NO_AUTHENTICODE_CATALOG : HResultError = HResultError::from_constant(0x800F023F); // SPAPI_E_NO_AUTHENTICODE_CATALOG

/// Authenticode(tm) signature verification is not supported for the specified INF.
pub const E_AUTHENTICODE_DISALLOWED : HResultError = HResultError::from_constant(0x800F0240); // SPAPI_E_AUTHENTICODE_DISALLOWED

/// The INF was signed with an Authenticode(tm) catalog from a trusted publisher.
pub const E_AUTHENTICODE_TRUSTED_PUBLISHER : HResultError = HResultError::from_constant(0x800F0241); // SPAPI_E_AUTHENTICODE_TRUSTED_PUBLISHER

/// The publisher of an Authenticode(tm) signed catalog has not yet been established as trusted.
pub const E_AUTHENTICODE_TRUST_NOT_ESTABLISHED : HResultError = HResultError::from_constant(0x800F0242); // SPAPI_E_AUTHENTICODE_TRUST_NOT_ESTABLISHED

/// The publisher of an Authenticode(tm) signed catalog was not established as trusted.
pub const E_AUTHENTICODE_PUBLISHER_NOT_TRUSTED : HResultError = HResultError::from_constant(0x800F0243); // SPAPI_E_AUTHENTICODE_PUBLISHER_NOT_TRUSTED

/// The software was tested for compliance with Windows Logo requirements on a different version of Windows, and may not be compatible with this version.
pub const E_SIGNATURE_OSATTRIBUTE_MISMATCH : HResultError = HResultError::from_constant(0x800F0244); // SPAPI_E_SIGNATURE_OSATTRIBUTE_MISMATCH

/// The file may only be validated by a catalog signed via Authenticode(tm).
pub const E_ONLY_VALIDATE_VIA_AUTHENTICODE : HResultError = HResultError::from_constant(0x800F0245); // SPAPI_E_ONLY_VALIDATE_VIA_AUTHENTICODE

/// One of the installers for this device cannot perform the installation at this time.
pub const E_DEVICE_INSTALLER_NOT_READY : HResultError = HResultError::from_constant(0x800F0246); // SPAPI_E_DEVICE_INSTALLER_NOT_READY

/// A problem was encountered while attempting to add the driver to the store.
pub const E_DRIVER_STORE_ADD_FAILED : HResultError = HResultError::from_constant(0x800F0247); // SPAPI_E_DRIVER_STORE_ADD_FAILED

/// The installation of this device is forbidden by system policy. Contact your system administrator.
pub const E_DEVICE_INSTALL_BLOCKED : HResultError = HResultError::from_constant(0x800F0248); // SPAPI_E_DEVICE_INSTALL_BLOCKED

/// The installation of this driver is forbidden by system policy. Contact your system administrator.
pub const E_DRIVER_INSTALL_BLOCKED : HResultError = HResultError::from_constant(0x800F0249); // SPAPI_E_DRIVER_INSTALL_BLOCKED

/// The specified INF is the wrong type for this operation.
pub const E_WRONG_INF_TYPE : HResultError = HResultError::from_constant(0x800F024A); // SPAPI_E_WRONG_INF_TYPE

/// The hash for the file is not present in the specified catalog file. The file is likely corrupt or the victim of tampering.
pub const E_FILE_HASH_NOT_IN_CATALOG : HResultError = HResultError::from_constant(0x800F024B); // SPAPI_E_FILE_HASH_NOT_IN_CATALOG

/// A problem was encountered while attempting to delete the driver from the store.
pub const E_DRIVER_STORE_DELETE_FAILED : HResultError = HResultError::from_constant(0x800F024C); // SPAPI_E_DRIVER_STORE_DELETE_FAILED

/// An unrecoverable stack overflow was encountered.
pub const E_UNRECOVERABLE_STACK_OVERFLOW : HResultError = HResultError::from_constant(0x800F0300); // SPAPI_E_UNRECOVERABLE_STACK_OVERFLOW

/// No installed components were detected.
pub const E_ERROR_NOT_INSTALLED : HResultError = HResultError::from_constant(0x800F1000); // SPAPI_E_ERROR_NOT_INSTALLED
