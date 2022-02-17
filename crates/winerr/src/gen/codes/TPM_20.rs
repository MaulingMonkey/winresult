// WARNING: this file is auto-generated by xtask gen and may be overwritten

use super::*;


/// TPM 2.0: Asymmetric algorithm not supported or not correct.
pub const E_ASYMMETRIC : ErrorHResult = ErrorHResult::from_constant(0x80280081); // TPM_20_E_ASYMMETRIC

/// TPM 2.0: Inconsistent attributes.
pub const E_ATTRIBUTES : ErrorHResult = ErrorHResult::from_constant(0x80280082); // TPM_20_E_ATTRIBUTES

/// TPM 2.0: Hash algorithm not supported or not appropriate.
pub const E_HASH : ErrorHResult = ErrorHResult::from_constant(0x80280083); // TPM_20_E_HASH

/// TPM 2.0: Value is out of range or is not correct for the context.
pub const E_VALUE : ErrorHResult = ErrorHResult::from_constant(0x80280084); // TPM_20_E_VALUE

/// TPM 2.0: Hierarchy is not enabled or is not correct for the use.
pub const E_HIERARCHY : ErrorHResult = ErrorHResult::from_constant(0x80280085); // TPM_20_E_HIERARCHY

/// TPM 2.0: Key size is not supported.
pub const E_KEY_SIZE : ErrorHResult = ErrorHResult::from_constant(0x80280087); // TPM_20_E_KEY_SIZE

/// TPM 2.0: Mask generation function not supported.
pub const E_MGF : ErrorHResult = ErrorHResult::from_constant(0x80280088); // TPM_20_E_MGF

/// TPM 2.0: Mode of operation not supported.
pub const E_MODE : ErrorHResult = ErrorHResult::from_constant(0x80280089); // TPM_20_E_MODE

/// TPM 2.0: The type of the value is not appropriate for the use.
pub const E_TYPE : ErrorHResult = ErrorHResult::from_constant(0x8028008A); // TPM_20_E_TYPE

/// TPM 2.0: The Handle is not correct for the use.
pub const E_HANDLE : ErrorHResult = ErrorHResult::from_constant(0x8028008B); // TPM_20_E_HANDLE

/// TPM 2.0: Unsupported key derivation function or function not appropriate for use.
pub const E_KDF : ErrorHResult = ErrorHResult::from_constant(0x8028008C); // TPM_20_E_KDF

/// TPM 2.0: Value was out of allowed range.
pub const E_RANGE : ErrorHResult = ErrorHResult::from_constant(0x8028008D); // TPM_20_E_RANGE

/// TPM 2.0: The authorization HMAC check failed and DA counter incremented.
pub const E_AUTH_FAIL : ErrorHResult = ErrorHResult::from_constant(0x8028008E); // TPM_20_E_AUTH_FAIL

/// TPM 2.0: Invalid nonce size.
pub const E_NONCE : ErrorHResult = ErrorHResult::from_constant(0x8028008F); // TPM_20_E_NONCE

/// TPM 2.0: Authorization requires assertion of PP.
pub const E_PP : ErrorHResult = ErrorHResult::from_constant(0x80280090); // TPM_20_E_PP

/// TPM 2.0: Unsupported or incompatible scheme.
pub const E_SCHEME : ErrorHResult = ErrorHResult::from_constant(0x80280092); // TPM_20_E_SCHEME

/// TPM 2.0: Structure is wrong size.
pub const E_SIZE : ErrorHResult = ErrorHResult::from_constant(0x80280095); // TPM_20_E_SIZE

/// TPM 2.0: Unsupported symmetric algorithm or key size, or not appropriate for instance.
pub const E_SYMMETRIC : ErrorHResult = ErrorHResult::from_constant(0x80280096); // TPM_20_E_SYMMETRIC

/// TPM 2.0: Incorrect structure tag.
pub const E_TAG : ErrorHResult = ErrorHResult::from_constant(0x80280097); // TPM_20_E_TAG

/// TPM 2.0: Union selector is incorrect.
pub const E_SELECTOR : ErrorHResult = ErrorHResult::from_constant(0x80280098); // TPM_20_E_SELECTOR

/// TPM 2.0: The TPM was unable to unmarshal a value because there were not enough octets in the input buffer.
pub const E_INSUFFICIENT : ErrorHResult = ErrorHResult::from_constant(0x8028009A); // TPM_20_E_INSUFFICIENT

