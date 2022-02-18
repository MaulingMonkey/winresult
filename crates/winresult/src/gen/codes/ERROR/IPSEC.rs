// WARNING: this file is auto-generated by xtask gen and may be overwritten

use super::*;


/// The specified quick mode policy already exists.
pub const QM_POLICY_EXISTS : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(13000); // ERROR_IPSEC_QM_POLICY_EXISTS

/// The specified quick mode policy was not found.
pub const QM_POLICY_NOT_FOUND : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(13001); // ERROR_IPSEC_QM_POLICY_NOT_FOUND

/// The specified quick mode policy is being used.
pub const QM_POLICY_IN_USE : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(13002); // ERROR_IPSEC_QM_POLICY_IN_USE

/// The specified main mode policy already exists.
pub const MM_POLICY_EXISTS : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(13003); // ERROR_IPSEC_MM_POLICY_EXISTS

/// The specified main mode policy was not found
pub const MM_POLICY_NOT_FOUND : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(13004); // ERROR_IPSEC_MM_POLICY_NOT_FOUND

/// The specified main mode policy is being used.
pub const MM_POLICY_IN_USE : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(13005); // ERROR_IPSEC_MM_POLICY_IN_USE

/// The specified main mode filter already exists.
pub const MM_FILTER_EXISTS : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(13006); // ERROR_IPSEC_MM_FILTER_EXISTS

/// The specified main mode filter was not found.
pub const MM_FILTER_NOT_FOUND : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(13007); // ERROR_IPSEC_MM_FILTER_NOT_FOUND

/// The specified transport mode filter already exists.
pub const TRANSPORT_FILTER_EXISTS : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(13008); // ERROR_IPSEC_TRANSPORT_FILTER_EXISTS

/// The specified transport mode filter does not exist.
pub const TRANSPORT_FILTER_NOT_FOUND : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(13009); // ERROR_IPSEC_TRANSPORT_FILTER_NOT_FOUND

/// The specified main mode authentication list exists.
pub const MM_AUTH_EXISTS : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(13010); // ERROR_IPSEC_MM_AUTH_EXISTS

/// The specified main mode authentication list was not found.
pub const MM_AUTH_NOT_FOUND : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(13011); // ERROR_IPSEC_MM_AUTH_NOT_FOUND

/// The specified main mode authentication list is being used.
pub const MM_AUTH_IN_USE : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(13012); // ERROR_IPSEC_MM_AUTH_IN_USE

/// The specified default main mode policy was not found.
pub const DEFAULT_MM_POLICY_NOT_FOUND : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(13013); // ERROR_IPSEC_DEFAULT_MM_POLICY_NOT_FOUND

/// The specified default main mode authentication list was not found.
pub const DEFAULT_MM_AUTH_NOT_FOUND : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(13014); // ERROR_IPSEC_DEFAULT_MM_AUTH_NOT_FOUND

/// The specified default quick mode policy was not found.
pub const DEFAULT_QM_POLICY_NOT_FOUND : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(13015); // ERROR_IPSEC_DEFAULT_QM_POLICY_NOT_FOUND

/// The specified tunnel mode filter exists.
pub const TUNNEL_FILTER_EXISTS : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(13016); // ERROR_IPSEC_TUNNEL_FILTER_EXISTS

/// The specified tunnel mode filter was not found.
pub const TUNNEL_FILTER_NOT_FOUND : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(13017); // ERROR_IPSEC_TUNNEL_FILTER_NOT_FOUND

/// The Main Mode filter is pending deletion.
pub const MM_FILTER_PENDING_DELETION : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(13018); // ERROR_IPSEC_MM_FILTER_PENDING_DELETION

/// The transport filter is pending deletion.
pub const TRANSPORT_FILTER_PENDING_DELETION : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(13019); // ERROR_IPSEC_TRANSPORT_FILTER_PENDING_DELETION

/// The tunnel filter is pending deletion.
pub const TUNNEL_FILTER_PENDING_DELETION : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(13020); // ERROR_IPSEC_TUNNEL_FILTER_PENDING_DELETION

