// WARNING: this file is auto-generated by xtask gen and may be overwritten

use super::*;


/// The cluster software is shutting down.
pub const SHUTTING_DOWN : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(5022); // ERROR_CLUSTER_SHUTTING_DOWN

/// The cluster node is not valid.
pub const INVALID_NODE : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(5039); // ERROR_CLUSTER_INVALID_NODE

/// The cluster node already exists.
pub const NODE_EXISTS : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(5040); // ERROR_CLUSTER_NODE_EXISTS

/// A node is in the process of joining the cluster.
pub const JOIN_IN_PROGRESS : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(5041); // ERROR_CLUSTER_JOIN_IN_PROGRESS

/// The cluster node was not found.
pub const NODE_NOT_FOUND : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(5042); // ERROR_CLUSTER_NODE_NOT_FOUND

/// The cluster local node information was not found.
pub const LOCAL_NODE_NOT_FOUND : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(5043); // ERROR_CLUSTER_LOCAL_NODE_NOT_FOUND

/// The cluster network already exists.
pub const NETWORK_EXISTS : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(5044); // ERROR_CLUSTER_NETWORK_EXISTS

/// The cluster network was not found.
pub const NETWORK_NOT_FOUND : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(5045); // ERROR_CLUSTER_NETWORK_NOT_FOUND

/// The cluster network interface already exists.
pub const NETINTERFACE_EXISTS : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(5046); // ERROR_CLUSTER_NETINTERFACE_EXISTS

/// The cluster network interface was not found.
pub const NETINTERFACE_NOT_FOUND : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(5047); // ERROR_CLUSTER_NETINTERFACE_NOT_FOUND

/// The cluster request is not valid for this object.
pub const INVALID_REQUEST : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(5048); // ERROR_CLUSTER_INVALID_REQUEST

/// The cluster network provider is not valid.
pub const INVALID_NETWORK_PROVIDER : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(5049); // ERROR_CLUSTER_INVALID_NETWORK_PROVIDER

/// The cluster node is down.
pub const NODE_DOWN : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(5050); // ERROR_CLUSTER_NODE_DOWN

/// The cluster node is not reachable.
pub const NODE_UNREACHABLE : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(5051); // ERROR_CLUSTER_NODE_UNREACHABLE

/// The cluster node is not a member of the cluster.
pub const NODE_NOT_MEMBER : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(5052); // ERROR_CLUSTER_NODE_NOT_MEMBER

/// A cluster join operation is not in progress.
pub const JOIN_NOT_IN_PROGRESS : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(5053); // ERROR_CLUSTER_JOIN_NOT_IN_PROGRESS

/// The cluster network is not valid.
pub const INVALID_NETWORK : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(5054); // ERROR_CLUSTER_INVALID_NETWORK

/// The cluster node is up.
pub const NODE_UP : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(5056); // ERROR_CLUSTER_NODE_UP

/// The cluster IP address is already in use.
pub const IPADDR_IN_USE : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(5057); // ERROR_CLUSTER_IPADDR_IN_USE

/// The cluster node is not paused.
pub const NODE_NOT_PAUSED : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(5058); // ERROR_CLUSTER_NODE_NOT_PAUSED

/// No cluster security context is available.
pub const NO_SECURITY_CONTEXT : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(5059); // ERROR_CLUSTER_NO_SECURITY_CONTEXT

/// The cluster network is not configured for internal cluster communication.
pub const NETWORK_NOT_INTERNAL : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(5060); // ERROR_CLUSTER_NETWORK_NOT_INTERNAL

/// The cluster node is already up.
pub const NODE_ALREADY_UP : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(5061); // ERROR_CLUSTER_NODE_ALREADY_UP

/// The cluster node is already down.
pub const NODE_ALREADY_DOWN : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(5062); // ERROR_CLUSTER_NODE_ALREADY_DOWN

/// The cluster network is already online.
pub const NETWORK_ALREADY_ONLINE : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(5063); // ERROR_CLUSTER_NETWORK_ALREADY_ONLINE