/// TPM 2.0: The signature is not valid.
pub const E_SIGNATURE : ErrorHResult = ErrorHResult::from_constant(0x8028009B); // TPM_20_E_SIGNATURE

/// TPM 2.0: Key fields are not compatible with the selected use.
pub const E_KEY : ErrorHResult = ErrorHResult::from_constant(0x8028009C); // TPM_20_E_KEY

/// TPM 2.0: A policy check failed.
pub const E_POLICY_FAIL : ErrorHResult = ErrorHResult::from_constant(0x8028009D); // TPM_20_E_POLICY_FAIL

/// TPM 2.0: Integrity check failed.
pub const E_INTEGRITY : ErrorHResult = ErrorHResult::from_constant(0x8028009F); // TPM_20_E_INTEGRITY

/// TPM 2.0: Invalid ticket.
pub const E_TICKET : ErrorHResult = ErrorHResult::from_constant(0x802800A0); // TPM_20_E_TICKET

/// TPM 2.0: Reserved bits not set to zero as required.
pub const E_RESERVED_BITS : ErrorHResult = ErrorHResult::from_constant(0x802800A1); // TPM_20_E_RESERVED_BITS

/// TPM 2.0: Authorization failure without DA implications.
pub const E_BAD_AUTH : ErrorHResult = ErrorHResult::from_constant(0x802800A2); // TPM_20_E_BAD_AUTH

/// TPM 2.0: The policy has expired.
pub const E_EXPIRED : ErrorHResult = ErrorHResult::from_constant(0x802800A3); // TPM_20_E_EXPIRED

/// TPM 2.0: The command code in the policy is not the command code of the command or the command code in a policy command references a command that is not implemented.
pub const E_POLICY_CC : ErrorHResult = ErrorHResult::from_constant(0x802800A4); // TPM_20_E_POLICY_CC

/// TPM 2.0: Public and sensitive portions of an object are not cryptographically bound.
pub const E_BINDING : ErrorHResult = ErrorHResult::from_constant(0x802800A5); // TPM_20_E_BINDING

/// TPM 2.0: Curve not supported.
pub const E_CURVE : ErrorHResult = ErrorHResult::from_constant(0x802800A6); // TPM_20_E_CURVE

/// TPM 2.0: Point is not on the required curve.
pub const E_ECC_POINT : ErrorHResult = ErrorHResult::from_constant(0x802800A7); // TPM_20_E_ECC_POINT

/// TPM 2.0: TPM not initialized.
pub const E_INITIALIZE : ErrorHResult = ErrorHResult::from_constant(0x80280100); // TPM_20_E_INITIALIZE

/// TPM 2.0: Commands not being accepted because of a TPM failure.
pub const E_FAILURE : ErrorHResult = ErrorHResult::from_constant(0x80280101); // TPM_20_E_FAILURE

/// TPM 2.0: Improper use of a sequence handle.
pub const E_SEQUENCE : ErrorHResult = ErrorHResult::from_constant(0x80280103); // TPM_20_E_SEQUENCE

/// TPM 2.0: TPM_RC_PRIVATE error.
pub const E_PRIVATE : ErrorHResult = ErrorHResult::from_constant(0x8028010B); // TPM_20_E_PRIVATE

/// TPM 2.0: TPM_RC_HMAC.
pub const E_HMAC : ErrorHResult = ErrorHResult::from_constant(0x80280119); // TPM_20_E_HMAC

/// TPM 2.0: TPM_RC_DISABLED.
pub const E_DISABLED : ErrorHResult = ErrorHResult::from_constant(0x80280120); // TPM_20_E_DISABLED

/// TPM 2.0: Command failed because audit sequence required exclusivity.
pub const E_EXCLUSIVE : ErrorHResult = ErrorHResult::from_constant(0x80280121); // TPM_20_E_EXCLUSIVE

/// TPM 2.0: Unsupported ECC curve.
pub const E_ECC_CURVE : ErrorHResult = ErrorHResult::from_constant(0x80280123); // TPM_20_E_ECC_CURVE

/// TPM 2.0: Authorization handle is not correct for command.
pub const E_AUTH_TYPE : ErrorHResult = ErrorHResult::from_constant(0x80280124); // TPM_20_E_AUTH_TYPE

/// TPM 2.0: Command requires an authorization session for handle and is not present.
pub const E_AUTH_MISSING : ErrorHResult = ErrorHResult::from_constant(0x80280125); // TPM_20_E_AUTH_MISSING

