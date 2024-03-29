// WARNING: this file is auto-generated by xtask gen and may be overwritten

use super::*;


/// DNS server unable to interpret format.
pub const ERROR_RCODE_FORMAT_ERROR : ErrorCode = ErrorCode::from_constant(9001); // DNS_ERROR_RCODE_FORMAT_ERROR

/// DNS server failure.
pub const ERROR_RCODE_SERVER_FAILURE : ErrorCode = ErrorCode::from_constant(9002); // DNS_ERROR_RCODE_SERVER_FAILURE

/// DNS name does not exist.
pub const ERROR_RCODE_NAME_ERROR : ErrorCode = ErrorCode::from_constant(9003); // DNS_ERROR_RCODE_NAME_ERROR

/// DNS request not supported by name server.
pub const ERROR_RCODE_NOT_IMPLEMENTED : ErrorCode = ErrorCode::from_constant(9004); // DNS_ERROR_RCODE_NOT_IMPLEMENTED

/// DNS operation refused.
pub const ERROR_RCODE_REFUSED : ErrorCode = ErrorCode::from_constant(9005); // DNS_ERROR_RCODE_REFUSED

/// DNS name that ought not exist, does exist.
pub const ERROR_RCODE_YXDOMAIN : ErrorCode = ErrorCode::from_constant(9006); // DNS_ERROR_RCODE_YXDOMAIN

/// DNS RR set that ought not exist, does exist.
pub const ERROR_RCODE_YXRRSET : ErrorCode = ErrorCode::from_constant(9007); // DNS_ERROR_RCODE_YXRRSET

/// DNS RR set that ought to exist, does not exist.
pub const ERROR_RCODE_NXRRSET : ErrorCode = ErrorCode::from_constant(9008); // DNS_ERROR_RCODE_NXRRSET

/// DNS server not authoritative for zone.
pub const ERROR_RCODE_NOTAUTH : ErrorCode = ErrorCode::from_constant(9009); // DNS_ERROR_RCODE_NOTAUTH

/// DNS name in update or prereq is not in zone.
pub const ERROR_RCODE_NOTZONE : ErrorCode = ErrorCode::from_constant(9010); // DNS_ERROR_RCODE_NOTZONE

/// DNS signature failed to verify.
pub const ERROR_RCODE_BADSIG : ErrorCode = ErrorCode::from_constant(9016); // DNS_ERROR_RCODE_BADSIG

/// DNS bad key.
pub const ERROR_RCODE_BADKEY : ErrorCode = ErrorCode::from_constant(9017); // DNS_ERROR_RCODE_BADKEY

/// DNS signature validity expired.
pub const ERROR_RCODE_BADTIME : ErrorCode = ErrorCode::from_constant(9018); // DNS_ERROR_RCODE_BADTIME

/// Only the DNS server acting as the key master for the zone may perform this operation.
pub const ERROR_KEYMASTER_REQUIRED : ErrorCode = ErrorCode::from_constant(9101); // DNS_ERROR_KEYMASTER_REQUIRED

/// This operation is not allowed on a zone that is signed or has signing keys.
pub const ERROR_NOT_ALLOWED_ON_SIGNED_ZONE : ErrorCode = ErrorCode::from_constant(9102); // DNS_ERROR_NOT_ALLOWED_ON_SIGNED_ZONE

/// NSEC3 is not compatible with the RSA-SHA-1 algorithm. Choose a different algorithm or use NSEC.
pub const ERROR_NSEC3_INCOMPATIBLE_WITH_RSA_SHA1 : ErrorCode = ErrorCode::from_constant(9103); // DNS_ERROR_NSEC3_INCOMPATIBLE_WITH_RSA_SHA1

/// The zone does not have enough signing keys. There must be at least one key signing key (KSK) and at least one zone signing key (ZSK).
pub const ERROR_NOT_ENOUGH_SIGNING_KEY_DESCRIPTORS : ErrorCode = ErrorCode::from_constant(9104); // DNS_ERROR_NOT_ENOUGH_SIGNING_KEY_DESCRIPTORS

/// The specified algorithm is not supported.
pub const ERROR_UNSUPPORTED_ALGORITHM : ErrorCode = ErrorCode::from_constant(9105); // DNS_ERROR_UNSUPPORTED_ALGORITHM

/// The specified key size is not supported.
pub const ERROR_INVALID_KEY_SIZE : ErrorCode = ErrorCode::from_constant(9106); // DNS_ERROR_INVALID_KEY_SIZE

/// One or more of the signing keys for a zone are not accessible to the DNS server. Zone signing will not be operational until this error is resolved.
pub const ERROR_SIGNING_KEY_NOT_ACCESSIBLE : ErrorCode = ErrorCode::from_constant(9107); // DNS_ERROR_SIGNING_KEY_NOT_ACCESSIBLE

/// The specified key storage provider does not support DPAPI++ data protection. Zone signing will not be operational until this error is resolved.
pub const ERROR_KSP_DOES_NOT_SUPPORT_PROTECTION : ErrorCode = ErrorCode::from_constant(9108); // DNS_ERROR_KSP_DOES_NOT_SUPPORT_PROTECTION

/// An unexpected DPAPI++ error was encountered. Zone signing will not be operational until this error is resolved.
pub const ERROR_UNEXPECTED_DATA_PROTECTION_ERROR : ErrorCode = ErrorCode::from_constant(9109); // DNS_ERROR_UNEXPECTED_DATA_PROTECTION_ERROR