/// The cluster network is already offline.
pub const NETWORK_ALREADY_OFFLINE : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(5064); // ERROR_CLUSTER_NETWORK_ALREADY_OFFLINE

/// The cluster node is already a member of the cluster.
pub const NODE_ALREADY_MEMBER : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(5065); // ERROR_CLUSTER_NODE_ALREADY_MEMBER

/// The cluster network is the only one configured for internal cluster communication between two or more active cluster nodes. The internal communication capability cannot be removed from the network.
pub const LAST_INTERNAL_NETWORK : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(5066); // ERROR_CLUSTER_LAST_INTERNAL_NETWORK

/// One or more cluster resources depend on the network to provide service to clients. The client access capability cannot be removed from the network.
pub const NETWORK_HAS_DEPENDENTS : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(5067); // ERROR_CLUSTER_NETWORK_HAS_DEPENDENTS

/// The cluster node is paused.
pub const NODE_PAUSED : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(5070); // ERROR_CLUSTER_NODE_PAUSED

/// The cluster node is not ready to perform the requested operation.
pub const NODE_NOT_READY : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(5072); // ERROR_CLUSTER_NODE_NOT_READY

/// The cluster node is shutting down.
pub const NODE_SHUTTING_DOWN : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(5073); // ERROR_CLUSTER_NODE_SHUTTING_DOWN

/// The cluster join operation was aborted.
pub const JOIN_ABORTED : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(5074); // ERROR_CLUSTER_JOIN_ABORTED

/// The node failed to join the cluster because the joining node and other nodes in the cluster have incompatible operating system versions. To get more information about operating system versions of the cluster, run the Validate a Configuration Wizard or the Test-Cluster Windows PowerShell cmdlet.
pub const INCOMPATIBLE_VERSIONS : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(5075); // ERROR_CLUSTER_INCOMPATIBLE_VERSIONS

/// This resource cannot be created because the cluster has reached the limit on the number of resources it can monitor.
pub const MAXNUM_OF_RESOURCES_EXCEEDED : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(5076); // ERROR_CLUSTER_MAXNUM_OF_RESOURCES_EXCEEDED

/// The system configuration changed during the cluster join or form operation. The join or form operation was aborted.
pub const SYSTEM_CONFIG_CHANGED : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(5077); // ERROR_CLUSTER_SYSTEM_CONFIG_CHANGED

/// The specified resource type was not found.
pub const RESOURCE_TYPE_NOT_FOUND : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(5078); // ERROR_CLUSTER_RESOURCE_TYPE_NOT_FOUND

/// The specified node does not support a resource of this type. This may be due to version inconsistencies or due to the absence of the resource DLL on this node.
pub const RESTYPE_NOT_SUPPORTED : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(5079); // ERROR_CLUSTER_RESTYPE_NOT_SUPPORTED

/// The specified resource name is not supported by this resource DLL. This may be due to a bad (or changed) name supplied to the resource DLL.
pub const RESNAME_NOT_FOUND : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(5080); // ERROR_CLUSTER_RESNAME_NOT_FOUND

/// No authentication package could be registered with the RPC server.
pub const NO_RPC_PACKAGES_REGISTERED : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(5081); // ERROR_CLUSTER_NO_RPC_PACKAGES_REGISTERED

/// You cannot bring the group online because the owner of the group is not in the preferred list for the group. To change the owner node for the group, move the group.
pub const OWNER_NOT_IN_PREFLIST : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(5082); // ERROR_CLUSTER_OWNER_NOT_IN_PREFLIST

/// The join operation failed because the cluster database sequence number has changed or is incompatible with the locker node. This may happen during a join operation if the cluster database was changing during the join.
pub const DATABASE_SEQMISMATCH : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(5083); // ERROR_CLUSTER_DATABASE_SEQMISMATCH

/// A non locker code got a request to reserve the lock for making global updates.
pub const GUM_NOT_LOCKER : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(5085); // ERROR_CLUSTER_GUM_NOT_LOCKER

/// A DFS root already exists in this cluster node.
pub const NODE_ALREADY_HAS_DFS_ROOT : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(5088); // ERROR_CLUSTER_NODE_ALREADY_HAS_DFS_ROOT

