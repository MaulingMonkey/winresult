// WARNING: this file is auto-generated by xtask gen and may be overwritten

use super::*;


/// The requested section was not present in the activation context.
pub const SECTION_NOT_FOUND : ErrorCode = ErrorCode::from_constant(14000); // ERROR_SXS_SECTION_NOT_FOUND

/// The application has failed to start because its side-by-side configuration is incorrect. Please see the application event log or use the command-line sxstrace.exe tool for more detail.
pub const CANT_GEN_ACTCTX : ErrorCode = ErrorCode::from_constant(14001); // ERROR_SXS_CANT_GEN_ACTCTX

/// The application binding data format is invalid.
pub const INVALID_ACTCTXDATA_FORMAT : ErrorCode = ErrorCode::from_constant(14002); // ERROR_SXS_INVALID_ACTCTXDATA_FORMAT

/// The referenced assembly is not installed on your system.
pub const ASSEMBLY_NOT_FOUND : ErrorCode = ErrorCode::from_constant(14003); // ERROR_SXS_ASSEMBLY_NOT_FOUND

/// The manifest file does not begin with the required tag and format information.
pub const MANIFEST_FORMAT_ERROR : ErrorCode = ErrorCode::from_constant(14004); // ERROR_SXS_MANIFEST_FORMAT_ERROR

/// The manifest file contains one or more syntax errors.
pub const MANIFEST_PARSE_ERROR : ErrorCode = ErrorCode::from_constant(14005); // ERROR_SXS_MANIFEST_PARSE_ERROR

/// The application attempted to activate a disabled activation context.
pub const ACTIVATION_CONTEXT_DISABLED : ErrorCode = ErrorCode::from_constant(14006); // ERROR_SXS_ACTIVATION_CONTEXT_DISABLED

/// The requested lookup key was not found in any active activation context.
pub const KEY_NOT_FOUND : ErrorCode = ErrorCode::from_constant(14007); // ERROR_SXS_KEY_NOT_FOUND

/// A component version required by the application conflicts with another component version already active.
pub const VERSION_CONFLICT : ErrorCode = ErrorCode::from_constant(14008); // ERROR_SXS_VERSION_CONFLICT

/// The type requested activation context section does not match the query API used.
pub const WRONG_SECTION_TYPE : ErrorCode = ErrorCode::from_constant(14009); // ERROR_SXS_WRONG_SECTION_TYPE

/// Lack of system resources has required isolated activation to be disabled for the current thread of execution.
pub const THREAD_QUERIES_DISABLED : ErrorCode = ErrorCode::from_constant(14010); // ERROR_SXS_THREAD_QUERIES_DISABLED

/// An attempt to set the process default activation context failed because the process default activation context was already set.
pub const PROCESS_DEFAULT_ALREADY_SET : ErrorCode = ErrorCode::from_constant(14011); // ERROR_SXS_PROCESS_DEFAULT_ALREADY_SET

/// The encoding group identifier specified is not recognized.
pub const UNKNOWN_ENCODING_GROUP : ErrorCode = ErrorCode::from_constant(14012); // ERROR_SXS_UNKNOWN_ENCODING_GROUP

/// The encoding requested is not recognized.
pub const UNKNOWN_ENCODING : ErrorCode = ErrorCode::from_constant(14013); // ERROR_SXS_UNKNOWN_ENCODING

/// The manifest contains a reference to an invalid URI.
pub const INVALID_XML_NAMESPACE_URI : ErrorCode = ErrorCode::from_constant(14014); // ERROR_SXS_INVALID_XML_NAMESPACE_URI

/// The application manifest contains a reference to a dependent assembly which is not installed
pub const ROOT_MANIFEST_DEPENDENCY_NOT_INSTALLED : ErrorCode = ErrorCode::from_constant(14015); // ERROR_SXS_ROOT_MANIFEST_DEPENDENCY_NOT_INSTALLED