/// An unexpected crypto error was encountered. Zone signing may not be operational until this error is resolved.
pub const ERROR_UNEXPECTED_CNG_ERROR : ErrorCode = ErrorCode::from_constant(9110); // DNS_ERROR_UNEXPECTED_CNG_ERROR

/// The DNS server encountered a signing key with an unknown version. Zone signing will not be operational until this error is resolved.
pub const ERROR_UNKNOWN_SIGNING_PARAMETER_VERSION : ErrorCode = ErrorCode::from_constant(9111); // DNS_ERROR_UNKNOWN_SIGNING_PARAMETER_VERSION

/// The specified key service provider cannot be opened by the DNS server.
pub const ERROR_KSP_NOT_ACCESSIBLE : ErrorCode = ErrorCode::from_constant(9112); // DNS_ERROR_KSP_NOT_ACCESSIBLE

/// The DNS server cannot accept any more signing keys with the specified algorithm and KSK flag value for this zone.
pub const ERROR_TOO_MANY_SKDS : ErrorCode = ErrorCode::from_constant(9113); // DNS_ERROR_TOO_MANY_SKDS

/// The specified rollover period is invalid.
pub const ERROR_INVALID_ROLLOVER_PERIOD : ErrorCode = ErrorCode::from_constant(9114); // DNS_ERROR_INVALID_ROLLOVER_PERIOD

/// The specified initial rollover offset is invalid.
pub const ERROR_INVALID_INITIAL_ROLLOVER_OFFSET : ErrorCode = ErrorCode::from_constant(9115); // DNS_ERROR_INVALID_INITIAL_ROLLOVER_OFFSET

/// The specified signing key is already in process of rolling over keys.
pub const ERROR_ROLLOVER_IN_PROGRESS : ErrorCode = ErrorCode::from_constant(9116); // DNS_ERROR_ROLLOVER_IN_PROGRESS

/// The specified signing key does not have a standby key to revoke.
pub const ERROR_STANDBY_KEY_NOT_PRESENT : ErrorCode = ErrorCode::from_constant(9117); // DNS_ERROR_STANDBY_KEY_NOT_PRESENT

/// This operation is not allowed on a zone signing key (ZSK).
pub const ERROR_NOT_ALLOWED_ON_ZSK : ErrorCode = ErrorCode::from_constant(9118); // DNS_ERROR_NOT_ALLOWED_ON_ZSK

/// This operation is not allowed on an active signing key.
pub const ERROR_NOT_ALLOWED_ON_ACTIVE_SKD : ErrorCode = ErrorCode::from_constant(9119); // DNS_ERROR_NOT_ALLOWED_ON_ACTIVE_SKD

/// The specified signing key is already queued for rollover.
pub const ERROR_ROLLOVER_ALREADY_QUEUED : ErrorCode = ErrorCode::from_constant(9120); // DNS_ERROR_ROLLOVER_ALREADY_QUEUED

/// This operation is not allowed on an unsigned zone.
pub const ERROR_NOT_ALLOWED_ON_UNSIGNED_ZONE : ErrorCode = ErrorCode::from_constant(9121); // DNS_ERROR_NOT_ALLOWED_ON_UNSIGNED_ZONE

/// This operation could not be completed because the DNS server listed as the current key master for this zone is down or misconfigured. Resolve the problem on the current key master for this zone or use another DNS server to seize the key master role.
pub const ERROR_BAD_KEYMASTER : ErrorCode = ErrorCode::from_constant(9122); // DNS_ERROR_BAD_KEYMASTER

/// The specified signature validity period is invalid.
pub const ERROR_INVALID_SIGNATURE_VALIDITY_PERIOD : ErrorCode = ErrorCode::from_constant(9123); // DNS_ERROR_INVALID_SIGNATURE_VALIDITY_PERIOD

/// The specified NSEC3 iteration count is higher than allowed by the minimum key length used in the zone.
pub const ERROR_INVALID_NSEC3_ITERATION_COUNT : ErrorCode = ErrorCode::from_constant(9124); // DNS_ERROR_INVALID_NSEC3_ITERATION_COUNT

/// This operation could not be completed because the DNS server has been configured with DNSSEC features disabled. Enable DNSSEC on the DNS server.
pub const ERROR_DNSSEC_IS_DISABLED : ErrorCode = ErrorCode::from_constant(9125); // DNS_ERROR_DNSSEC_IS_DISABLED

/// This operation could not be completed because the XML stream received is empty or syntactically invalid.
pub const ERROR_INVALID_XML : ErrorCode = ErrorCode::from_constant(9126); // DNS_ERROR_INVALID_XML

/// This operation completed, but no trust anchors were added because all of the trust anchors received were either invalid, unsupported, expired, or would not become valid in less than 30 days.
pub const ERROR_NO_VALID_TRUST_ANCHORS : ErrorCode = ErrorCode::from_constant(9127); // DNS_ERROR_NO_VALID_TRUST_ANCHORS

/// The specified signing key is not waiting for parental DS update.
pub const ERROR_ROLLOVER_NOT_POKEABLE : ErrorCode = ErrorCode::from_constant(9128); // DNS_ERROR_ROLLOVER_NOT_POKEABLE