/// The Main Mode policy is pending deletion.
pub const MM_POLICY_PENDING_DELETION : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(13021); // ERROR_IPSEC_MM_POLICY_PENDING_DELETION

/// The Main Mode authentication bundle is pending deletion.
pub const MM_AUTH_PENDING_DELETION : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(13022); // ERROR_IPSEC_MM_AUTH_PENDING_DELETION

/// The Quick Mode policy is pending deletion.
pub const QM_POLICY_PENDING_DELETION : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(13023); // ERROR_IPSEC_QM_POLICY_PENDING_DELETION

/// ERROR_IPSEC_IKE_NEG_STATUS_BEGIN
pub const IKE_NEG_STATUS_BEGIN : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(13800); // ERROR_IPSEC_IKE_NEG_STATUS_BEGIN

/// IKE authentication credentials are unacceptable
pub const IKE_AUTH_FAIL : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(13801); // ERROR_IPSEC_IKE_AUTH_FAIL

/// IKE security attributes are unacceptable
pub const IKE_ATTRIB_FAIL : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(13802); // ERROR_IPSEC_IKE_ATTRIB_FAIL

/// IKE Negotiation in progress
pub const IKE_NEGOTIATION_PENDING : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(13803); // ERROR_IPSEC_IKE_NEGOTIATION_PENDING

/// General processing error
pub const IKE_GENERAL_PROCESSING_ERROR : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(13804); // ERROR_IPSEC_IKE_GENERAL_PROCESSING_ERROR

/// Negotiation timed out
pub const IKE_TIMED_OUT : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(13805); // ERROR_IPSEC_IKE_TIMED_OUT

/// IKE failed to find valid machine certificate. Contact your Network Security Administrator about installing a valid certificate in the appropriate Certificate Store.
pub const IKE_NO_CERT : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(13806); // ERROR_IPSEC_IKE_NO_CERT

/// IKE SA deleted by peer before establishment completed
pub const IKE_SA_DELETED : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(13807); // ERROR_IPSEC_IKE_SA_DELETED

/// IKE SA deleted before establishment completed
pub const IKE_SA_REAPED : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(13808); // ERROR_IPSEC_IKE_SA_REAPED

/// Negotiation request sat in Queue too long
pub const IKE_MM_ACQUIRE_DROP : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(13809); // ERROR_IPSEC_IKE_MM_ACQUIRE_DROP

/// Negotiation request sat in Queue too long
pub const IKE_QM_ACQUIRE_DROP : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(13810); // ERROR_IPSEC_IKE_QM_ACQUIRE_DROP

/// Negotiation request sat in Queue too long
pub const IKE_QUEUE_DROP_MM : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(13811); // ERROR_IPSEC_IKE_QUEUE_DROP_MM

/// Negotiation request sat in Queue too long
pub const IKE_QUEUE_DROP_NO_MM : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(13812); // ERROR_IPSEC_IKE_QUEUE_DROP_NO_MM

/// No response from peer
pub const IKE_DROP_NO_RESPONSE : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(13813); // ERROR_IPSEC_IKE_DROP_NO_RESPONSE

/// Negotiation took too long
pub const IKE_MM_DELAY_DROP : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(13814); // ERROR_IPSEC_IKE_MM_DELAY_DROP

/// Negotiation took too long
pub const IKE_QM_DELAY_DROP : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(13815); // ERROR_IPSEC_IKE_QM_DELAY_DROP

/// Unknown error occurred
pub const IKE_ERROR : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(13816); // ERROR_IPSEC_IKE_ERROR

/// Certificate Revocation Check failed
pub const IKE_CRL_FAILED : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(13817); // ERROR_IPSEC_IKE_CRL_FAILED

/// Invalid certificate key usage
pub const IKE_INVALID_KEY_USAGE : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(13818); // ERROR_IPSEC_IKE_INVALID_KEY_USAGE

/// Invalid certificate type
pub const IKE_INVALID_CERT_TYPE : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(13819); // ERROR_IPSEC_IKE_INVALID_CERT_TYPE