/// An operation was attempted that is incompatible with the current membership state of the node.
pub const MEMBERSHIP_INVALID_STATE : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(5890); // ERROR_CLUSTER_MEMBERSHIP_INVALID_STATE

/// The quorum resource does not contain the quorum log.
pub const QUORUMLOG_NOT_FOUND : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(5891); // ERROR_CLUSTER_QUORUMLOG_NOT_FOUND

/// The membership engine requested shutdown of the cluster service on this node.
pub const MEMBERSHIP_HALT : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(5892); // ERROR_CLUSTER_MEMBERSHIP_HALT

/// The join operation failed because the cluster instance ID of the joining node does not match the cluster instance ID of the sponsor node.
pub const INSTANCE_ID_MISMATCH : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(5893); // ERROR_CLUSTER_INSTANCE_ID_MISMATCH

/// A matching cluster network for the specified IP address could not be found.
pub const NETWORK_NOT_FOUND_FOR_IP : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(5894); // ERROR_CLUSTER_NETWORK_NOT_FOUND_FOR_IP

/// The actual data type of the property did not match the expected data type of the property.
pub const PROPERTY_DATA_TYPE_MISMATCH : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(5895); // ERROR_CLUSTER_PROPERTY_DATA_TYPE_MISMATCH

/// The cluster node was evicted from the cluster successfully, but the node was not cleaned up. To determine what cleanup steps failed and how to recover, see the Failover Clustering application event log using Event Viewer.
pub const EVICT_WITHOUT_CLEANUP : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(5896); // ERROR_CLUSTER_EVICT_WITHOUT_CLEANUP

/// Two or more parameter values specified for a resource's properties are in conflict.
pub const PARAMETER_MISMATCH : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(5897); // ERROR_CLUSTER_PARAMETER_MISMATCH

/// This computer cannot be made a member of a cluster because it does not have the correct version of Windows installed.
pub const WRONG_OS_VERSION : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(5899); // ERROR_CLUSTER_WRONG_OS_VERSION

/// A cluster cannot be created with the specified cluster name because that cluster name is already in use. Specify a different name for the cluster.
pub const CANT_CREATE_DUP_CLUSTER_NAME : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(5900); // ERROR_CLUSTER_CANT_CREATE_DUP_CLUSTER_NAME

/// One or more nodes in the cluster are running a version of Windows that does not support this operation.
pub const OLD_VERSION : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(5904); // ERROR_CLUSTER_OLD_VERSION

/// The name of the corresponding computer account doesn't match the Network Name for this resource.
pub const MISMATCHED_COMPUTER_ACCT_NAME : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(5905); // ERROR_CLUSTER_MISMATCHED_COMPUTER_ACCT_NAME

/// No network adapters are available.
pub const NO_NET_ADAPTERS : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(5906); // ERROR_CLUSTER_NO_NET_ADAPTERS

/// The cluster node has been poisoned.
pub const POISONED : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(5907); // ERROR_CLUSTER_POISONED

/// The group is unable to accept the request since it is moving to another node.
pub const GROUP_MOVING : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(5908); // ERROR_CLUSTER_GROUP_MOVING

/// The resource type cannot accept the request since is too busy performing another operation.
pub const RESOURCE_TYPE_BUSY : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(5909); // ERROR_CLUSTER_RESOURCE_TYPE_BUSY

/// An internal cluster error occurred. A call to an invalid function was attempted.
pub const INTERNAL_INVALID_FUNCTION : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(5912); // ERROR_CLUSTER_INTERNAL_INVALID_FUNCTION

/// A parameter value is out of acceptable range.
pub const PARAMETER_OUT_OF_BOUNDS : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(5913); // ERROR_CLUSTER_PARAMETER_OUT_OF_BOUNDS

/// A network error occurred while sending data to another node in the cluster. The number of bytes transmitted was less than required.
pub const PARTIAL_SEND : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(5914); // ERROR_CLUSTER_PARTIAL_SEND