/// Hash collision detected during NSEC3 signing. Specify a different user-provided salt, or use a randomly generated salt, and attempt to sign the zone again.
pub const ERROR_NSEC3_NAME_COLLISION : ErrorCode = ErrorCode::from_constant(9129); // DNS_ERROR_NSEC3_NAME_COLLISION

/// NSEC is not compatible with the NSEC3-RSA-SHA-1 algorithm. Choose a different algorithm or use NSEC3.
pub const ERROR_NSEC_INCOMPATIBLE_WITH_NSEC3_RSA_SHA1 : ErrorCode = ErrorCode::from_constant(9130); // DNS_ERROR_NSEC_INCOMPATIBLE_WITH_NSEC3_RSA_SHA1

/// Bad DNS packet.
pub const ERROR_BAD_PACKET : ErrorCode = ErrorCode::from_constant(9502); // DNS_ERROR_BAD_PACKET

/// No DNS packet.
pub const ERROR_NO_PACKET : ErrorCode = ErrorCode::from_constant(9503); // DNS_ERROR_NO_PACKET

/// DNS error, check rcode.
pub const ERROR_RCODE : ErrorCode = ErrorCode::from_constant(9504); // DNS_ERROR_RCODE

/// Unsecured DNS packet.
pub const ERROR_UNSECURE_PACKET : ErrorCode = ErrorCode::from_constant(9505); // DNS_ERROR_UNSECURE_PACKET

pub const ERROR_NO_MEMORY : ErrorCode = ErrorCode::from_constant(14); // DNS_ERROR_NO_MEMORY
pub const ERROR_INVALID_NAME : ErrorCode = ErrorCode::from_constant(123); // DNS_ERROR_INVALID_NAME
pub const ERROR_INVALID_DATA : ErrorCode = ErrorCode::from_constant(13); // DNS_ERROR_INVALID_DATA

/// Invalid DNS type.
pub const ERROR_INVALID_TYPE : ErrorCode = ErrorCode::from_constant(9551); // DNS_ERROR_INVALID_TYPE

/// Invalid IP address.
pub const ERROR_INVALID_IP_ADDRESS : ErrorCode = ErrorCode::from_constant(9552); // DNS_ERROR_INVALID_IP_ADDRESS

/// Invalid property.
pub const ERROR_INVALID_PROPERTY : ErrorCode = ErrorCode::from_constant(9553); // DNS_ERROR_INVALID_PROPERTY

/// Try DNS operation again later.
pub const ERROR_TRY_AGAIN_LATER : ErrorCode = ErrorCode::from_constant(9554); // DNS_ERROR_TRY_AGAIN_LATER

/// Record for given name and type is not unique.
pub const ERROR_NOT_UNIQUE : ErrorCode = ErrorCode::from_constant(9555); // DNS_ERROR_NOT_UNIQUE

/// DNS name does not comply with RFC specifications.
pub const ERROR_NON_RFC_NAME : ErrorCode = ErrorCode::from_constant(9556); // DNS_ERROR_NON_RFC_NAME

/// DNS name contains an invalid character.
pub const ERROR_INVALID_NAME_CHAR : ErrorCode = ErrorCode::from_constant(9560); // DNS_ERROR_INVALID_NAME_CHAR

/// DNS name is entirely numeric.
pub const ERROR_NUMERIC_NAME : ErrorCode = ErrorCode::from_constant(9561); // DNS_ERROR_NUMERIC_NAME

/// The operation requested is not permitted on a DNS root server.
pub const ERROR_NOT_ALLOWED_ON_ROOT_SERVER : ErrorCode = ErrorCode::from_constant(9562); // DNS_ERROR_NOT_ALLOWED_ON_ROOT_SERVER

/// The record could not be created because this part of the DNS namespace has been delegated to another server.
pub const ERROR_NOT_ALLOWED_UNDER_DELEGATION : ErrorCode = ErrorCode::from_constant(9563); // DNS_ERROR_NOT_ALLOWED_UNDER_DELEGATION

/// The DNS server could not find a set of root hints.
pub const ERROR_CANNOT_FIND_ROOT_HINTS : ErrorCode = ErrorCode::from_constant(9564); // DNS_ERROR_CANNOT_FIND_ROOT_HINTS

/// The DNS server found root hints but they were not consistent across all adapters.
pub const ERROR_INCONSISTENT_ROOT_HINTS : ErrorCode = ErrorCode::from_constant(9565); // DNS_ERROR_INCONSISTENT_ROOT_HINTS

/// The specified value is too small for this parameter.
pub const ERROR_DWORD_VALUE_TOO_SMALL : ErrorCode = ErrorCode::from_constant(9566); // DNS_ERROR_DWORD_VALUE_TOO_SMALL

/// The specified value is too large for this parameter.
pub const ERROR_DWORD_VALUE_TOO_LARGE : ErrorCode = ErrorCode::from_constant(9567); // DNS_ERROR_DWORD_VALUE_TOO_LARGE

/// This operation is not allowed while the DNS server is loading zones in the background. Please try again later.
pub const ERROR_BACKGROUND_LOADING : ErrorCode = ErrorCode::from_constant(9568); // DNS_ERROR_BACKGROUND_LOADING