/// IKE negotiation failed because the machine certificate used does not have a private key. IPsec certificates require a private key. Contact your Network Security administrator about replacing with a certificate that has a private key.
pub const IKE_NO_PRIVATE_KEY : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(13820); // ERROR_IPSEC_IKE_NO_PRIVATE_KEY

/// Simultaneous rekeys were detected.
pub const IKE_SIMULTANEOUS_REKEY : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(13821); // ERROR_IPSEC_IKE_SIMULTANEOUS_REKEY

/// Failure in Diffie-Hellman computation
pub const IKE_DH_FAIL : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(13822); // ERROR_IPSEC_IKE_DH_FAIL

/// Don't know how to process critical payload
pub const IKE_CRITICAL_PAYLOAD_NOT_RECOGNIZED : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(13823); // ERROR_IPSEC_IKE_CRITICAL_PAYLOAD_NOT_RECOGNIZED

/// Invalid header
pub const IKE_INVALID_HEADER : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(13824); // ERROR_IPSEC_IKE_INVALID_HEADER

/// No policy configured
pub const IKE_NO_POLICY : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(13825); // ERROR_IPSEC_IKE_NO_POLICY

/// Failed to verify signature
pub const IKE_INVALID_SIGNATURE : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(13826); // ERROR_IPSEC_IKE_INVALID_SIGNATURE

/// Failed to authenticate using Kerberos
pub const IKE_KERBEROS_ERROR : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(13827); // ERROR_IPSEC_IKE_KERBEROS_ERROR

/// Peer's certificate did not have a public key
pub const IKE_NO_PUBLIC_KEY : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(13828); // ERROR_IPSEC_IKE_NO_PUBLIC_KEY

/// Error processing error payload
pub const IKE_PROCESS_ERR : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(13829); // ERROR_IPSEC_IKE_PROCESS_ERR

/// Error processing SA payload
pub const IKE_PROCESS_ERR_SA : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(13830); // ERROR_IPSEC_IKE_PROCESS_ERR_SA

/// Error processing Proposal payload
pub const IKE_PROCESS_ERR_PROP : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(13831); // ERROR_IPSEC_IKE_PROCESS_ERR_PROP

/// Error processing Transform payload
pub const IKE_PROCESS_ERR_TRANS : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(13832); // ERROR_IPSEC_IKE_PROCESS_ERR_TRANS

/// Error processing KE payload
pub const IKE_PROCESS_ERR_KE : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(13833); // ERROR_IPSEC_IKE_PROCESS_ERR_KE

/// Error processing ID payload
pub const IKE_PROCESS_ERR_ID : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(13834); // ERROR_IPSEC_IKE_PROCESS_ERR_ID

/// Error processing Cert payload
pub const IKE_PROCESS_ERR_CERT : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(13835); // ERROR_IPSEC_IKE_PROCESS_ERR_CERT

/// Error processing Certificate Request payload
pub const IKE_PROCESS_ERR_CERT_REQ : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(13836); // ERROR_IPSEC_IKE_PROCESS_ERR_CERT_REQ

/// Error processing Hash payload
pub const IKE_PROCESS_ERR_HASH : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(13837); // ERROR_IPSEC_IKE_PROCESS_ERR_HASH

/// Error processing Signature payload
pub const IKE_PROCESS_ERR_SIG : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(13838); // ERROR_IPSEC_IKE_PROCESS_ERR_SIG

/// Error processing Nonce payload
pub const IKE_PROCESS_ERR_NONCE : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(13839); // ERROR_IPSEC_IKE_PROCESS_ERR_NONCE

/// Error processing Notify payload
pub const IKE_PROCESS_ERR_NOTIFY : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(13840); // ERROR_IPSEC_IKE_PROCESS_ERR_NOTIFY

/// Error processing Delete Payload
pub const IKE_PROCESS_ERR_DELETE : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(13841); // ERROR_IPSEC_IKE_PROCESS_ERR_DELETE

/// Error processing VendorId payload
pub const IKE_PROCESS_ERR_VENDOR : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(13842); // ERROR_IPSEC_IKE_PROCESS_ERR_VENDOR

