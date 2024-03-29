// WARNING: this file is auto-generated by xtask gen and may be overwritten

use super::*;


/// TabletPC inking error code. The property was not found, or supported by the recognizer
pub const E_INVALID_PROPERTY : HResultError = HResultError::from_constant(0x80040241); // TPC_E_INVALID_PROPERTY

/// TabletPC inking error code. No default tablet
pub const E_NO_DEFAULT_TABLET : HResultError = HResultError::from_constant(0x80040212); // TPC_E_NO_DEFAULT_TABLET

/// TabletPC inking error code. Unknown property specified
pub const E_UNKNOWN_PROPERTY : HResultError = HResultError::from_constant(0x8004021B); // TPC_E_UNKNOWN_PROPERTY

/// TabletPC inking error code. An invalid input rectangle was specified
pub const E_INVALID_INPUT_RECT : HResultError = HResultError::from_constant(0x80040219); // TPC_E_INVALID_INPUT_RECT

/// TabletPC inking error code. The stroke object was deleted
pub const E_INVALID_STROKE : HResultError = HResultError::from_constant(0x80040222); // TPC_E_INVALID_STROKE

/// TabletPC inking error code. Initialization failure
pub const E_INITIALIZE_FAIL : HResultError = HResultError::from_constant(0x80040223); // TPC_E_INITIALIZE_FAIL

/// TabletPC inking error code. The data required for the operation was not supplied
pub const E_NOT_RELEVANT : HResultError = HResultError::from_constant(0x80040232); // TPC_E_NOT_RELEVANT

/// TabletPC inking error code. Invalid packet description
pub const E_INVALID_PACKET_DESCRIPTION : HResultError = HResultError::from_constant(0x80040233); // TPC_E_INVALID_PACKET_DESCRIPTION

/// TabletPC inking error code. There are no handwriting recognizers registered
pub const E_RECOGNIZER_NOT_REGISTERED : HResultError = HResultError::from_constant(0x80040235); // TPC_E_RECOGNIZER_NOT_REGISTERED

/// TabletPC inking error code. User does not have the necessary rights to read recognizer information
pub const E_INVALID_RIGHTS : HResultError = HResultError::from_constant(0x80040236); // TPC_E_INVALID_RIGHTS

/// TabletPC inking error code. API calls were made in an incorrect order
pub const E_OUT_OF_ORDER_CALL : HResultError = HResultError::from_constant(0x80040237); // TPC_E_OUT_OF_ORDER_CALL

/// TabletPC inking error code. Queue is full
pub const E_QUEUE_FULL : HResultError = HResultError::from_constant(0x80040238); // TPC_E_QUEUE_FULL

/// TabletPC inking error code. RtpEnabled called multiple times
pub const E_INVALID_CONFIGURATION : HResultError = HResultError::from_constant(0x80040239); // TPC_E_INVALID_CONFIGURATION

/// TabletPC inking error code. A recognizer returned invalid data
pub const E_INVALID_DATA_FROM_RECOGNIZER : HResultError = HResultError::from_constant(0x8004023A); // TPC_E_INVALID_DATA_FROM_RECOGNIZER

/// TabletPC inking error code. String was truncated
pub const S_TRUNCATED : HResultSuccess = HResultSuccess::from_constant(0x00040252); // TPC_S_TRUNCATED

/// TabletPC inking error code. Recognition or training was interrupted
pub const S_INTERRUPTED : HResultSuccess = HResultSuccess::from_constant(0x00040253); // TPC_S_INTERRUPTED

/// TabletPC inking error code. No personalization update to the recognizer because no training data found
pub const S_NO_DATA_TO_PROCESS : HResultSuccess = HResultSuccess::from_constant(0x00040254); // TPC_S_NO_DATA_TO_PROCESS
