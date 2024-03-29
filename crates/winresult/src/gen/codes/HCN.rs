// WARNING: this file is auto-generated by xtask gen and may be overwritten

use super::*;


/// The network was not found.
pub const E_NETWORK_NOT_FOUND : HResultError = HResultError::from_constant(0x803B0001); // HCN_E_NETWORK_NOT_FOUND

/// The endpoint was not found.
pub const E_ENDPOINT_NOT_FOUND : HResultError = HResultError::from_constant(0x803B0002); // HCN_E_ENDPOINT_NOT_FOUND

/// The network's underlying layer was not found.
pub const E_LAYER_NOT_FOUND : HResultError = HResultError::from_constant(0x803B0003); // HCN_E_LAYER_NOT_FOUND

/// The virtual switch was not found.
pub const E_SWITCH_NOT_FOUND : HResultError = HResultError::from_constant(0x803B0004); // HCN_E_SWITCH_NOT_FOUND

/// The network does not have a subnet for this endpoint.
pub const E_SUBNET_NOT_FOUND : HResultError = HResultError::from_constant(0x803B0005); // HCN_E_SUBNET_NOT_FOUND

/// An adapter was not found.
pub const E_ADAPTER_NOT_FOUND : HResultError = HResultError::from_constant(0x803B0006); // HCN_E_ADAPTER_NOT_FOUND

/// The switch-port was not found.
pub const E_PORT_NOT_FOUND : HResultError = HResultError::from_constant(0x803B0007); // HCN_E_PORT_NOT_FOUND

/// An expected policy was not found.
pub const E_POLICY_NOT_FOUND : HResultError = HResultError::from_constant(0x803B0008); // HCN_E_POLICY_NOT_FOUND

/// A required VFP port setting was not found.
pub const E_VFP_PORTSETTING_NOT_FOUND : HResultError = HResultError::from_constant(0x803B0009); // HCN_E_VFP_PORTSETTING_NOT_FOUND

/// The provided network configuration is invalid or missing parameters.
pub const E_INVALID_NETWORK : HResultError = HResultError::from_constant(0x803B000A); // HCN_E_INVALID_NETWORK

/// Invalid network type.
pub const E_INVALID_NETWORK_TYPE : HResultError = HResultError::from_constant(0x803B000B); // HCN_E_INVALID_NETWORK_TYPE

/// The provided endpoint configuration is invalid or missing parameters.
pub const E_INVALID_ENDPOINT : HResultError = HResultError::from_constant(0x803B000C); // HCN_E_INVALID_ENDPOINT

/// The provided policy configuration is invalid or missing parameters.
pub const E_INVALID_POLICY : HResultError = HResultError::from_constant(0x803B000D); // HCN_E_INVALID_POLICY

/// Invalid policy type.
pub const E_INVALID_POLICY_TYPE : HResultError = HResultError::from_constant(0x803B000E); // HCN_E_INVALID_POLICY_TYPE

/// This requested operation is invalid for a remote endpoint.
pub const E_INVALID_REMOTE_ENDPOINT_OPERATION : HResultError = HResultError::from_constant(0x803B000F); // HCN_E_INVALID_REMOTE_ENDPOINT_OPERATION

/// A network with this name already exists.
pub const E_NETWORK_ALREADY_EXISTS : HResultError = HResultError::from_constant(0x803B0010); // HCN_E_NETWORK_ALREADY_EXISTS

/// A network with this name already exists.
pub const E_LAYER_ALREADY_EXISTS : HResultError = HResultError::from_constant(0x803B0011); // HCN_E_LAYER_ALREADY_EXISTS

/// Policy information already exists on this object.
pub const E_POLICY_ALREADY_EXISTS : HResultError = HResultError::from_constant(0x803B0012); // HCN_E_POLICY_ALREADY_EXISTS

/// The specified port already exists.
pub const E_PORT_ALREADY_EXISTS : HResultError = HResultError::from_constant(0x803B0013); // HCN_E_PORT_ALREADY_EXISTS

/// This endpoint is already attached to the switch.
pub const E_ENDPOINT_ALREADY_ATTACHED : HResultError = HResultError::from_constant(0x803B0014); // HCN_E_ENDPOINT_ALREADY_ATTACHED

/// The specified request is unsupported.
pub const E_REQUEST_UNSUPPORTED : HResultError = HResultError::from_constant(0x803B0015); // HCN_E_REQUEST_UNSUPPORTED

/// Port mapping is not supported on the given network.
pub const E_MAPPING_NOT_SUPPORTED : HResultError = HResultError::from_constant(0x803B0016); // HCN_E_MAPPING_NOT_SUPPORTED

/// There was an operation attempted on a degraded object.
pub const E_DEGRADED_OPERATION : HResultError = HResultError::from_constant(0x803B0017); // HCN_E_DEGRADED_OPERATION