/// Invalid payload received
pub const IKE_INVALID_PAYLOAD : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(13843); // ERROR_IPSEC_IKE_INVALID_PAYLOAD

/// Soft SA loaded
pub const IKE_LOAD_SOFT_SA : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(13844); // ERROR_IPSEC_IKE_LOAD_SOFT_SA

/// Soft SA torn down
pub const IKE_SOFT_SA_TORN_DOWN : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(13845); // ERROR_IPSEC_IKE_SOFT_SA_TORN_DOWN

/// Invalid cookie received.
pub const IKE_INVALID_COOKIE : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(13846); // ERROR_IPSEC_IKE_INVALID_COOKIE

/// Peer failed to send valid machine certificate
pub const IKE_NO_PEER_CERT : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(13847); // ERROR_IPSEC_IKE_NO_PEER_CERT

/// Certification Revocation check of peer's certificate failed
pub const IKE_PEER_CRL_FAILED : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(13848); // ERROR_IPSEC_IKE_PEER_CRL_FAILED

/// New policy invalidated SAs formed with old policy
pub const IKE_POLICY_CHANGE : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(13849); // ERROR_IPSEC_IKE_POLICY_CHANGE

/// There is no available Main Mode IKE policy.
pub const IKE_NO_MM_POLICY : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(13850); // ERROR_IPSEC_IKE_NO_MM_POLICY

/// Failed to enabled TCB privilege.
pub const IKE_NOTCBPRIV : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(13851); // ERROR_IPSEC_IKE_NOTCBPRIV

/// Failed to load SECURITY.DLL.
pub const IKE_SECLOADFAIL : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(13852); // ERROR_IPSEC_IKE_SECLOADFAIL

/// Failed to obtain security function table dispatch address from SSPI.
pub const IKE_FAILSSPINIT : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(13853); // ERROR_IPSEC_IKE_FAILSSPINIT

/// Failed to query Kerberos package to obtain max token size.
pub const IKE_FAILQUERYSSP : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(13854); // ERROR_IPSEC_IKE_FAILQUERYSSP

/// Failed to obtain Kerberos server credentials for ISAKMP/ERROR_IPSEC_IKE service. Kerberos authentication will not function. The most likely reason for this is lack of domain membership. This is normal if your computer is a member of a workgroup.
pub const IKE_SRVACQFAIL : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(13855); // ERROR_IPSEC_IKE_SRVACQFAIL

/// Failed to determine SSPI principal name for ISAKMP/ERROR_IPSEC_IKE service (QueryCredentialsAttributes).
pub const IKE_SRVQUERYCRED : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(13856); // ERROR_IPSEC_IKE_SRVQUERYCRED

/// Failed to obtain new SPI for the inbound SA from IPsec driver. The most common cause for this is that the driver does not have the correct filter. Check your policy to verify the filters.
pub const IKE_GETSPIFAIL : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(13857); // ERROR_IPSEC_IKE_GETSPIFAIL

/// Given filter is invalid
pub const IKE_INVALID_FILTER : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(13858); // ERROR_IPSEC_IKE_INVALID_FILTER

/// Memory allocation failed.
pub const IKE_OUT_OF_MEMORY : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(13859); // ERROR_IPSEC_IKE_OUT_OF_MEMORY

/// Failed to add Security Association to IPsec Driver. The most common cause for this is if the IKE negotiation took too long to complete. If the problem persists, reduce the load on the faulting machine.
pub const IKE_ADD_UPDATE_KEY_FAILED : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(13860); // ERROR_IPSEC_IKE_ADD_UPDATE_KEY_FAILED

/// Invalid policy
pub const IKE_INVALID_POLICY : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(13861); // ERROR_IPSEC_IKE_INVALID_POLICY

/// Invalid DOI
pub const IKE_UNKNOWN_DOI : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(13862); // ERROR_IPSEC_IKE_UNKNOWN_DOI

/// Invalid situation
pub const IKE_INVALID_SITUATION : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(13863); // ERROR_IPSEC_IKE_INVALID_SITUATION

