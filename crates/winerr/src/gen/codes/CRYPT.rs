// WARNING: this file is auto-generated by xtask gen and may be overwritten

use super::*;


/// An error occurred while performing an operation on a cryptographic message.
pub const E_MSG_ERROR : ErrorHResult = ErrorHResult::from_constant(0x80091001); // CRYPT_E_MSG_ERROR

/// Unknown cryptographic algorithm.
pub const E_UNKNOWN_ALGO : ErrorHResult = ErrorHResult::from_constant(0x80091002); // CRYPT_E_UNKNOWN_ALGO

/// The object identifier is poorly formatted.
pub const E_OID_FORMAT : ErrorHResult = ErrorHResult::from_constant(0x80091003); // CRYPT_E_OID_FORMAT

/// Invalid cryptographic message type.
pub const E_INVALID_MSG_TYPE : ErrorHResult = ErrorHResult::from_constant(0x80091004); // CRYPT_E_INVALID_MSG_TYPE

/// Unexpected cryptographic message encoding.
pub const E_UNEXPECTED_ENCODING : ErrorHResult = ErrorHResult::from_constant(0x80091005); // CRYPT_E_UNEXPECTED_ENCODING

/// The cryptographic message does not contain an expected authenticated attribute.
pub const E_AUTH_ATTR_MISSING : ErrorHResult = ErrorHResult::from_constant(0x80091006); // CRYPT_E_AUTH_ATTR_MISSING

/// The hash value is not correct.
pub const E_HASH_VALUE : ErrorHResult = ErrorHResult::from_constant(0x80091007); // CRYPT_E_HASH_VALUE

/// The index value is not valid.
pub const E_INVALID_INDEX : ErrorHResult = ErrorHResult::from_constant(0x80091008); // CRYPT_E_INVALID_INDEX

/// The content of the cryptographic message has already been decrypted.
pub const E_ALREADY_DECRYPTED : ErrorHResult = ErrorHResult::from_constant(0x80091009); // CRYPT_E_ALREADY_DECRYPTED

/// The content of the cryptographic message has not been decrypted yet.
pub const E_NOT_DECRYPTED : ErrorHResult = ErrorHResult::from_constant(0x8009100A); // CRYPT_E_NOT_DECRYPTED

/// The enveloped-data message does not contain the specified recipient.
pub const E_RECIPIENT_NOT_FOUND : ErrorHResult = ErrorHResult::from_constant(0x8009100B); // CRYPT_E_RECIPIENT_NOT_FOUND

/// Invalid control type.
pub const E_CONTROL_TYPE : ErrorHResult = ErrorHResult::from_constant(0x8009100C); // CRYPT_E_CONTROL_TYPE

/// Invalid issuer and/or serial number.
pub const E_ISSUER_SERIALNUMBER : ErrorHResult = ErrorHResult::from_constant(0x8009100D); // CRYPT_E_ISSUER_SERIALNUMBER

/// Cannot find the original signer.
pub const E_SIGNER_NOT_FOUND : ErrorHResult = ErrorHResult::from_constant(0x8009100E); // CRYPT_E_SIGNER_NOT_FOUND

/// The cryptographic message does not contain all of the requested attributes.
pub const E_ATTRIBUTES_MISSING : ErrorHResult = ErrorHResult::from_constant(0x8009100F); // CRYPT_E_ATTRIBUTES_MISSING

/// The streamed cryptographic message is not ready to return data.
pub const E_STREAM_MSG_NOT_READY : ErrorHResult = ErrorHResult::from_constant(0x80091010); // CRYPT_E_STREAM_MSG_NOT_READY

/// The streamed cryptographic message requires more data to complete the decode operation.
pub const E_STREAM_INSUFFICIENT_DATA : ErrorHResult = ErrorHResult::from_constant(0x80091011); // CRYPT_E_STREAM_INSUFFICIENT_DATA