/// The operation requested is not permitted on against a DNS server running on a read-only DC.
pub const ERROR_NOT_ALLOWED_ON_RODC : ErrorCode = ErrorCode::from_constant(9569); // DNS_ERROR_NOT_ALLOWED_ON_RODC

/// No data is allowed to exist underneath a DNAME record.
pub const ERROR_NOT_ALLOWED_UNDER_DNAME : ErrorCode = ErrorCode::from_constant(9570); // DNS_ERROR_NOT_ALLOWED_UNDER_DNAME

/// This operation requires credentials delegation.
pub const ERROR_DELEGATION_REQUIRED : ErrorCode = ErrorCode::from_constant(9571); // DNS_ERROR_DELEGATION_REQUIRED

/// Name resolution policy table has been corrupted. DNS resolution will fail until it is fixed. Contact your network administrator.
pub const ERROR_INVALID_POLICY_TABLE : ErrorCode = ErrorCode::from_constant(9572); // DNS_ERROR_INVALID_POLICY_TABLE

/// Not allowed to remove all addresses.
pub const ERROR_ADDRESS_REQUIRED : ErrorCode = ErrorCode::from_constant(9573); // DNS_ERROR_ADDRESS_REQUIRED

/// DNS zone does not exist.
pub const ERROR_ZONE_DOES_NOT_EXIST : ErrorCode = ErrorCode::from_constant(9601); // DNS_ERROR_ZONE_DOES_NOT_EXIST

/// DNS zone information not available.
pub const ERROR_NO_ZONE_INFO : ErrorCode = ErrorCode::from_constant(9602); // DNS_ERROR_NO_ZONE_INFO

/// Invalid operation for DNS zone.
pub const ERROR_INVALID_ZONE_OPERATION : ErrorCode = ErrorCode::from_constant(9603); // DNS_ERROR_INVALID_ZONE_OPERATION

/// Invalid DNS zone configuration.
pub const ERROR_ZONE_CONFIGURATION_ERROR : ErrorCode = ErrorCode::from_constant(9604); // DNS_ERROR_ZONE_CONFIGURATION_ERROR

/// DNS zone has no start of authority (SOA) record.
pub const ERROR_ZONE_HAS_NO_SOA_RECORD : ErrorCode = ErrorCode::from_constant(9605); // DNS_ERROR_ZONE_HAS_NO_SOA_RECORD

/// DNS zone has no Name Server (NS) record.
pub const ERROR_ZONE_HAS_NO_NS_RECORDS : ErrorCode = ErrorCode::from_constant(9606); // DNS_ERROR_ZONE_HAS_NO_NS_RECORDS

/// DNS zone is locked.
pub const ERROR_ZONE_LOCKED : ErrorCode = ErrorCode::from_constant(9607); // DNS_ERROR_ZONE_LOCKED

/// DNS zone creation failed.
pub const ERROR_ZONE_CREATION_FAILED : ErrorCode = ErrorCode::from_constant(9608); // DNS_ERROR_ZONE_CREATION_FAILED

/// DNS zone already exists.
pub const ERROR_ZONE_ALREADY_EXISTS : ErrorCode = ErrorCode::from_constant(9609); // DNS_ERROR_ZONE_ALREADY_EXISTS

/// DNS automatic zone already exists.
pub const ERROR_AUTOZONE_ALREADY_EXISTS : ErrorCode = ErrorCode::from_constant(9610); // DNS_ERROR_AUTOZONE_ALREADY_EXISTS

/// Invalid DNS zone type.
pub const ERROR_INVALID_ZONE_TYPE : ErrorCode = ErrorCode::from_constant(9611); // DNS_ERROR_INVALID_ZONE_TYPE

/// Secondary DNS zone requires master IP address.
pub const ERROR_SECONDARY_REQUIRES_MASTER_IP : ErrorCode = ErrorCode::from_constant(9612); // DNS_ERROR_SECONDARY_REQUIRES_MASTER_IP

/// DNS zone not secondary.
pub const ERROR_ZONE_NOT_SECONDARY : ErrorCode = ErrorCode::from_constant(9613); // DNS_ERROR_ZONE_NOT_SECONDARY

/// Need secondary IP address.
pub const ERROR_NEED_SECONDARY_ADDRESSES : ErrorCode = ErrorCode::from_constant(9614); // DNS_ERROR_NEED_SECONDARY_ADDRESSES

/// WINS initialization failed.
pub const ERROR_WINS_INIT_FAILED : ErrorCode = ErrorCode::from_constant(9615); // DNS_ERROR_WINS_INIT_FAILED

/// Need WINS servers.
pub const ERROR_NEED_WINS_SERVERS : ErrorCode = ErrorCode::from_constant(9616); // DNS_ERROR_NEED_WINS_SERVERS

/// NBTSTAT initialization call failed.
pub const ERROR_NBSTAT_INIT_FAILED : ErrorCode = ErrorCode::from_constant(9617); // DNS_ERROR_NBSTAT_INIT_FAILED

/// Invalid delete of start of authority (SOA)
pub const ERROR_SOA_DELETE_INVALID : ErrorCode = ErrorCode::from_constant(9618); // DNS_ERROR_SOA_DELETE_INVALID

/// A conditional forwarding zone already exists for that name.
pub const ERROR_FORWARDER_ALREADY_EXISTS : ErrorCode = ErrorCode::from_constant(9619); // DNS_ERROR_FORWARDER_ALREADY_EXISTS