/// Diffie-Hellman failure
pub const IKE_DH_FAILURE : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(13864); // ERROR_IPSEC_IKE_DH_FAILURE

/// Invalid Diffie-Hellman group
pub const IKE_INVALID_GROUP : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(13865); // ERROR_IPSEC_IKE_INVALID_GROUP

/// Error encrypting payload
pub const IKE_ENCRYPT : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(13866); // ERROR_IPSEC_IKE_ENCRYPT

/// Error decrypting payload
pub const IKE_DECRYPT : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(13867); // ERROR_IPSEC_IKE_DECRYPT

/// Policy match error
pub const IKE_POLICY_MATCH : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(13868); // ERROR_IPSEC_IKE_POLICY_MATCH

/// Unsupported ID
pub const IKE_UNSUPPORTED_ID : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(13869); // ERROR_IPSEC_IKE_UNSUPPORTED_ID

/// Hash verification failed
pub const IKE_INVALID_HASH : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(13870); // ERROR_IPSEC_IKE_INVALID_HASH

/// Invalid hash algorithm
pub const IKE_INVALID_HASH_ALG : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(13871); // ERROR_IPSEC_IKE_INVALID_HASH_ALG

/// Invalid hash size
pub const IKE_INVALID_HASH_SIZE : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(13872); // ERROR_IPSEC_IKE_INVALID_HASH_SIZE

/// Invalid encryption algorithm
pub const IKE_INVALID_ENCRYPT_ALG : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(13873); // ERROR_IPSEC_IKE_INVALID_ENCRYPT_ALG

/// Invalid authentication algorithm
pub const IKE_INVALID_AUTH_ALG : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(13874); // ERROR_IPSEC_IKE_INVALID_AUTH_ALG

/// Invalid certificate signature
pub const IKE_INVALID_SIG : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(13875); // ERROR_IPSEC_IKE_INVALID_SIG

/// Load failed
pub const IKE_LOAD_FAILED : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(13876); // ERROR_IPSEC_IKE_LOAD_FAILED

/// Deleted via RPC call
pub const IKE_RPC_DELETE : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(13877); // ERROR_IPSEC_IKE_RPC_DELETE

/// Temporary state created to perform reinitialization. This is not a real failure.
pub const IKE_BENIGN_REINIT : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(13878); // ERROR_IPSEC_IKE_BENIGN_REINIT

/// The lifetime value received in the Responder Lifetime Notify is below the Windows 2000 configured minimum value. Please fix the policy on the peer machine.
pub const IKE_INVALID_RESPONDER_LIFETIME_NOTIFY : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(13879); // ERROR_IPSEC_IKE_INVALID_RESPONDER_LIFETIME_NOTIFY

/// The recipient cannot handle version of IKE specified in the header.
pub const IKE_INVALID_MAJOR_VERSION : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(13880); // ERROR_IPSEC_IKE_INVALID_MAJOR_VERSION

/// Key length in certificate is too small for configured security requirements.
pub const IKE_INVALID_CERT_KEYLEN : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(13881); // ERROR_IPSEC_IKE_INVALID_CERT_KEYLEN

/// Max number of established MM SAs to peer exceeded.
pub const IKE_MM_LIMIT : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(13882); // ERROR_IPSEC_IKE_MM_LIMIT

/// IKE received a policy that disables negotiation.
pub const IKE_NEGOTIATION_DISABLED : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(13883); // ERROR_IPSEC_IKE_NEGOTIATION_DISABLED

/// Reached maximum quick mode limit for the main mode. New main mode will be started.
pub const IKE_QM_LIMIT : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(13884); // ERROR_IPSEC_IKE_QM_LIMIT

/// Main mode SA lifetime expired or peer sent a main mode delete.
pub const IKE_MM_EXPIRED : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(13885); // ERROR_IPSEC_IKE_MM_EXPIRED

/// Main mode SA assumed to be invalid because peer stopped responding.
pub const IKE_PEER_MM_ASSUMED_INVALID : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(13886); // ERROR_IPSEC_IKE_PEER_MM_ASSUMED_INVALID