/// Cannot modify a switch shared by multiple networks.
pub const E_SHARED_SWITCH_MODIFICATION : HResultError = HResultError::from_constant(0x803B0018); // HCN_E_SHARED_SWITCH_MODIFICATION

/// Failed to interpret a parameter as a GUID.
pub const E_GUID_CONVERSION_FAILURE : HResultError = HResultError::from_constant(0x803B0019); // HCN_E_GUID_CONVERSION_FAILURE

/// Failed to process registry key.
pub const E_REGKEY_FAILURE : HResultError = HResultError::from_constant(0x803B001A); // HCN_E_REGKEY_FAILURE

/// Invalid JSON document string.
pub const E_INVALID_JSON : HResultError = HResultError::from_constant(0x803B001B); // HCN_E_INVALID_JSON

/// The reference is invalid in the JSON document.
pub const E_INVALID_JSON_REFERENCE : HResultError = HResultError::from_constant(0x803B001C); // HCN_E_INVALID_JSON_REFERENCE

/// Endpoint sharing is disabled.
pub const E_ENDPOINT_SHARING_DISABLED : HResultError = HResultError::from_constant(0x803B001D); // HCN_E_ENDPOINT_SHARING_DISABLED

/// IP address is either invalid or not part of any configured subnet(s).
pub const E_INVALID_IP : HResultError = HResultError::from_constant(0x803B001E); // HCN_E_INVALID_IP

/// The specified switch extension does not exist on this switch.
pub const E_SWITCH_EXTENSION_NOT_FOUND : HResultError = HResultError::from_constant(0x803B001F); // HCN_E_SWITCH_EXTENSION_NOT_FOUND

/// Operation cannot be performed while service is stopping.
pub const E_MANAGER_STOPPED : HResultError = HResultError::from_constant(0x803B0020); // HCN_E_MANAGER_STOPPED

/// Internet Connection Sharing service (SharedAccess) is disabled and cannot be started
pub const E_ICS_DISABLED : HResultError = HResultError::from_constant(0x803B002A); // HCN_E_ICS_DISABLED

/// This requested operation is invalid as endpoint is already part of a network namespace.
pub const E_ENDPOINT_NAMESPACE_ALREADY_EXISTS : HResultError = HResultError::from_constant(0x803B002B); // HCN_E_ENDPOINT_NAMESPACE_ALREADY_EXISTS

/// The specified entity cannot be removed while it still has references.
pub const E_ENTITY_HAS_REFERENCES : HResultError = HResultError::from_constant(0x803B002C); // HCN_E_ENTITY_HAS_REFERENCES

/// The internal port must exist and cannot be zero.
pub const E_INVALID_INTERNAL_PORT : HResultError = HResultError::from_constant(0x803B002D); // HCN_E_INVALID_INTERNAL_PORT

/// The requested operation for attach namespace failed.
pub const E_NAMESPACE_ATTACH_FAILED : HResultError = HResultError::from_constant(0x803B002E); // HCN_E_NAMESPACE_ATTACH_FAILED

/// An address provided is invalid or reserved.
pub const E_ADDR_INVALID_OR_RESERVED : HResultError = HResultError::from_constant(0x803B002F); // HCN_E_ADDR_INVALID_OR_RESERVED

/// The prefix provided is invalid.
pub const E_INVALID_PREFIX : HResultError = HResultError::from_constant(0x803B0030); // HCN_E_INVALID_PREFIX

/// A call was performed against an object that was torn down.
pub const E_OBJECT_USED_AFTER_UNLOAD : HResultError = HResultError::from_constant(0x803B0031); // HCN_E_OBJECT_USED_AFTER_UNLOAD

/// The provided subnet configuration is invalid or missing parameters.
pub const E_INVALID_SUBNET : HResultError = HResultError::from_constant(0x803B0032); // HCN_E_INVALID_SUBNET

/// The provided IP subnet configuration is invalid or missing parameters.
pub const E_INVALID_IP_SUBNET : HResultError = HResultError::from_constant(0x803B0033); // HCN_E_INVALID_IP_SUBNET

/// The endpoint must be attached to complete the operation.
pub const E_ENDPOINT_NOT_ATTACHED : HResultError = HResultError::from_constant(0x803B0034); // HCN_E_ENDPOINT_NOT_ATTACHED

/// The endpoint must be local to complete the operation.
pub const E_ENDPOINT_NOT_LOCAL : HResultError = HResultError::from_constant(0x803B0035); // HCN_E_ENDPOINT_NOT_LOCAL

/// A network of this type can not be created because VFP is not available.
pub const E_VFP_NOT_ALLOWED : HResultError = HResultError::from_constant(0x803B0037); // HCN_E_VFP_NOT_ALLOWED