/// The manifest for an assembly used by the application has a reference to a dependent assembly which is not installed
pub const LEAF_MANIFEST_DEPENDENCY_NOT_INSTALLED : ErrorCode = ErrorCode::from_constant(14016); // ERROR_SXS_LEAF_MANIFEST_DEPENDENCY_NOT_INSTALLED

/// The manifest contains an attribute for the assembly identity which is not valid.
pub const INVALID_ASSEMBLY_IDENTITY_ATTRIBUTE : ErrorCode = ErrorCode::from_constant(14017); // ERROR_SXS_INVALID_ASSEMBLY_IDENTITY_ATTRIBUTE

/// The manifest is missing the required default namespace specification on the assembly element.
pub const MANIFEST_MISSING_REQUIRED_DEFAULT_NAMESPACE : ErrorCode = ErrorCode::from_constant(14018); // ERROR_SXS_MANIFEST_MISSING_REQUIRED_DEFAULT_NAMESPACE

/// The manifest has a default namespace specified on the assembly element but its value is not "urn:schemas-microsoft-com:asm.v1".
pub const MANIFEST_INVALID_REQUIRED_DEFAULT_NAMESPACE : ErrorCode = ErrorCode::from_constant(14019); // ERROR_SXS_MANIFEST_INVALID_REQUIRED_DEFAULT_NAMESPACE

/// The private manifest probed has crossed a path with an unsupported reparse point.
pub const PRIVATE_MANIFEST_CROSS_PATH_WITH_REPARSE_POINT : ErrorCode = ErrorCode::from_constant(14020); // ERROR_SXS_PRIVATE_MANIFEST_CROSS_PATH_WITH_REPARSE_POINT

/// Two or more components referenced directly or indirectly by the application manifest have files by the same name.
pub const DUPLICATE_DLL_NAME : ErrorCode = ErrorCode::from_constant(14021); // ERROR_SXS_DUPLICATE_DLL_NAME

/// Two or more components referenced directly or indirectly by the application manifest have window classes with the same name.
pub const DUPLICATE_WINDOWCLASS_NAME : ErrorCode = ErrorCode::from_constant(14022); // ERROR_SXS_DUPLICATE_WINDOWCLASS_NAME

/// Two or more components referenced directly or indirectly by the application manifest have the same COM server CLSIDs.
pub const DUPLICATE_CLSID : ErrorCode = ErrorCode::from_constant(14023); // ERROR_SXS_DUPLICATE_CLSID

/// Two or more components referenced directly or indirectly by the application manifest have proxies for the same COM interface IIDs.
pub const DUPLICATE_IID : ErrorCode = ErrorCode::from_constant(14024); // ERROR_SXS_DUPLICATE_IID

/// Two or more components referenced directly or indirectly by the application manifest have the same COM type library TLBIDs.
pub const DUPLICATE_TLBID : ErrorCode = ErrorCode::from_constant(14025); // ERROR_SXS_DUPLICATE_TLBID

/// Two or more components referenced directly or indirectly by the application manifest have the same COM ProgIDs.
pub const DUPLICATE_PROGID : ErrorCode = ErrorCode::from_constant(14026); // ERROR_SXS_DUPLICATE_PROGID

/// Two or more components referenced directly or indirectly by the application manifest are different versions of the same component which is not permitted.
pub const DUPLICATE_ASSEMBLY_NAME : ErrorCode = ErrorCode::from_constant(14027); // ERROR_SXS_DUPLICATE_ASSEMBLY_NAME

/// A component's file does not match the verification information present in the component manifest.
pub const FILE_HASH_MISMATCH : ErrorCode = ErrorCode::from_constant(14028); // ERROR_SXS_FILE_HASH_MISMATCH

/// The policy manifest contains one or more syntax errors.
pub const POLICY_PARSE_ERROR : ErrorCode = ErrorCode::from_constant(14029); // ERROR_SXS_POLICY_PARSE_ERROR

/// Assembly Protection Error : Unable to recover the specified assembly.
pub const PROTECTION_RECOVERY_FAILED : ErrorCode = ErrorCode::from_constant(14074); // ERROR_SXS_PROTECTION_RECOVERY_FAILED