/// Certificate doesn't chain to a trusted root in IPsec policy.
pub const IKE_CERT_CHAIN_POLICY_MISMATCH : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(13887); // ERROR_IPSEC_IKE_CERT_CHAIN_POLICY_MISMATCH

/// Received unexpected message ID.
pub const IKE_UNEXPECTED_MESSAGE_ID : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(13888); // ERROR_IPSEC_IKE_UNEXPECTED_MESSAGE_ID

/// Received invalid authentication offers.
pub const IKE_INVALID_AUTH_PAYLOAD : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(13889); // ERROR_IPSEC_IKE_INVALID_AUTH_PAYLOAD

/// Sent DoS cookie notify to initiator.
pub const IKE_DOS_COOKIE_SENT : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(13890); // ERROR_IPSEC_IKE_DOS_COOKIE_SENT

/// IKE service is shutting down.
pub const IKE_SHUTTING_DOWN : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(13891); // ERROR_IPSEC_IKE_SHUTTING_DOWN

/// Could not verify binding between CGA address and certificate.
pub const IKE_CGA_AUTH_FAILED : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(13892); // ERROR_IPSEC_IKE_CGA_AUTH_FAILED

/// Error processing NatOA payload.
pub const IKE_PROCESS_ERR_NATOA : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(13893); // ERROR_IPSEC_IKE_PROCESS_ERR_NATOA

/// Parameters of the main mode are invalid for this quick mode.
pub const IKE_INVALID_MM_FOR_QM : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(13894); // ERROR_IPSEC_IKE_INVALID_MM_FOR_QM

/// Quick mode SA was expired by IPsec driver.
pub const IKE_QM_EXPIRED : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(13895); // ERROR_IPSEC_IKE_QM_EXPIRED

/// Too many dynamically added IKEEXT filters were detected.
pub const IKE_TOO_MANY_FILTERS : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(13896); // ERROR_IPSEC_IKE_TOO_MANY_FILTERS

/// NAP reauth succeeded and must delete the dummy NAP IKEv2 tunnel.
pub const IKE_KILL_DUMMY_NAP_TUNNEL : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(13898); // ERROR_IPSEC_IKE_KILL_DUMMY_NAP_TUNNEL

/// Error in assigning inner IP address to initiator in tunnel mode.
pub const IKE_INNER_IP_ASSIGNMENT_FAILURE : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(13899); // ERROR_IPSEC_IKE_INNER_IP_ASSIGNMENT_FAILURE

/// Require configuration payload missing.
pub const IKE_REQUIRE_CP_PAYLOAD_MISSING : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(13900); // ERROR_IPSEC_IKE_REQUIRE_CP_PAYLOAD_MISSING

/// A negotiation running as the security principle who issued the connection is in progress
pub const KEY_MODULE_IMPERSONATION_NEGOTIATION_PENDING : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(13901); // ERROR_IPSEC_KEY_MODULE_IMPERSONATION_NEGOTIATION_PENDING

/// SA was deleted due to IKEv1/AuthIP co-existence suppress check.
pub const IKE_COEXISTENCE_SUPPRESS : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(13902); // ERROR_IPSEC_IKE_COEXISTENCE_SUPPRESS

/// Incoming SA request was dropped due to peer IP address rate limiting.
pub const IKE_RATELIMIT_DROP : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(13903); // ERROR_IPSEC_IKE_RATELIMIT_DROP

/// Peer does not support MOBIKE.
pub const IKE_PEER_DOESNT_SUPPORT_MOBIKE : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(13904); // ERROR_IPSEC_IKE_PEER_DOESNT_SUPPORT_MOBIKE

/// SA establishment is not authorized.
pub const IKE_AUTHORIZATION_FAILURE : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(13905); // ERROR_IPSEC_IKE_AUTHORIZATION_FAILURE

/// SA establishment is not authorized because there is not a sufficiently strong PKINIT-based credential.
pub const IKE_STRONG_CRED_AUTHORIZATION_FAILURE : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(13906); // ERROR_IPSEC_IKE_STRONG_CRED_AUTHORIZATION_FAILURE