/// This zone must be configured with one or more master DNS server IP addresses.
pub const ERROR_ZONE_REQUIRES_MASTER_IP : ErrorCode = ErrorCode::from_constant(9620); // DNS_ERROR_ZONE_REQUIRES_MASTER_IP

/// The operation cannot be performed because this zone is shut down.
pub const ERROR_ZONE_IS_SHUTDOWN : ErrorCode = ErrorCode::from_constant(9621); // DNS_ERROR_ZONE_IS_SHUTDOWN

/// This operation cannot be performed because the zone is currently being signed. Please try again later.
pub const ERROR_ZONE_LOCKED_FOR_SIGNING : ErrorCode = ErrorCode::from_constant(9622); // DNS_ERROR_ZONE_LOCKED_FOR_SIGNING

/// Primary DNS zone requires datafile.
pub const ERROR_PRIMARY_REQUIRES_DATAFILE : ErrorCode = ErrorCode::from_constant(9651); // DNS_ERROR_PRIMARY_REQUIRES_DATAFILE

/// Invalid datafile name for DNS zone.
pub const ERROR_INVALID_DATAFILE_NAME : ErrorCode = ErrorCode::from_constant(9652); // DNS_ERROR_INVALID_DATAFILE_NAME

/// Failed to open datafile for DNS zone.
pub const ERROR_DATAFILE_OPEN_FAILURE : ErrorCode = ErrorCode::from_constant(9653); // DNS_ERROR_DATAFILE_OPEN_FAILURE

/// Failed to write datafile for DNS zone.
pub const ERROR_FILE_WRITEBACK_FAILED : ErrorCode = ErrorCode::from_constant(9654); // DNS_ERROR_FILE_WRITEBACK_FAILED

/// Failure while reading datafile for DNS zone.
pub const ERROR_DATAFILE_PARSING : ErrorCode = ErrorCode::from_constant(9655); // DNS_ERROR_DATAFILE_PARSING

/// DNS record does not exist.
pub const ERROR_RECORD_DOES_NOT_EXIST : ErrorCode = ErrorCode::from_constant(9701); // DNS_ERROR_RECORD_DOES_NOT_EXIST

/// DNS record format error.
pub const ERROR_RECORD_FORMAT : ErrorCode = ErrorCode::from_constant(9702); // DNS_ERROR_RECORD_FORMAT

/// Node creation failure in DNS.
pub const ERROR_NODE_CREATION_FAILED : ErrorCode = ErrorCode::from_constant(9703); // DNS_ERROR_NODE_CREATION_FAILED

/// Unknown DNS record type.
pub const ERROR_UNKNOWN_RECORD_TYPE : ErrorCode = ErrorCode::from_constant(9704); // DNS_ERROR_UNKNOWN_RECORD_TYPE

/// DNS record timed out.
pub const ERROR_RECORD_TIMED_OUT : ErrorCode = ErrorCode::from_constant(9705); // DNS_ERROR_RECORD_TIMED_OUT

/// Name not in DNS zone.
pub const ERROR_NAME_NOT_IN_ZONE : ErrorCode = ErrorCode::from_constant(9706); // DNS_ERROR_NAME_NOT_IN_ZONE

/// CNAME loop detected.
pub const ERROR_CNAME_LOOP : ErrorCode = ErrorCode::from_constant(9707); // DNS_ERROR_CNAME_LOOP

/// Node is a CNAME DNS record.
pub const ERROR_NODE_IS_CNAME : ErrorCode = ErrorCode::from_constant(9708); // DNS_ERROR_NODE_IS_CNAME

/// A CNAME record already exists for given name.
pub const ERROR_CNAME_COLLISION : ErrorCode = ErrorCode::from_constant(9709); // DNS_ERROR_CNAME_COLLISION

/// Record only at DNS zone root.
pub const ERROR_RECORD_ONLY_AT_ZONE_ROOT : ErrorCode = ErrorCode::from_constant(9710); // DNS_ERROR_RECORD_ONLY_AT_ZONE_ROOT

/// DNS record already exists.
pub const ERROR_RECORD_ALREADY_EXISTS : ErrorCode = ErrorCode::from_constant(9711); // DNS_ERROR_RECORD_ALREADY_EXISTS

/// Secondary DNS zone data error.
pub const ERROR_SECONDARY_DATA : ErrorCode = ErrorCode::from_constant(9712); // DNS_ERROR_SECONDARY_DATA

/// Could not create DNS cache data.
pub const ERROR_NO_CREATE_CACHE_DATA : ErrorCode = ErrorCode::from_constant(9713); // DNS_ERROR_NO_CREATE_CACHE_DATA

/// DNS name does not exist.
pub const ERROR_NAME_DOES_NOT_EXIST : ErrorCode = ErrorCode::from_constant(9714); // DNS_ERROR_NAME_DOES_NOT_EXIST

/// The directory service is unavailable.
pub const ERROR_DS_UNAVAILABLE : ErrorCode = ErrorCode::from_constant(9717); // DNS_ERROR_DS_UNAVAILABLE

/// DNS zone already exists in the directory service.
pub const ERROR_DS_ZONE_ALREADY_EXISTS : ErrorCode = ErrorCode::from_constant(9718); // DNS_ERROR_DS_ZONE_ALREADY_EXISTS