/// TPM 2.0: Policy failure in Math Operation or an invalid authPolicy value.
pub const E_POLICY : ErrorHResult = ErrorHResult::from_constant(0x80280126); // TPM_20_E_POLICY

/// TPM 2.0: PCR check fail.
pub const E_PCR : ErrorHResult = ErrorHResult::from_constant(0x80280127); // TPM_20_E_PCR

/// TPM 2.0: PCR have changed since checked.
pub const E_PCR_CHANGED : ErrorHResult = ErrorHResult::from_constant(0x80280128); // TPM_20_E_PCR_CHANGED

/// TPM 2.0: The TPM is not in the right mode for upgrade.
pub const E_UPGRADE : ErrorHResult = ErrorHResult::from_constant(0x8028012D); // TPM_20_E_UPGRADE

/// TPM 2.0: Context ID counter is at maximum.
pub const E_TOO_MANY_CONTEXTS : ErrorHResult = ErrorHResult::from_constant(0x8028012E); // TPM_20_E_TOO_MANY_CONTEXTS

/// TPM 2.0: authValue or authPolicy is not available for selected entity.
pub const E_AUTH_UNAVAILABLE : ErrorHResult = ErrorHResult::from_constant(0x8028012F); // TPM_20_E_AUTH_UNAVAILABLE

/// TPM 2.0: A _TPM_Init and Startup(CLEAR) is required before the TPM can resume operation.
pub const E_REBOOT : ErrorHResult = ErrorHResult::from_constant(0x80280130); // TPM_20_E_REBOOT

/// TPM 2.0: The protection algorithms (hash and symmetric) are not reasonably balanced. The digest size of the hash must be larger than the key size of the symmetric algorithm.
pub const E_UNBALANCED : ErrorHResult = ErrorHResult::from_constant(0x80280131); // TPM_20_E_UNBALANCED

/// TPM 2.0: The TPM command's commandSize value is inconsistent with contents of the command buffer; either the size is not the same as the bytes loaded by the hardware interface layer or the value is not large enough to hold a command header.
pub const E_COMMAND_SIZE : ErrorHResult = ErrorHResult::from_constant(0x80280142); // TPM_20_E_COMMAND_SIZE

/// TPM 2.0: Command code not supported.
pub const E_COMMAND_CODE : ErrorHResult = ErrorHResult::from_constant(0x80280143); // TPM_20_E_COMMAND_CODE

/// TPM 2.0: The value of authorizationSize is out of range or the number of octets in the authorization Area is greater than required.
pub const E_AUTHSIZE : ErrorHResult = ErrorHResult::from_constant(0x80280144); // TPM_20_E_AUTHSIZE

/// TPM 2.0: Use of an authorization session with a context command or another command that cannot have an authorization session.
pub const E_AUTH_CONTEXT : ErrorHResult = ErrorHResult::from_constant(0x80280145); // TPM_20_E_AUTH_CONTEXT

/// TPM 2.0: NV offset+size is out of range.
pub const E_NV_RANGE : ErrorHResult = ErrorHResult::from_constant(0x80280146); // TPM_20_E_NV_RANGE

/// TPM 2.0: Requested allocation size is larger than allowed.
pub const E_NV_SIZE : ErrorHResult = ErrorHResult::from_constant(0x80280147); // TPM_20_E_NV_SIZE

/// TPM 2.0: NV access locked.
pub const E_NV_LOCKED : ErrorHResult = ErrorHResult::from_constant(0x80280148); // TPM_20_E_NV_LOCKED

/// TPM 2.0: NV access authorization fails in command actions
pub const E_NV_AUTHORIZATION : ErrorHResult = ErrorHResult::from_constant(0x80280149); // TPM_20_E_NV_AUTHORIZATION

/// TPM 2.0: An NV index is used before being initialized or the state saved by TPM2_Shutdown(STATE) could not be restored.
pub const E_NV_UNINITIALIZED : ErrorHResult = ErrorHResult::from_constant(0x8028014A); // TPM_20_E_NV_UNINITIALIZED

/// TPM 2.0: Insufficient space for NV allocation.
pub const E_NV_SPACE : ErrorHResult = ErrorHResult::from_constant(0x8028014B); // TPM_20_E_NV_SPACE

/// TPM 2.0: NV index or persistent object already defined.
pub const E_NV_DEFINED : ErrorHResult = ErrorHResult::from_constant(0x8028014C); // TPM_20_E_NV_DEFINED