/// SA establishment is not authorized.  You may need to enter updated or different credentials such as a smartcard.
pub const IKE_AUTHORIZATION_FAILURE_WITH_OPTIONAL_RETRY : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(13907); // ERROR_IPSEC_IKE_AUTHORIZATION_FAILURE_WITH_OPTIONAL_RETRY

/// SA establishment is not authorized because there is not a sufficiently strong PKINIT-based credential. This might be related to certificate-to-account mapping failure for the SA.
pub const IKE_STRONG_CRED_AUTHORIZATION_AND_CERTMAP_FAILURE : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(13908); // ERROR_IPSEC_IKE_STRONG_CRED_AUTHORIZATION_AND_CERTMAP_FAILURE

/// The SPI in the packet does not match a valid IPsec SA.
pub const BAD_SPI : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(13910); // ERROR_IPSEC_BAD_SPI

/// Packet was received on an IPsec SA whose lifetime has expired.
pub const SA_LIFETIME_EXPIRED : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(13911); // ERROR_IPSEC_SA_LIFETIME_EXPIRED

/// Packet was received on an IPsec SA that does not match the packet characteristics.
pub const WRONG_SA : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(13912); // ERROR_IPSEC_WRONG_SA

/// Packet sequence number replay check failed.
pub const REPLAY_CHECK_FAILED : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(13913); // ERROR_IPSEC_REPLAY_CHECK_FAILED

/// IPsec header and/or trailer in the packet is invalid.
pub const INVALID_PACKET : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(13914); // ERROR_IPSEC_INVALID_PACKET

/// IPsec integrity check failed.
pub const INTEGRITY_CHECK_FAILED : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(13915); // ERROR_IPSEC_INTEGRITY_CHECK_FAILED

/// IPsec dropped a clear text packet.
pub const CLEAR_TEXT_DROP : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(13916); // ERROR_IPSEC_CLEAR_TEXT_DROP

/// IPsec dropped an incoming ESP packet in authenticated firewall mode. This drop is benign.
pub const AUTH_FIREWALL_DROP : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(13917); // ERROR_IPSEC_AUTH_FIREWALL_DROP

/// IPsec dropped a packet due to DoS throttling.
pub const THROTTLE_DROP : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(13918); // ERROR_IPSEC_THROTTLE_DROP

/// IPsec DoS Protection matched an explicit block rule.
pub const DOSP_BLOCK : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(13925); // ERROR_IPSEC_DOSP_BLOCK

/// IPsec DoS Protection received an IPsec specific multicast packet which is not allowed.
pub const DOSP_RECEIVED_MULTICAST : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(13926); // ERROR_IPSEC_DOSP_RECEIVED_MULTICAST

/// IPsec DoS Protection received an incorrectly formatted packet.
pub const DOSP_INVALID_PACKET : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(13927); // ERROR_IPSEC_DOSP_INVALID_PACKET

/// IPsec DoS Protection failed to look up state.
pub const DOSP_STATE_LOOKUP_FAILED : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(13928); // ERROR_IPSEC_DOSP_STATE_LOOKUP_FAILED

/// IPsec DoS Protection failed to create state because the maximum number of entries allowed by policy has been reached.
pub const DOSP_MAX_ENTRIES : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(13929); // ERROR_IPSEC_DOSP_MAX_ENTRIES

/// IPsec DoS Protection received an IPsec negotiation packet for a keying module which is not allowed by policy.
pub const DOSP_KEYMOD_NOT_ALLOWED : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(13930); // ERROR_IPSEC_DOSP_KEYMOD_NOT_ALLOWED

/// IPsec DoS Protection has not been enabled.
pub const DOSP_NOT_INSTALLED : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(13931); // ERROR_IPSEC_DOSP_NOT_INSTALLED

/// IPsec DoS Protection failed to create a per internal IP rate limit queue because the maximum number of queues allowed by policy has been reached.
pub const DOSP_MAX_PER_IP_RATELIMIT_QUEUES : ErrorCodeMicrosoft = ErrorCodeMicrosoft::from_constant(13932); // ERROR_IPSEC_DOSP_MAX_PER_IP_RATELIMIT_QUEUES