/// DNS server not creating or reading the boot file for the directory service integrated DNS zone.
pub const ERROR_NO_BOOTFILE_IF_DS_ZONE : ErrorCode = ErrorCode::from_constant(9719); // DNS_ERROR_NO_BOOTFILE_IF_DS_ZONE

/// Node is a DNAME DNS record.
pub const ERROR_NODE_IS_DNAME : ErrorCode = ErrorCode::from_constant(9720); // DNS_ERROR_NODE_IS_DNAME

/// A DNAME record already exists for given name.
pub const ERROR_DNAME_COLLISION : ErrorCode = ErrorCode::from_constant(9721); // DNS_ERROR_DNAME_COLLISION

/// An alias loop has been detected with either CNAME or DNAME records.
pub const ERROR_ALIAS_LOOP : ErrorCode = ErrorCode::from_constant(9722); // DNS_ERROR_ALIAS_LOOP

/// DNS zone transfer failed.
pub const ERROR_AXFR : ErrorCode = ErrorCode::from_constant(9752); // DNS_ERROR_AXFR

/// TCP/IP network protocol not installed.
pub const ERROR_NO_TCPIP : ErrorCode = ErrorCode::from_constant(9851); // DNS_ERROR_NO_TCPIP

/// No DNS servers configured for local system.
pub const ERROR_NO_DNS_SERVERS : ErrorCode = ErrorCode::from_constant(9852); // DNS_ERROR_NO_DNS_SERVERS

/// The specified directory partition does not exist.
pub const ERROR_DP_DOES_NOT_EXIST : ErrorCode = ErrorCode::from_constant(9901); // DNS_ERROR_DP_DOES_NOT_EXIST

/// The specified directory partition already exists.
pub const ERROR_DP_ALREADY_EXISTS : ErrorCode = ErrorCode::from_constant(9902); // DNS_ERROR_DP_ALREADY_EXISTS

/// This DNS server is not enlisted in the specified directory partition.
pub const ERROR_DP_NOT_ENLISTED : ErrorCode = ErrorCode::from_constant(9903); // DNS_ERROR_DP_NOT_ENLISTED

/// This DNS server is already enlisted in the specified directory partition.
pub const ERROR_DP_ALREADY_ENLISTED : ErrorCode = ErrorCode::from_constant(9904); // DNS_ERROR_DP_ALREADY_ENLISTED

/// The directory partition is not available at this time. Please wait a few minutes and try again.
pub const ERROR_DP_NOT_AVAILABLE : ErrorCode = ErrorCode::from_constant(9905); // DNS_ERROR_DP_NOT_AVAILABLE

/// The operation failed because the domain naming master FSMO role could not be reached. The domain controller holding the domain naming master FSMO role is down or unable to service the request or is not running Windows Server 2003 or later.
pub const ERROR_DP_FSMO_ERROR : ErrorCode = ErrorCode::from_constant(9906); // DNS_ERROR_DP_FSMO_ERROR

/// The RRL is not enabled.
pub const ERROR_RRL_NOT_ENABLED : ErrorCode = ErrorCode::from_constant(9911); // DNS_ERROR_RRL_NOT_ENABLED

/// The window size parameter is invalid. It should be greater than or equal to 1.
pub const ERROR_RRL_INVALID_WINDOW_SIZE : ErrorCode = ErrorCode::from_constant(9912); // DNS_ERROR_RRL_INVALID_WINDOW_SIZE

/// The IPv4 prefix length parameter is invalid. It should be less than or equal to 32.
pub const ERROR_RRL_INVALID_IPV4_PREFIX : ErrorCode = ErrorCode::from_constant(9913); // DNS_ERROR_RRL_INVALID_IPV4_PREFIX

/// The IPv6 prefix length parameter is invalid. It should be less than or equal to 128.
pub const ERROR_RRL_INVALID_IPV6_PREFIX : ErrorCode = ErrorCode::from_constant(9914); // DNS_ERROR_RRL_INVALID_IPV6_PREFIX

/// The TC Rate parameter is invalid. It should be less than 10.
pub const ERROR_RRL_INVALID_TC_RATE : ErrorCode = ErrorCode::from_constant(9915); // DNS_ERROR_RRL_INVALID_TC_RATE

/// The Leak Rate parameter is invalid. It should be either 0, or between 2 and 10.
pub const ERROR_RRL_INVALID_LEAK_RATE : ErrorCode = ErrorCode::from_constant(9916); // DNS_ERROR_RRL_INVALID_LEAK_RATE

/// The Leak Rate or TC Rate parameter is invalid. Leak Rate should be greater than TC Rate.
pub const ERROR_RRL_LEAK_RATE_LESSTHAN_TC_RATE : ErrorCode = ErrorCode::from_constant(9917); // DNS_ERROR_RRL_LEAK_RATE_LESSTHAN_TC_RATE

/// The virtualization instance already exists.
pub const ERROR_VIRTUALIZATION_INSTANCE_ALREADY_EXISTS : ErrorCode = ErrorCode::from_constant(9921); // DNS_ERROR_VIRTUALIZATION_INSTANCE_ALREADY_EXISTS

/// The virtualization instance does not exist.
pub const ERROR_VIRTUALIZATION_INSTANCE_DOES_NOT_EXIST : ErrorCode = ErrorCode::from_constant(9922); // DNS_ERROR_VIRTUALIZATION_INSTANCE_DOES_NOT_EXIST