/// An invalid cluster registry operation was attempted.
pub const REGISTRY_INVALID_FUNCTION : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(5915); // ERROR_CLUSTER_REGISTRY_INVALID_FUNCTION

/// An input string of characters is not properly terminated.
pub const INVALID_STRING_TERMINATION : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(5916); // ERROR_CLUSTER_INVALID_STRING_TERMINATION

/// An input string of characters is not in a valid format for the data it represents.
pub const INVALID_STRING_FORMAT : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(5917); // ERROR_CLUSTER_INVALID_STRING_FORMAT

/// An internal cluster error occurred. A cluster database transaction was attempted while a transaction was already in progress.
pub const DATABASE_TRANSACTION_IN_PROGRESS : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(5918); // ERROR_CLUSTER_DATABASE_TRANSACTION_IN_PROGRESS

/// An internal cluster error occurred. There was an attempt to commit a cluster database transaction while no transaction was in progress.
pub const DATABASE_TRANSACTION_NOT_IN_PROGRESS : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(5919); // ERROR_CLUSTER_DATABASE_TRANSACTION_NOT_IN_PROGRESS

/// An internal cluster error occurred. Data was not properly initialized.
pub const NULL_DATA : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(5920); // ERROR_CLUSTER_NULL_DATA

/// An error occurred while reading from a stream of data. An unexpected number of bytes was returned.
pub const PARTIAL_READ : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(5921); // ERROR_CLUSTER_PARTIAL_READ

/// An error occurred while writing to a stream of data. The required number of bytes could not be written.
pub const PARTIAL_WRITE : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(5922); // ERROR_CLUSTER_PARTIAL_WRITE

/// An error occurred while deserializing a stream of cluster data.
pub const CANT_DESERIALIZE_DATA : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(5923); // ERROR_CLUSTER_CANT_DESERIALIZE_DATA

/// A quorum of cluster nodes was not present to form a cluster.
pub const NO_QUORUM : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(5925); // ERROR_CLUSTER_NO_QUORUM

/// The cluster network is not valid for an IPv6 Address resource, or it does not match the configured address.
pub const INVALID_IPV6_NETWORK : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(5926); // ERROR_CLUSTER_INVALID_IPV6_NETWORK

/// The cluster network is not valid for an IPv6 Tunnel resource. Check the configuration of the IP Address resource on which the IPv6 Tunnel resource depends.
pub const INVALID_IPV6_TUNNEL_NETWORK : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(5927); // ERROR_CLUSTER_INVALID_IPV6_TUNNEL_NETWORK

/// The RHS process failed to initialize.
pub const RHS_FAILED_INITIALIZATION : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(5931); // ERROR_CLUSTER_RHS_FAILED_INITIALIZATION

/// The Failover Clustering feature is not installed on this node.
pub const NOT_INSTALLED : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(5932); // ERROR_CLUSTER_NOT_INSTALLED

/// The resources must be online on the same node for this operation
pub const RESOURCES_MUST_BE_ONLINE_ON_THE_SAME_NODE : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(5933); // ERROR_CLUSTER_RESOURCES_MUST_BE_ONLINE_ON_THE_SAME_NODE

/// A new node can not be added since this cluster is already at its maximum number of nodes.
pub const MAX_NODES_IN_CLUSTER : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(5934); // ERROR_CLUSTER_MAX_NODES_IN_CLUSTER

/// This cluster can not be created since the specified number of nodes exceeds the maximum allowed limit.
pub const TOO_MANY_NODES : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(5935); // ERROR_CLUSTER_TOO_MANY_NODES

/// An attempt to use the specified cluster name failed because an enabled computer object with the given name already exists in the domain.
pub const OBJECT_ALREADY_USED : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(5936); // ERROR_CLUSTER_OBJECT_ALREADY_USED

/// Eviction of this node is invalid at this time. Due to quorum requirements node eviction will result in cluster shutdown.
/// If it is the last node in the cluster, destroy cluster command should be used.
pub const EVICT_INVALID_REQUEST : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(5939); // ERROR_CLUSTER_EVICT_INVALID_REQUEST