/// Assembly Protection Error : The public key for an assembly was too short to be allowed.
pub const PROTECTION_PUBLIC_KEY_TOO_SHORT : ErrorCode = ErrorCode::from_constant(14075); // ERROR_SXS_PROTECTION_PUBLIC_KEY_TOO_SHORT

/// Assembly Protection Error : The catalog for an assembly is not valid, or does not match the assembly's manifest.
pub const PROTECTION_CATALOG_NOT_VALID : ErrorCode = ErrorCode::from_constant(14076); // ERROR_SXS_PROTECTION_CATALOG_NOT_VALID

/// An HRESULT could not be translated to a corresponding Win32 error code.
pub const UNTRANSLATABLE_HRESULT : ErrorCode = ErrorCode::from_constant(14077); // ERROR_SXS_UNTRANSLATABLE_HRESULT

/// Assembly Protection Error : The catalog for an assembly is missing.
pub const PROTECTION_CATALOG_FILE_MISSING : ErrorCode = ErrorCode::from_constant(14078); // ERROR_SXS_PROTECTION_CATALOG_FILE_MISSING

/// The supplied assembly identity is missing one or more attributes which must be present in this context.
pub const MISSING_ASSEMBLY_IDENTITY_ATTRIBUTE : ErrorCode = ErrorCode::from_constant(14079); // ERROR_SXS_MISSING_ASSEMBLY_IDENTITY_ATTRIBUTE

/// The supplied assembly identity has one or more attribute names that contain characters not permitted in XML names.
pub const INVALID_ASSEMBLY_IDENTITY_ATTRIBUTE_NAME : ErrorCode = ErrorCode::from_constant(14080); // ERROR_SXS_INVALID_ASSEMBLY_IDENTITY_ATTRIBUTE_NAME

/// The referenced assembly could not be found.
pub const ASSEMBLY_MISSING : ErrorCode = ErrorCode::from_constant(14081); // ERROR_SXS_ASSEMBLY_MISSING

/// The activation context activation stack for the running thread of execution is corrupt.
pub const CORRUPT_ACTIVATION_STACK : ErrorCode = ErrorCode::from_constant(14082); // ERROR_SXS_CORRUPT_ACTIVATION_STACK

/// The application isolation metadata for this process or thread has become corrupt.
pub const CORRUPTION : ErrorCode = ErrorCode::from_constant(14083); // ERROR_SXS_CORRUPTION

/// The activation context being deactivated is not the most recently activated one.
pub const EARLY_DEACTIVATION : ErrorCode = ErrorCode::from_constant(14084); // ERROR_SXS_EARLY_DEACTIVATION

/// The activation context being deactivated is not active for the current thread of execution.
pub const INVALID_DEACTIVATION : ErrorCode = ErrorCode::from_constant(14085); // ERROR_SXS_INVALID_DEACTIVATION

/// The activation context being deactivated has already been deactivated.
pub const MULTIPLE_DEACTIVATION : ErrorCode = ErrorCode::from_constant(14086); // ERROR_SXS_MULTIPLE_DEACTIVATION

/// A component used by the isolation facility has requested to terminate the process.
pub const PROCESS_TERMINATION_REQUESTED : ErrorCode = ErrorCode::from_constant(14087); // ERROR_SXS_PROCESS_TERMINATION_REQUESTED

/// A kernel mode component is releasing a reference on an activation context.
pub const RELEASE_ACTIVATION_CONTEXT : ErrorCode = ErrorCode::from_constant(14088); // ERROR_SXS_RELEASE_ACTIVATION_CONTEXT

/// The activation context of system default assembly could not be generated.
pub const SYSTEM_DEFAULT_ACTIVATION_CONTEXT_EMPTY : ErrorCode = ErrorCode::from_constant(14089); // ERROR_SXS_SYSTEM_DEFAULT_ACTIVATION_CONTEXT_EMPTY