/// The virtualization tree is locked.
pub const ERROR_VIRTUALIZATION_TREE_LOCKED : ErrorCode = ErrorCode::from_constant(9923); // DNS_ERROR_VIRTUALIZATION_TREE_LOCKED

/// Invalid virtualization instance name.
pub const ERROR_INVAILD_VIRTUALIZATION_INSTANCE_NAME : ErrorCode = ErrorCode::from_constant(9924); // DNS_ERROR_INVAILD_VIRTUALIZATION_INSTANCE_NAME

/// The default virtualization instance cannot be added, removed or modified.
pub const ERROR_DEFAULT_VIRTUALIZATION_INSTANCE : ErrorCode = ErrorCode::from_constant(9925); // DNS_ERROR_DEFAULT_VIRTUALIZATION_INSTANCE

/// The scope already exists for the zone.
pub const ERROR_ZONESCOPE_ALREADY_EXISTS : ErrorCode = ErrorCode::from_constant(9951); // DNS_ERROR_ZONESCOPE_ALREADY_EXISTS

/// The scope does not exist for the zone.
pub const ERROR_ZONESCOPE_DOES_NOT_EXIST : ErrorCode = ErrorCode::from_constant(9952); // DNS_ERROR_ZONESCOPE_DOES_NOT_EXIST

/// The scope is the same as the default zone scope.
pub const ERROR_DEFAULT_ZONESCOPE : ErrorCode = ErrorCode::from_constant(9953); // DNS_ERROR_DEFAULT_ZONESCOPE

/// The scope name contains invalid characters.
pub const ERROR_INVALID_ZONESCOPE_NAME : ErrorCode = ErrorCode::from_constant(9954); // DNS_ERROR_INVALID_ZONESCOPE_NAME

/// Operation not allowed when the zone has scopes.
pub const ERROR_NOT_ALLOWED_WITH_ZONESCOPES : ErrorCode = ErrorCode::from_constant(9955); // DNS_ERROR_NOT_ALLOWED_WITH_ZONESCOPES

/// Failed to load zone scope.
pub const ERROR_LOAD_ZONESCOPE_FAILED : ErrorCode = ErrorCode::from_constant(9956); // DNS_ERROR_LOAD_ZONESCOPE_FAILED

/// Failed to write data file for DNS zone scope. Please verify the file exists and is writable.
pub const ERROR_ZONESCOPE_FILE_WRITEBACK_FAILED : ErrorCode = ErrorCode::from_constant(9957); // DNS_ERROR_ZONESCOPE_FILE_WRITEBACK_FAILED

/// The scope name contains invalid characters.
pub const ERROR_INVALID_SCOPE_NAME : ErrorCode = ErrorCode::from_constant(9958); // DNS_ERROR_INVALID_SCOPE_NAME

/// The scope does not exist.
pub const ERROR_SCOPE_DOES_NOT_EXIST : ErrorCode = ErrorCode::from_constant(9959); // DNS_ERROR_SCOPE_DOES_NOT_EXIST

/// The scope is the same as the default scope.
pub const ERROR_DEFAULT_SCOPE : ErrorCode = ErrorCode::from_constant(9960); // DNS_ERROR_DEFAULT_SCOPE

/// The operation is invalid on the scope.
pub const ERROR_INVALID_SCOPE_OPERATION : ErrorCode = ErrorCode::from_constant(9961); // DNS_ERROR_INVALID_SCOPE_OPERATION

/// The scope is locked.
pub const ERROR_SCOPE_LOCKED : ErrorCode = ErrorCode::from_constant(9962); // DNS_ERROR_SCOPE_LOCKED

/// The scope already exists.
pub const ERROR_SCOPE_ALREADY_EXISTS : ErrorCode = ErrorCode::from_constant(9963); // DNS_ERROR_SCOPE_ALREADY_EXISTS

/// A policy with the same name already exists on this level (server level or zone level) on the DNS server.
pub const ERROR_POLICY_ALREADY_EXISTS : ErrorCode = ErrorCode::from_constant(9971); // DNS_ERROR_POLICY_ALREADY_EXISTS

/// No policy with this name exists on this level (server level or zone level) on the DNS server.
pub const ERROR_POLICY_DOES_NOT_EXIST : ErrorCode = ErrorCode::from_constant(9972); // DNS_ERROR_POLICY_DOES_NOT_EXIST

/// The criteria provided in the policy are invalid.
pub const ERROR_POLICY_INVALID_CRITERIA : ErrorCode = ErrorCode::from_constant(9973); // DNS_ERROR_POLICY_INVALID_CRITERIA

/// At least one of the settings of this policy is invalid.
pub const ERROR_POLICY_INVALID_SETTINGS : ErrorCode = ErrorCode::from_constant(9974); // DNS_ERROR_POLICY_INVALID_SETTINGS

/// The client subnet cannot be deleted while it is being accessed by a policy.
pub const ERROR_CLIENT_SUBNET_IS_ACCESSED : ErrorCode = ErrorCode::from_constant(9975); // DNS_ERROR_CLIENT_SUBNET_IS_ACCESSED

/// The client subnet does not exist on the DNS server.
pub const ERROR_CLIENT_SUBNET_DOES_NOT_EXIST : ErrorCode = ErrorCode::from_constant(9976); // DNS_ERROR_CLIENT_SUBNET_DOES_NOT_EXIST