/// Only one instance of this resource type is allowed in the cluster.
pub const SINGLETON_RESOURCE : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(5940); // ERROR_CLUSTER_SINGLETON_RESOURCE

/// Only one instance of this resource type is allowed per resource group.
pub const GROUP_SINGLETON_RESOURCE : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(5941); // ERROR_CLUSTER_GROUP_SINGLETON_RESOURCE

/// The resource failed to come online due to the failure of one or more provider resources.
pub const RESOURCE_PROVIDER_FAILED : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(5942); // ERROR_CLUSTER_RESOURCE_PROVIDER_FAILED

/// The resource has indicated that it cannot come online on any node.
pub const RESOURCE_CONFIGURATION_ERROR : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(5943); // ERROR_CLUSTER_RESOURCE_CONFIGURATION_ERROR

/// The current operation cannot be performed on this group at this time.
pub const GROUP_BUSY : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(5944); // ERROR_CLUSTER_GROUP_BUSY

/// The directory or file is not located on a cluster shared volume.
pub const NOT_SHARED_VOLUME : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(5945); // ERROR_CLUSTER_NOT_SHARED_VOLUME

/// The Security Descriptor does not meet the requirements for a cluster.
pub const INVALID_SECURITY_DESCRIPTOR : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(5946); // ERROR_CLUSTER_INVALID_SECURITY_DESCRIPTOR

/// There is one or more shared volumes resources configured in the cluster.
/// Those resources must be moved to available storage in order for operation to succeed.
pub const SHARED_VOLUMES_IN_USE : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(5947); // ERROR_CLUSTER_SHARED_VOLUMES_IN_USE

/// This group or resource cannot be directly manipulated.
/// Use shared volume APIs to perform desired operation.
pub const USE_SHARED_VOLUMES_API : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(5948); // ERROR_CLUSTER_USE_SHARED_VOLUMES_API

/// Back up is in progress. Please wait for backup completion before trying this operation again.
pub const BACKUP_IN_PROGRESS : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(5949); // ERROR_CLUSTER_BACKUP_IN_PROGRESS

/// The cluster watchdog is terminating.
pub const WATCHDOG_TERMINATING : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(5952); // ERROR_CLUSTER_WATCHDOG_TERMINATING

/// A resource vetoed a move between two nodes because they are incompatible.
pub const RESOURCE_VETOED_MOVE_INCOMPATIBLE_NODES : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(5953); // ERROR_CLUSTER_RESOURCE_VETOED_MOVE_INCOMPATIBLE_NODES

/// The request is invalid either because node weight cannot be changed while the cluster is in disk-only quorum mode, or because changing the node weight would violate the minimum cluster quorum requirements.
pub const INVALID_NODE_WEIGHT : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(5954); // ERROR_CLUSTER_INVALID_NODE_WEIGHT

/// The resource vetoed the call.
pub const RESOURCE_VETOED_CALL : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(5955); // ERROR_CLUSTER_RESOURCE_VETOED_CALL

/// A resource vetoed a move between two nodes because the destination currently does not have enough resources to complete the operation.
pub const RESOURCE_VETOED_MOVE_NOT_ENOUGH_RESOURCES_ON_DESTINATION : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(5957); // ERROR_CLUSTER_RESOURCE_VETOED_MOVE_NOT_ENOUGH_RESOURCES_ON_DESTINATION

/// A resource vetoed a move between two nodes because the source currently does not have enough resources to complete the operation.
pub const RESOURCE_VETOED_MOVE_NOT_ENOUGH_RESOURCES_ON_SOURCE : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(5958); // ERROR_CLUSTER_RESOURCE_VETOED_MOVE_NOT_ENOUGH_RESOURCES_ON_SOURCE

/// The requested operation can not be completed because the group is queued for an operation.
pub const GROUP_QUEUED : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(5959); // ERROR_CLUSTER_GROUP_QUEUED

/// The requested operation can not be completed because a resource has locked status.
pub const RESOURCE_LOCKED_STATUS : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(5960); // ERROR_CLUSTER_RESOURCE_LOCKED_STATUS