/// The length specified for the output data was insufficient.
pub const E_BAD_LEN : ErrorHResult = ErrorHResult::from_constant(0x80092001); // CRYPT_E_BAD_LEN

/// An error occurred during encode or decode operation.
pub const E_BAD_ENCODE : ErrorHResult = ErrorHResult::from_constant(0x80092002); // CRYPT_E_BAD_ENCODE

/// An error occurred while reading or writing to a file.
pub const E_FILE_ERROR : ErrorHResult = ErrorHResult::from_constant(0x80092003); // CRYPT_E_FILE_ERROR

/// Cannot find object or property.
pub const E_NOT_FOUND : ErrorHResult = ErrorHResult::from_constant(0x80092004); // CRYPT_E_NOT_FOUND

/// The object or property already exists.
pub const E_EXISTS : ErrorHResult = ErrorHResult::from_constant(0x80092005); // CRYPT_E_EXISTS

/// No provider was specified for the store or object.
pub const E_NO_PROVIDER : ErrorHResult = ErrorHResult::from_constant(0x80092006); // CRYPT_E_NO_PROVIDER

/// The specified certificate is self signed.
pub const E_SELF_SIGNED : ErrorHResult = ErrorHResult::from_constant(0x80092007); // CRYPT_E_SELF_SIGNED

/// The previous certificate or CRL context was deleted.
pub const E_DELETED_PREV : ErrorHResult = ErrorHResult::from_constant(0x80092008); // CRYPT_E_DELETED_PREV

/// Cannot find the requested object.
pub const E_NO_MATCH : ErrorHResult = ErrorHResult::from_constant(0x80092009); // CRYPT_E_NO_MATCH

/// The certificate does not have a property that references a private key.
pub const E_UNEXPECTED_MSG_TYPE : ErrorHResult = ErrorHResult::from_constant(0x8009200A); // CRYPT_E_UNEXPECTED_MSG_TYPE

/// Cannot find the certificate and private key for decryption.
pub const E_NO_KEY_PROPERTY : ErrorHResult = ErrorHResult::from_constant(0x8009200B); // CRYPT_E_NO_KEY_PROPERTY

/// Cannot find the certificate and private key to use for decryption.
pub const E_NO_DECRYPT_CERT : ErrorHResult = ErrorHResult::from_constant(0x8009200C); // CRYPT_E_NO_DECRYPT_CERT

/// Not a cryptographic message or the cryptographic message is not formatted correctly.
pub const E_BAD_MSG : ErrorHResult = ErrorHResult::from_constant(0x8009200D); // CRYPT_E_BAD_MSG

/// The signed cryptographic message does not have a signer for the specified signer index.
pub const E_NO_SIGNER : ErrorHResult = ErrorHResult::from_constant(0x8009200E); // CRYPT_E_NO_SIGNER

/// Final closure is pending until additional frees or closes.
pub const E_PENDING_CLOSE : ErrorHResult = ErrorHResult::from_constant(0x8009200F); // CRYPT_E_PENDING_CLOSE

/// The certificate is revoked.
pub const E_REVOKED : ErrorHResult = ErrorHResult::from_constant(0x80092010); // CRYPT_E_REVOKED

/// No Dll or exported function was found to verify revocation.
pub const E_NO_REVOCATION_DLL : ErrorHResult = ErrorHResult::from_constant(0x80092011); // CRYPT_E_NO_REVOCATION_DLL

/// The revocation function was unable to check revocation for the certificate.
pub const E_NO_REVOCATION_CHECK : ErrorHResult = ErrorHResult::from_constant(0x80092012); // CRYPT_E_NO_REVOCATION_CHECK

/// The revocation function was unable to check revocation because the revocation server was offline.
pub const E_REVOCATION_OFFLINE : ErrorHResult = ErrorHResult::from_constant(0x80092013); // CRYPT_E_REVOCATION_OFFLINE