/// The value of an attribute in an identity is not within the legal range.
pub const INVALID_IDENTITY_ATTRIBUTE_VALUE : ErrorCode = ErrorCode::from_constant(14090); // ERROR_SXS_INVALID_IDENTITY_ATTRIBUTE_VALUE

/// The name of an attribute in an identity is not within the legal range.
pub const INVALID_IDENTITY_ATTRIBUTE_NAME : ErrorCode = ErrorCode::from_constant(14091); // ERROR_SXS_INVALID_IDENTITY_ATTRIBUTE_NAME

/// An identity contains two definitions for the same attribute.
pub const IDENTITY_DUPLICATE_ATTRIBUTE : ErrorCode = ErrorCode::from_constant(14092); // ERROR_SXS_IDENTITY_DUPLICATE_ATTRIBUTE

/// The identity string is malformed. This may be due to a trailing comma, more than two unnamed attributes, missing attribute name or missing attribute value.
pub const IDENTITY_PARSE_ERROR : ErrorCode = ErrorCode::from_constant(14093); // ERROR_SXS_IDENTITY_PARSE_ERROR

/// The public key token does not correspond to the public key specified.
pub const INCORRECT_PUBLIC_KEY_TOKEN : ErrorCode = ErrorCode::from_constant(14095); // ERROR_SXS_INCORRECT_PUBLIC_KEY_TOKEN

/// The component must be locked before making the request.
pub const ASSEMBLY_NOT_LOCKED : ErrorCode = ErrorCode::from_constant(14097); // ERROR_SXS_ASSEMBLY_NOT_LOCKED

/// The component store has been corrupted.
pub const COMPONENT_STORE_CORRUPT : ErrorCode = ErrorCode::from_constant(14098); // ERROR_SXS_COMPONENT_STORE_CORRUPT

/// The identities of the manifests are identical but their contents are different.
pub const MANIFEST_IDENTITY_SAME_BUT_CONTENTS_DIFFERENT : ErrorCode = ErrorCode::from_constant(14101); // ERROR_SXS_MANIFEST_IDENTITY_SAME_BUT_CONTENTS_DIFFERENT

/// The component identities are different.
pub const IDENTITIES_DIFFERENT : ErrorCode = ErrorCode::from_constant(14102); // ERROR_SXS_IDENTITIES_DIFFERENT

/// The assembly is not a deployment.
pub const ASSEMBLY_IS_NOT_A_DEPLOYMENT : ErrorCode = ErrorCode::from_constant(14103); // ERROR_SXS_ASSEMBLY_IS_NOT_A_DEPLOYMENT

/// The file is not a part of the assembly.
pub const FILE_NOT_PART_OF_ASSEMBLY : ErrorCode = ErrorCode::from_constant(14104); // ERROR_SXS_FILE_NOT_PART_OF_ASSEMBLY

/// The size of the manifest exceeds the maximum allowed.
pub const MANIFEST_TOO_BIG : ErrorCode = ErrorCode::from_constant(14105); // ERROR_SXS_MANIFEST_TOO_BIG

/// The setting is not registered.
pub const SETTING_NOT_REGISTERED : ErrorCode = ErrorCode::from_constant(14106); // ERROR_SXS_SETTING_NOT_REGISTERED

/// One or more required members of the transaction are not present.
pub const TRANSACTION_CLOSURE_INCOMPLETE : ErrorCode = ErrorCode::from_constant(14107); // ERROR_SXS_TRANSACTION_CLOSURE_INCOMPLETE

/// A component is missing file verification information in its manifest.
pub const FILE_HASH_MISSING : ErrorCode = ErrorCode::from_constant(14110); // ERROR_SXS_FILE_HASH_MISSING

/// Two or more components referenced directly or indirectly by the application manifest have the same WinRT ActivatableClass IDs.
pub const DUPLICATE_ACTIVATABLE_CLASS : ErrorCode = ErrorCode::from_constant(14111); // ERROR_SXS_DUPLICATE_ACTIVATABLE_CLASS