/// The resource cannot move to another node because a cluster shared volume vetoed the operation.
pub const SHARED_VOLUME_FAILOVER_NOT_ALLOWED : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(5961); // ERROR_CLUSTER_SHARED_VOLUME_FAILOVER_NOT_ALLOWED

/// A node drain is already in progress.
pub const NODE_DRAIN_IN_PROGRESS : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(5962); // ERROR_CLUSTER_NODE_DRAIN_IN_PROGRESS

/// Clustered storage is not connected to the node.
pub const DISK_NOT_CONNECTED : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(5963); // ERROR_CLUSTER_DISK_NOT_CONNECTED

/// CSVFS failed operation as volume is in redirected mode.
pub const SHARED_VOLUME_REDIRECTED : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(5966); // ERROR_CLUSTER_SHARED_VOLUME_REDIRECTED

/// CSVFS failed operation as volume is not in redirected mode.
pub const SHARED_VOLUME_NOT_REDIRECTED : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(5967); // ERROR_CLUSTER_SHARED_VOLUME_NOT_REDIRECTED

/// Cluster properties cannot be returned at this time.
pub const CANNOT_RETURN_PROPERTIES : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(5968); // ERROR_CLUSTER_CANNOT_RETURN_PROPERTIES

/// The clustered disk resource contains software snapshot diff area that are not supported for Cluster Shared Volumes.
pub const RESOURCE_CONTAINS_UNSUPPORTED_DIFF_AREA_FOR_SHARED_VOLUMES : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(5969); // ERROR_CLUSTER_RESOURCE_CONTAINS_UNSUPPORTED_DIFF_AREA_FOR_SHARED_VOLUMES

/// The operation cannot be completed because the resource is in maintenance mode.
pub const RESOURCE_IS_IN_MAINTENANCE_MODE : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(5970); // ERROR_CLUSTER_RESOURCE_IS_IN_MAINTENANCE_MODE

/// The operation cannot be completed because of cluster affinity conflicts
pub const AFFINITY_CONFLICT : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(5971); // ERROR_CLUSTER_AFFINITY_CONFLICT

/// The operation cannot be completed because the resource is a replica virtual machine.
pub const RESOURCE_IS_REPLICA_VIRTUAL_MACHINE : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(5972); // ERROR_CLUSTER_RESOURCE_IS_REPLICA_VIRTUAL_MACHINE

/// The Cluster Functional Level could not be increased because not all nodes in the cluster support the updated version.
pub const UPGRADE_INCOMPATIBLE_VERSIONS : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(5973); // ERROR_CLUSTER_UPGRADE_INCOMPATIBLE_VERSIONS

/// Updating the cluster functional level failed because the cluster is running in fix quorum mode.
/// Start additional nodes which are members of the cluster until the cluster reaches quorum and the cluster will automatically
/// switch out of fix quorum mode, or stop and restart the cluster without the FixQuorum switch. Once the cluster is out
/// of fix quorum mode retry the Update-ClusterFunctionalLevel PowerShell cmdlet to update the cluster functional level.
pub const UPGRADE_FIX_QUORUM_NOT_SUPPORTED : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(5974); // ERROR_CLUSTER_UPGRADE_FIX_QUORUM_NOT_SUPPORTED

/// The cluster functional level has been successfully updated but not all features are available yet. Restart the cluster by
/// using the Stop-Cluster PowerShell cmdlet followed by the Start-Cluster PowerShell cmdlet and all cluster features will
/// be available.
pub const UPGRADE_RESTART_REQUIRED : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(5975); // ERROR_CLUSTER_UPGRADE_RESTART_REQUIRED

/// The cluster is currently performing a version upgrade.
pub const UPGRADE_IN_PROGRESS : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(5976); // ERROR_CLUSTER_UPGRADE_IN_PROGRESS

/// The cluster did not successfully complete the version upgrade.
pub const UPGRADE_INCOMPLETE : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(5977); // ERROR_CLUSTER_UPGRADE_INCOMPLETE