/// The certificate is not in the revocation server's database.
pub const E_NOT_IN_REVOCATION_DATABASE : ErrorHResult = ErrorHResult::from_constant(0x80092014); // CRYPT_E_NOT_IN_REVOCATION_DATABASE

/// The string contains a non-numeric character.
pub const E_INVALID_NUMERIC_STRING : ErrorHResult = ErrorHResult::from_constant(0x80092020); // CRYPT_E_INVALID_NUMERIC_STRING

/// The string contains a non-printable character.
pub const E_INVALID_PRINTABLE_STRING : ErrorHResult = ErrorHResult::from_constant(0x80092021); // CRYPT_E_INVALID_PRINTABLE_STRING

/// The string contains a character not in the 7 bit ASCII character set.
pub const E_INVALID_IA5_STRING : ErrorHResult = ErrorHResult::from_constant(0x80092022); // CRYPT_E_INVALID_IA5_STRING

/// The string contains an invalid X500 name attribute key, oid, value or delimiter.
pub const E_INVALID_X500_STRING : ErrorHResult = ErrorHResult::from_constant(0x80092023); // CRYPT_E_INVALID_X500_STRING

/// The dwValueType for the CERT_NAME_VALUE is not one of the character strings. Most likely it is either a CERT_RDN_ENCODED_BLOB or CERT_RDN_OCTET_STRING.
pub const E_NOT_CHAR_STRING : ErrorHResult = ErrorHResult::from_constant(0x80092024); // CRYPT_E_NOT_CHAR_STRING

/// The Put operation cannot continue. The file needs to be resized. However, there is already a signature present. A complete signing operation must be done.
pub const E_FILERESIZED : ErrorHResult = ErrorHResult::from_constant(0x80092025); // CRYPT_E_FILERESIZED

/// The cryptographic operation failed due to a local security option setting.
pub const E_SECURITY_SETTINGS : ErrorHResult = ErrorHResult::from_constant(0x80092026); // CRYPT_E_SECURITY_SETTINGS

/// No DLL or exported function was found to verify subject usage.
pub const E_NO_VERIFY_USAGE_DLL : ErrorHResult = ErrorHResult::from_constant(0x80092027); // CRYPT_E_NO_VERIFY_USAGE_DLL

/// The called function was unable to do a usage check on the subject.
pub const E_NO_VERIFY_USAGE_CHECK : ErrorHResult = ErrorHResult::from_constant(0x80092028); // CRYPT_E_NO_VERIFY_USAGE_CHECK

/// Since the server was offline, the called function was unable to complete the usage check.
pub const E_VERIFY_USAGE_OFFLINE : ErrorHResult = ErrorHResult::from_constant(0x80092029); // CRYPT_E_VERIFY_USAGE_OFFLINE

/// The subject was not found in a Certificate Trust List (CTL).
pub const E_NOT_IN_CTL : ErrorHResult = ErrorHResult::from_constant(0x8009202A); // CRYPT_E_NOT_IN_CTL

/// None of the signers of the cryptographic message or certificate trust list is trusted.
pub const E_NO_TRUSTED_SIGNER : ErrorHResult = ErrorHResult::from_constant(0x8009202B); // CRYPT_E_NO_TRUSTED_SIGNER

/// The public key's algorithm parameters are missing.
pub const E_MISSING_PUBKEY_PARA : ErrorHResult = ErrorHResult::from_constant(0x8009202C); // CRYPT_E_MISSING_PUBKEY_PARA

/// An object could not be located using the object locator infrastructure with the given name.
pub const E_OBJECT_LOCATOR_OBJECT_NOT_FOUND : ErrorHResult = ErrorHResult::from_constant(0x8009202D); // CRYPT_E_OBJECT_LOCATOR_OBJECT_NOT_FOUND

/// OSS Certificate encode/decode error code base
/// See asn1code.h for a definition of the OSS runtime errors. The OSS error values are offset by CRYPT_E_OSS_ERROR.
pub const E_OSS_ERROR : ErrorHResult = ErrorHResult::from_constant(0x80093000); // CRYPT_E_OSS_ERROR