/// A client subnet with this name already exists on the DNS server.
pub const ERROR_CLIENT_SUBNET_ALREADY_EXISTS : ErrorCode = ErrorCode::from_constant(9977); // DNS_ERROR_CLIENT_SUBNET_ALREADY_EXISTS

/// The IP subnet specified does not exist in the client subnet.
pub const ERROR_SUBNET_DOES_NOT_EXIST : ErrorCode = ErrorCode::from_constant(9978); // DNS_ERROR_SUBNET_DOES_NOT_EXIST

/// The IP subnet that is being added, already exists in the client subnet.
pub const ERROR_SUBNET_ALREADY_EXISTS : ErrorCode = ErrorCode::from_constant(9979); // DNS_ERROR_SUBNET_ALREADY_EXISTS

/// The policy is locked.
pub const ERROR_POLICY_LOCKED : ErrorCode = ErrorCode::from_constant(9980); // DNS_ERROR_POLICY_LOCKED

/// The weight of the scope in the policy is invalid.
pub const ERROR_POLICY_INVALID_WEIGHT : ErrorCode = ErrorCode::from_constant(9981); // DNS_ERROR_POLICY_INVALID_WEIGHT

/// The DNS policy name is invalid.
pub const ERROR_POLICY_INVALID_NAME : ErrorCode = ErrorCode::from_constant(9982); // DNS_ERROR_POLICY_INVALID_NAME

/// The policy is missing criteria.
pub const ERROR_POLICY_MISSING_CRITERIA : ErrorCode = ErrorCode::from_constant(9983); // DNS_ERROR_POLICY_MISSING_CRITERIA

/// The name of the the client subnet record is invalid.
pub const ERROR_INVALID_CLIENT_SUBNET_NAME : ErrorCode = ErrorCode::from_constant(9984); // DNS_ERROR_INVALID_CLIENT_SUBNET_NAME

/// Invalid policy processing order.
pub const ERROR_POLICY_PROCESSING_ORDER_INVALID : ErrorCode = ErrorCode::from_constant(9985); // DNS_ERROR_POLICY_PROCESSING_ORDER_INVALID

/// The scope information has not been provided for a policy that requires it.
pub const ERROR_POLICY_SCOPE_MISSING : ErrorCode = ErrorCode::from_constant(9986); // DNS_ERROR_POLICY_SCOPE_MISSING

/// The scope information has been provided for a policy that does not require it.
pub const ERROR_POLICY_SCOPE_NOT_ALLOWED : ErrorCode = ErrorCode::from_constant(9987); // DNS_ERROR_POLICY_SCOPE_NOT_ALLOWED

/// The server scope cannot be deleted because it is referenced by a DNS Policy.
pub const ERROR_SERVERSCOPE_IS_REFERENCED : ErrorCode = ErrorCode::from_constant(9988); // DNS_ERROR_SERVERSCOPE_IS_REFERENCED

/// The zone scope cannot be deleted because it is referenced by a DNS Policy.
pub const ERROR_ZONESCOPE_IS_REFERENCED : ErrorCode = ErrorCode::from_constant(9989); // DNS_ERROR_ZONESCOPE_IS_REFERENCED

/// The criterion client subnet provided in the policy is invalid.
pub const ERROR_POLICY_INVALID_CRITERIA_CLIENT_SUBNET : ErrorCode = ErrorCode::from_constant(9990); // DNS_ERROR_POLICY_INVALID_CRITERIA_CLIENT_SUBNET

/// The criterion transport protocol provided in the policy is invalid.
pub const ERROR_POLICY_INVALID_CRITERIA_TRANSPORT_PROTOCOL : ErrorCode = ErrorCode::from_constant(9991); // DNS_ERROR_POLICY_INVALID_CRITERIA_TRANSPORT_PROTOCOL

/// The criterion network protocol provided in the policy is invalid.
pub const ERROR_POLICY_INVALID_CRITERIA_NETWORK_PROTOCOL : ErrorCode = ErrorCode::from_constant(9992); // DNS_ERROR_POLICY_INVALID_CRITERIA_NETWORK_PROTOCOL

/// The criterion interface provided in the policy is invalid.
pub const ERROR_POLICY_INVALID_CRITERIA_INTERFACE : ErrorCode = ErrorCode::from_constant(9993); // DNS_ERROR_POLICY_INVALID_CRITERIA_INTERFACE

/// The criterion FQDN provided in the policy is invalid.
pub const ERROR_POLICY_INVALID_CRITERIA_FQDN : ErrorCode = ErrorCode::from_constant(9994); // DNS_ERROR_POLICY_INVALID_CRITERIA_FQDN

/// The criterion query type provided in the policy is invalid.
pub const ERROR_POLICY_INVALID_CRITERIA_QUERY_TYPE : ErrorCode = ErrorCode::from_constant(9995); // DNS_ERROR_POLICY_INVALID_CRITERIA_QUERY_TYPE

/// The criterion time of day provided in the policy is invalid.
pub const ERROR_POLICY_INVALID_CRITERIA_TIME_OF_DAY : ErrorCode = ErrorCode::from_constant(9996); // DNS_ERROR_POLICY_INVALID_CRITERIA_TIME_OF_DAY