/// The cluster node is in grace period.
pub const NODE_IN_GRACE_PERIOD : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(5978); // ERROR_CLUSTER_NODE_IN_GRACE_PERIOD

/// The operation has failed because CSV volume was not able to recover in time specified on this file object.
pub const CSV_IO_PAUSE_TIMEOUT : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(5979); // ERROR_CLUSTER_CSV_IO_PAUSE_TIMEOUT

/// The operation failed because the requested cluster resource is currently unmonitored.
pub const RESOURCE_NOT_MONITORED : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(5981); // ERROR_CLUSTER_RESOURCE_NOT_MONITORED

/// The operation failed because a resource does not support running in an unmonitored state.
pub const RESOURCE_DOES_NOT_SUPPORT_UNMONITORED : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(5982); // ERROR_CLUSTER_RESOURCE_DOES_NOT_SUPPORT_UNMONITORED

/// The operation cannot be completed because a resource participates in replication.
pub const RESOURCE_IS_REPLICATED : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(5983); // ERROR_CLUSTER_RESOURCE_IS_REPLICATED

/// The operation failed because the requested cluster node has been isolated
pub const NODE_ISOLATED : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(5984); // ERROR_CLUSTER_NODE_ISOLATED

/// The operation failed because the requested cluster node has been quarantined
pub const NODE_QUARANTINED : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(5985); // ERROR_CLUSTER_NODE_QUARANTINED

/// The operation failed because the specified database update condition was not met
pub const DATABASE_UPDATE_CONDITION_FAILED : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(5986); // ERROR_CLUSTER_DATABASE_UPDATE_CONDITION_FAILED

/// A clustered space is in a degraded condition and the requested action cannot be completed at this time.
pub const SPACE_DEGRADED : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(5987); // ERROR_CLUSTER_SPACE_DEGRADED

/// The operation failed because token delegation for this control is not supported.
pub const TOKEN_DELEGATION_NOT_SUPPORTED : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(5988); // ERROR_CLUSTER_TOKEN_DELEGATION_NOT_SUPPORTED

/// The operation has failed because CSV has invalidated this file object.
pub const CSV_INVALID_HANDLE : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(5989); // ERROR_CLUSTER_CSV_INVALID_HANDLE

/// This operation is supported only on the CSV coordinator node.
pub const CSV_SUPPORTED_ONLY_ON_COORDINATOR : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(5990); // ERROR_CLUSTER_CSV_SUPPORTED_ONLY_ON_COORDINATOR

/// The specified parent fault domain is not found.
pub const FAULT_DOMAIN_PARENT_NOT_FOUND : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(5994); // ERROR_CLUSTER_FAULT_DOMAIN_PARENT_NOT_FOUND

/// The fault domain cannot be a child of the parent specified.
pub const FAULT_DOMAIN_INVALID_HIERARCHY : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(5995); // ERROR_CLUSTER_FAULT_DOMAIN_INVALID_HIERARCHY

/// Storage Spaces Direct has rejected the proposed fault domain changes because it impacts the fault tolerance of the storage.
pub const FAULT_DOMAIN_FAILED_S2D_VALIDATION : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(5996); // ERROR_CLUSTER_FAULT_DOMAIN_FAILED_S2D_VALIDATION

/// Storage Spaces Direct has rejected the proposed fault domain changes because it reduces the storage connected to the system.
pub const FAULT_DOMAIN_S2D_CONNECTIVITY_LOSS : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(5997); // ERROR_CLUSTER_FAULT_DOMAIN_S2D_CONNECTIVITY_LOSS

/// Cluster infrastructure file server creation failed because a valid non-empty file server name was not provided.
pub const INVALID_INFRASTRUCTURE_FILESERVER_NAME : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(5998); // ERROR_CLUSTER_INVALID_INFRASTRUCTURE_FILESERVER_NAME

/// The object cannot be deleted from the local cluster because it is registered with the cluster set.
pub const OBJECT_IS_CLUSTER_SET_VM : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(6250); // ERROR_CLUSTER_OBJECT_IS_CLUSTER_SET_VM