/// TPM 2.0: Context in TPM2_ContextLoad() is not valid.
pub const E_BAD_CONTEXT : ErrorHResult = ErrorHResult::from_constant(0x80280150); // TPM_20_E_BAD_CONTEXT

/// TPM 2.0: chHash value already set or not correct for use.
pub const E_CPHASH : ErrorHResult = ErrorHResult::from_constant(0x80280151); // TPM_20_E_CPHASH

/// TPM 2.0: Handle for parent is not a valid parent.
pub const E_PARENT : ErrorHResult = ErrorHResult::from_constant(0x80280152); // TPM_20_E_PARENT

/// TPM 2.0: Some function needs testing.
pub const E_NEEDS_TEST : ErrorHResult = ErrorHResult::from_constant(0x80280153); // TPM_20_E_NEEDS_TEST

/// TPM 2.0: returned when an internal function cannot process a request due to an unspecified problem. This code is usually related to invalid parameters that are not properly filtered by the input unmarshaling code.
pub const E_NO_RESULT : ErrorHResult = ErrorHResult::from_constant(0x80280154); // TPM_20_E_NO_RESULT

/// TPM 2.0: The sensitive area did not unmarshal correctly after decryption - this code is used in lieu of the other unmarshaling errors so that an attacker cannot determine where the unmarshaling error occurred.
pub const E_SENSITIVE : ErrorHResult = ErrorHResult::from_constant(0x80280155); // TPM_20_E_SENSITIVE

/// TPM 2.0: Gap for context ID is too large.
pub const E_CONTEXT_GAP : ErrorHResult = ErrorHResult::from_constant(0x80280901); // TPM_20_E_CONTEXT_GAP

/// TPM 2.0: Out of memory for object contexts.
pub const E_OBJECT_MEMORY : ErrorHResult = ErrorHResult::from_constant(0x80280902); // TPM_20_E_OBJECT_MEMORY

/// TPM 2.0: Out of memory for session contexts.
pub const E_SESSION_MEMORY : ErrorHResult = ErrorHResult::from_constant(0x80280903); // TPM_20_E_SESSION_MEMORY

/// TPM 2.0: Out of shared object/session memory or need space for internal operations.
pub const E_MEMORY : ErrorHResult = ErrorHResult::from_constant(0x80280904); // TPM_20_E_MEMORY

/// TPM 2.0: Out of session handles - a session must be flushed before a nes session may be created.
pub const E_SESSION_HANDLES : ErrorHResult = ErrorHResult::from_constant(0x80280905); // TPM_20_E_SESSION_HANDLES

/// TPM 2.0: Out of object handles - the handle space for objects is depleted and a reboot is required.
pub const E_OBJECT_HANDLES : ErrorHResult = ErrorHResult::from_constant(0x80280906); // TPM_20_E_OBJECT_HANDLES

/// TPM 2.0: Bad locality.
pub const E_LOCALITY : ErrorHResult = ErrorHResult::from_constant(0x80280907); // TPM_20_E_LOCALITY

/// TPM 2.0: The TPM has suspended operation on the command; forward progress was made and the command may be retried.
pub const E_YIELDED : ErrorHResult = ErrorHResult::from_constant(0x80280908); // TPM_20_E_YIELDED

/// TPM 2.0: The command was canceled.
pub const E_CANCELED : ErrorHResult = ErrorHResult::from_constant(0x80280909); // TPM_20_E_CANCELED

/// TPM 2.0: TPM is performing self-tests.
pub const E_TESTING : ErrorHResult = ErrorHResult::from_constant(0x8028090A); // TPM_20_E_TESTING

/// TPM 2.0: The TPM is rate-limiting accesses to prevent wearout of NV
pub const E_NV_RATE : ErrorHResult = ErrorHResult::from_constant(0x80280920); // TPM_20_E_NV_RATE

/// TPM 2.0: Authorization for objects subject to DA protection are not allowed at this time because the TPM is in DA lockout mode.
pub const E_LOCKOUT : ErrorHResult = ErrorHResult::from_constant(0x80280921); // TPM_20_E_LOCKOUT

/// TPM 2.0: The TPM was not able to start the command.
pub const E_RETRY : ErrorHResult = ErrorHResult::from_constant(0x80280922); // TPM_20_E_RETRY

/// TPM 2.0: the command may require writing of NV and NV is not current accessible.
pub const E_NV_UNAVAILABLE : ErrorHResult = ErrorHResult::from_constant(0x80280923); // TPM_20_E_NV_UNAVAILABLE