/// ASN1 Certificate encode/decode error code base. The ASN1 error values are offset by CRYPT_E_ASN1_ERROR.
pub const E_ASN1_ERROR : ErrorHResult = ErrorHResult::from_constant(0x80093100); // CRYPT_E_ASN1_ERROR

/// ASN1 internal encode or decode error.
pub const E_ASN1_INTERNAL : ErrorHResult = ErrorHResult::from_constant(0x80093101); // CRYPT_E_ASN1_INTERNAL

/// ASN1 unexpected end of data.
pub const E_ASN1_EOD : ErrorHResult = ErrorHResult::from_constant(0x80093102); // CRYPT_E_ASN1_EOD

/// ASN1 corrupted data.
pub const E_ASN1_CORRUPT : ErrorHResult = ErrorHResult::from_constant(0x80093103); // CRYPT_E_ASN1_CORRUPT

/// ASN1 value too large.
pub const E_ASN1_LARGE : ErrorHResult = ErrorHResult::from_constant(0x80093104); // CRYPT_E_ASN1_LARGE

/// ASN1 constraint violated.
pub const E_ASN1_CONSTRAINT : ErrorHResult = ErrorHResult::from_constant(0x80093105); // CRYPT_E_ASN1_CONSTRAINT

/// ASN1 out of memory.
pub const E_ASN1_MEMORY : ErrorHResult = ErrorHResult::from_constant(0x80093106); // CRYPT_E_ASN1_MEMORY

/// ASN1 buffer overflow.
pub const E_ASN1_OVERFLOW : ErrorHResult = ErrorHResult::from_constant(0x80093107); // CRYPT_E_ASN1_OVERFLOW

/// ASN1 function not supported for this PDU.
pub const E_ASN1_BADPDU : ErrorHResult = ErrorHResult::from_constant(0x80093108); // CRYPT_E_ASN1_BADPDU

/// ASN1 bad arguments to function call.
pub const E_ASN1_BADARGS : ErrorHResult = ErrorHResult::from_constant(0x80093109); // CRYPT_E_ASN1_BADARGS

/// ASN1 bad real value.
pub const E_ASN1_BADREAL : ErrorHResult = ErrorHResult::from_constant(0x8009310A); // CRYPT_E_ASN1_BADREAL

/// ASN1 bad tag value met.
pub const E_ASN1_BADTAG : ErrorHResult = ErrorHResult::from_constant(0x8009310B); // CRYPT_E_ASN1_BADTAG

/// ASN1 bad choice value.
pub const E_ASN1_CHOICE : ErrorHResult = ErrorHResult::from_constant(0x8009310C); // CRYPT_E_ASN1_CHOICE

/// ASN1 bad encoding rule.
pub const E_ASN1_RULE : ErrorHResult = ErrorHResult::from_constant(0x8009310D); // CRYPT_E_ASN1_RULE

/// ASN1 bad unicode (UTF8).
pub const E_ASN1_UTF8 : ErrorHResult = ErrorHResult::from_constant(0x8009310E); // CRYPT_E_ASN1_UTF8

/// ASN1 bad PDU type.
pub const E_ASN1_PDU_TYPE : ErrorHResult = ErrorHResult::from_constant(0x80093133); // CRYPT_E_ASN1_PDU_TYPE

/// ASN1 not yet implemented.
pub const E_ASN1_NYI : ErrorHResult = ErrorHResult::from_constant(0x80093134); // CRYPT_E_ASN1_NYI

/// ASN1 skipped unknown extension(s).
pub const E_ASN1_EXTENDED : ErrorHResult = ErrorHResult::from_constant(0x80093201); // CRYPT_E_ASN1_EXTENDED

/// ASN1 end of data expected
pub const E_ASN1_NOEOD : ErrorHResult = ErrorHResult::from_constant(0x80093202); // CRYPT_E_ASN1_NOEOD
