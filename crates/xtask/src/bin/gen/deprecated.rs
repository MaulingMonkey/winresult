use crate::scan::{Code, Codes};

pub(crate) fn codes<'a>(codes: &mut Codes<'a>) {
    for (rs_mod,                rs_id,                      rs_ty,          value,      cpp,                                        deprecated) in [
        ("VM_SAVED_STATE_DUMP", "E_PXE_NOT_PRESENT",        "HResultError", 0xC0370505, "VM_SAVED_STATE_DUMP_E_PXE_NOT_PRESENT",    "no longer exposed by windows headers, error code reused for E_VA_NOT_MAPPED"),
        ("VM_SAVED_STATE_DUMP", "E_PDPTE_NOT_PRESENT",      "HResultError", 0xC0370506, "VM_SAVED_STATE_DUMP_E_PDPTE_NOT_PRESENT",  "no longer exposed by windows headers, error code reused for E_INVALID_VP_STATE"),
        ("VM_SAVED_STATE_DUMP", "E_PDE_NOT_PRESENT",        "HResultError", 0xC0370507, "VM_SAVED_STATE_DUMP_E_PDE_NOT_PRESENT",    "no longer exposed by windows headers"),
        ("VM_SAVED_STATE_DUMP", "E_PTE_NOT_PRESENT",        "HResultError", 0xC0370508, "VM_SAVED_STATE_DUMP_E_PTE_NOT_PRESENT",    "no longer exposed by windows headers"),
        ("ERROR",               "VRF_CFG_AND_IO_ENABLED",   "ErrorCode",    1183,       "ERROR_VRF_CFG_AND_IO_ENABLED",             "no longer exposed by windows headers"),
        ("STATUS",              "VRF_CFG_AND_IO_ENABLED",   "NtStatus",     0xC000049F, "STATUS_VRF_CFG_AND_IO_ENABLED",            "no longer exposed by windows headers"),
    ] {
        codes.push_force(Code {
            cpp:        cpp.into(),
            rs_mod:     rs_mod.into(),
            rs_id:      rs_id.into(),
            rs_ty:      rs_ty.into(),
            rs_value:   value,
            docs:       Vec::new(),
            hide:       true,
            redundant:  deprecated.contains("renamed") || deprecated.contains("replaced") || deprecated.contains("reused"),
            deprecated: Some(deprecated.into()),
        });
    }
}
