// WARNING: this file is auto-generated by xtask gen and may be overwritten

use super::*;


/// Input data cannot be processed in the non-chronological order.
pub const E_OUT_OF_ORDER : ErrorHResult = ErrorHResult::from_constant(0x80400000); // INPUT_E_OUT_OF_ORDER

/// Requested operation cannot be performed inside the callback or event handler.
pub const E_REENTRANCY : ErrorHResult = ErrorHResult::from_constant(0x80400001); // INPUT_E_REENTRANCY

/// Input cannot be processed because there is ongoing interaction with another pointer type.
pub const E_MULTIMODAL : ErrorHResult = ErrorHResult::from_constant(0x80400002); // INPUT_E_MULTIMODAL

/// One or more fields in the input packet are invalid.
pub const E_PACKET : ErrorHResult = ErrorHResult::from_constant(0x80400003); // INPUT_E_PACKET

/// Packets in the frame are inconsistent. Either pointer ids are not unique or there is a discrepancy in timestamps, frame ids, pointer types or source devices.
pub const E_FRAME : ErrorHResult = ErrorHResult::from_constant(0x80400004); // INPUT_E_FRAME

/// The history of frames is inconsistent. Pointer ids, types, source devices don't match, or frame ids are not unique, or timestamps are out of order.
pub const E_HISTORY : ErrorHResult = ErrorHResult::from_constant(0x80400005); // INPUT_E_HISTORY

/// Failed to retrieve information about the input device.
pub const E_DEVICE_INFO : ErrorHResult = ErrorHResult::from_constant(0x80400006); // INPUT_E_DEVICE_INFO

/// Coordinate system transformation failed to transform the data.
pub const E_TRANSFORM : ErrorHResult = ErrorHResult::from_constant(0x80400007); // INPUT_E_TRANSFORM

/// The property is not supported or not reported correctly by the input device.
pub const E_DEVICE_PROPERTY : ErrorHResult = ErrorHResult::from_constant(0x80400008); // INPUT_E_DEVICE_PROPERTY