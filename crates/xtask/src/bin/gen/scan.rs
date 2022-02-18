use crate::Header;

use regex::Regex;

use std::borrow::*;
use std::collections::*;

const ERROR_PREFIX_TO_RUSTY : &'static [(&'static str, &'static str)] = &[
    // cpp prefix           mod
    ("ERROR_CLOUD_FILE_",   "ERROR::CLOUD_FILE",    ),
    ("ERROR_CLUSTER_",      "ERROR::CLUSTER",       ),
    ("ERROR_DBG_",          "ERROR::DBG",           ),
    ("ERROR_DS_",           "ERROR::DS",            ),
    ("ERROR_EVT_",          "ERROR::EVT",           ),
    ("ERROR_GRAPHICS_",     "ERROR::GRAPHICS",      ),
    ("ERROR_IPSEC_",        "ERROR::IPSEC",         ),
    ("ERROR_MRM_",          "ERROR::MRM",           ),
    ("ERROR_MUI_",          "ERROR::MUI",           ),
    ("ERROR_NDIS_",         "ERROR::NDIS",          ),
    ("ERROR_PRI_MERGE_",    "ERROR::PRI_MERGE",     ),
    ("ERROR_SECUREBOOT_",   "ERROR::SECUREBOOT",    ),
    ("ERROR_SERVICE_",      "ERROR::SERVICE",       ),
    ("ERROR_SVHDX_",        "ERROR::SVHDX",         ),
    ("ERROR_SXS_",          "ERROR::SXS",           ),
    ("ERROR_VHD_",          "ERROR::VHD",           ),
    ("ERROR_WMI_",          "ERROR::WMI",           ),
];



#[derive(Default)]
pub struct Codes<'s> {
    pub cpp_processed:  HashSet<&'s str>,
    pub mods:           BTreeMap<&'s str, Vec<Code<'s>>>,
}

impl<'s> Codes<'s> {
    fn push(&mut self, code: Code<'s>) {
        if !self.cpp_processed.contains(code.cpp) {
            self.push_force(code);
        }
    }

    fn push_force(&mut self, code: Code<'s>) {
        self.cpp_processed.insert(code.cpp);
        self.mods.entry(code.rs_mod).or_default().push(code);
    }
}

pub struct Code<'s> {
    pub cpp:        &'s str,
    pub rs_mod:     &'s str,
    pub rs_id:      Cow<'s, str>,
    pub rs_ty:      &'static str,
    pub rs_value:   Cow<'s, str>,
    pub docs:       Vec<Cow<'s, str>>,
    pub redundant:  bool,
    pub hide:       bool,
}

impl Code<'_> {
    pub(crate) fn matches_ty(&self, ty: &str) -> bool {
        match (self.rs_ty, ty) {
            ("SuccessHResult",  "HRESULT")  => true,
            ("ErrorHResult",    "HRESULT")  => true,
            (x, y)                          => x == y,
        }
    }
}

pub(crate) fn hardcoded(codes: &mut Codes) {
    // success codes
    let mut redundant = false;
    for (cpp,               rs_mod,     rs_id,      value) in [
        // redundant codes after this line
        ("ERROR_SUCCESS",   "ERROR",    "SUCCESS",  "0"),
    ].iter().copied() {
        if value == "0" && cpp != "S_OK" { redundant = true }
        codes.mods.entry(rs_mod).or_default().push(Code {
            cpp:        cpp.into(),
            rs_mod:     rs_mod.into(),
            rs_id:      rs_id.into(),
            rs_ty:      "SuccessCodeMicrosoft".into(),
            rs_value:   value.into(),
            docs:       Default::default(),
            hide:       false,
            redundant,
        });
    }
}

pub(crate) fn winerror_h<'s: 'c, 'c>(header: &'s Header, codes: &mut Codes<'c>) {
    let path = &header.path;
    let header = header.code.as_str();

    let mut lines = header.lines().enumerate();
    let re_define_error = Regex::new(r##"^#\s*define\s+(?P<error>(?P<prefix>([A-Z0-9_]+?_)?(S|E|X|ERROR))_(?P<err>[a-zA-Z0-9_]+))\s+(?P<value>.+?)[L]?\s*(//.*)?$"##).expect("re_define_error");
    let re_placeholders = Regex::new(r"(0x)?%[0-9a-zA-Z_]+").expect("re_placeholders");
    let re_url          = Regex::new(r"( |^)(http[s]?://[^ ]+)").expect("re_url");

    // Skip to, and past, ERROR_SUCCESS.  TODO: manually support it?
    loop {
        let line = lines.next().expect("EOF before finding #define ERROR_SUCCESS").1;
        if line.starts_with("#define ERROR_SUCCESS") {
            for _ in 0 .. 3 { let _ = lines.next(); }
            break;
        }
    }

    let mut docs = Vec::new();
    while let Some((line_idx, line)) = lines.next() {
        let line = line.trim();

        if line.is_empty() || line.contains("vailable") && line.contains("error codes") { // XXX: simplify
            docs.clear();
            continue;
        }

        if let Some(define_error) = re_define_error.captures(line) {
            let error   = define_error.name("error" ).unwrap().as_str();
            let prefix  = define_error.name("prefix").unwrap().as_str();
            let err     = define_error.name("err"   ).unwrap().as_str();
            let value   = define_error.name("value" ).unwrap().as_str();
            let success = prefix == "S" || prefix.ends_with("_S");

            if error.ends_with("_MASK"   ) { docs.clear(); continue }
            if error.ends_with("_E_FIRST") { docs.clear(); continue }
            if error.ends_with("_E_LAST" ) { docs.clear(); continue }
            if error.ends_with("_S_FIRST") { docs.clear(); continue }
            if error.ends_with("_S_LAST" ) { docs.clear(); continue }

            let (value, mut redundant) = match value {
                "NO_ERROR"                  => { docs.clear(); continue },
                "ERROR_INVALID_DATA"        => ("13",   true),
                "ERROR_OUTOFMEMORY"         => ("14",   true),
                "ERROR_INVALID_NAME"        => ("123",  true),
                "DNS_ERROR_RCODE_BADTIME"   => ("9018", true),

                "CACHE_E_NOCACHE_UPDATED"   => (value,  true),
                "CAT_E_CATIDNOEXIST"        => (value,  true),
                "CAT_E_NODESCRIPTION"       => (value,  true),
                "CLASS_E_NOAGGREGATION"     => (value,  true),

                _                           => (value,  false),
            };

            if !codes.cpp_processed.insert(error) {
                match error {
                    // normal: these COM-related defines get redefined on _MAC (OS X), just ignore the second def
                    "E_NOTIMPL" | "E_OUTOFMEMORY" | "E_INVALIDARG" | "E_NOINTERFACE" | "E_POINTER" | "E_HANDLE" | "E_ABORT" | "E_FAIL" | "E_ACCESSDENIED" => {},
                    _ => {
                        mmrbi::warning!(at: path, line: line_idx+1, "{} found multiple times", error);
                    },
                }
                docs.clear();
                continue;
            }

            let (ty, value) = if value == "nope" {
                continue // XXX
            } else if let Some(value) = value.strip_prefix_suffix("_NDIS_ERROR_TYPEDEF_(", "L)") {
                ("HRESULT", value)
            } else if let Some(value) = value.strip_prefix_suffix("_HRESULT_TYPEDEF_(", "L)") {
                ("HRESULT", value)
            } else if let Some(value) = value.strip_prefix_suffix("((HRESULT)", "L)") {
                ("HRESULT", value)
            } else if value.starts_with("HRESULT_FROM_WIN32(ERROR_") {
                continue // XXX
            } else if value.starts_with("SEC_E_") {
                continue // XXX
            } else if let Some(_value16) = value.try_parse_u16() {
                if success {
                    ("SuccessCodeMicrosoft", value)
                } else {
                    ("ErrorCodeMicrosoft", value)
                }
            } else if let Some(_value32) = value.try_parse_u32() {
                ("HRESULT", value)
            } else {
                mmrbi::error!(at: path, line: line_idx+1, "unexpected value for `{}`: `{}`", error, value);
                continue
            };

            let ty = if ty == "HRESULT" {
                if let Some(value) = value.try_parse_u32() {
                    if value >= 0x8000_0000 {
                        "ErrorHResult"
                    } else {
                        "SuccessHResult"
                    }
                } else {
                    mmrbi::error!(at: path, line: line_idx+1, "unable to parse integer value for `{}`: `{}`", error, value);
                    continue
                }
            } else {
                ty
            };

            let ty = match error {
                "S_OK" | "S_FALSE" => "SuccessHResult",
                _ => ty,
            };

            if prefix.ends_with("_S") || prefix.ends_with("_E") || prefix.ends_with("_X") {
                // translate:   WHATEVER_E::... => WHATEVER::E_...
                let prefix = &prefix[..prefix.len()-2];
                let err    = &error[prefix.len()+1..];

                let prefix = match prefix {
                    "STATEREPOSTORY" => "STATEREPOSITORY", // fix winsdk header typo
                    _ => prefix,
                };

                codes.push_force(Code {
                    cpp:        error,
                    rs_mod:     prefix,
                    rs_id:      err.into(),
                    rs_ty:      ty.into(),
                    rs_value:   value.into(),
                    docs:       docs.iter().cloned().collect(),
                    hide:       false,
                    redundant,
                });
            } else if prefix.starts_with("S_") || prefix.starts_with("E_") || prefix.starts_with("X_") {
                let prefix  = &prefix[..1];
                let err     = &error[2..];
                codes.push_force(Code {
                    cpp:        error,
                    rs_mod:     prefix,
                    rs_id:      err.into(),
                    rs_ty:      ty.into(),
                    rs_value:   value.into(),
                    docs:       docs.iter().cloned().collect(),
                    hide:       false,
                    redundant,
                });
            } else if !prefix.starts_with("ERROR_") && prefix.ends_with("_ERROR") {
                // translate:       D3D11_ERROR => D3D11
                // but leave alone: ERROR_WHATEVER_ERROR_...
                let prefix = &prefix[..prefix.len()-6];
                let err    = &error[prefix.len()+1..];
                codes.push_force(Code {
                    cpp:        error,
                    rs_mod:     prefix,
                    rs_id:      err.into(),
                    rs_ty:      ty.into(),
                    rs_value:   value.into(),
                    docs:       docs.iter().cloned().collect(),
                    hide:       false,
                    redundant,
                });
            } else {
                let mut hide = false;
                'prefixes: for (prefix, rs_mod) in ERROR_PREFIX_TO_RUSTY.iter().copied() {
                    if let Some(err) = error.strip_prefix(prefix) {
                        codes.push_force(Code {
                            cpp:        error,
                            rs_mod:     rs_mod.into(),
                            rs_id:      err.into(),
                            rs_ty:      ty.into(),
                            rs_value:   value.into(),
                            docs:       docs.iter().cloned().collect(),
                            hide:       false,
                            redundant,
                        });
                        redundant   = true;
                        hide        = true; // hide ERROR::* in favor of ERROR_DS::* and other more specific prefixes
                        break 'prefixes
                    }
                }

                if hide {
                    codes.push_force(Code {
                        cpp:        error,
                        rs_mod:     "ERROR".into(),
                        rs_id:      error.strip_prefix("ERROR_").unwrap().into(),
                        rs_ty:      ty.into(),
                        rs_value:   value.into(),
                        docs:       docs.iter().cloned().collect(),
                        redundant,
                        hide,
                    });
                } else {
                    codes.push_force(Code {
                        cpp:        error,
                        rs_mod:     prefix,
                        rs_id:      err.into(),
                        rs_ty:      ty.into(),
                        rs_value:   value.into(),
                        docs:       docs.iter().cloned().collect(),
                        redundant,
                        hide,
                    });
                }
            }
        } else if let Some(comment) = line.strip_prefix("//") {
            let comment = comment.trim_start();
            if comment.is_empty() { continue }
            if comment.starts_with("MessageId:") || comment == "MessageText:" { docs.clear(); continue }
            assert!(!comment.starts_with("MessageText:"), "MessageText text on same line as header");
            if let Some(header) = comment.strip_prefix_suffix("{", "}") {
                docs.push(format!("### {header}").into());
            } else {
                let comment = re_placeholders   .replace_all(&comment, "`$0`");
                let comment = re_url            .replace_all(&comment, "$1<$2>");
                let comment = comment.replace("*", "\\*");
                docs.push(comment.into());
            }
        } else {
            docs.clear();
        }
    }
}

pub(crate) fn d3d<'s: 'c, 'c>(header: &'s Header, codes: &mut Codes<'c>) {
    let path = &header.path;
    let header = header.code.as_str();

    let mut lines = header.lines().enumerate();
    let re_d3d_hr = Regex::new(r##"^\s*#\s*define\s+(?P<error>(?P<prefix>S|D3D(ERR|OK)?)_(?P<err>[a-zA-Z0-9_]+))\s+(?P<value>S_OK|MAKE_D3D.+?|MAKE_DD.+?)$"##).expect("re_d3d_hr");
    while let Some((line_idx, line)) = lines.next() {
        if let Some(def) = re_d3d_hr.captures(line) {
            let error   = def.name("error" ).unwrap().as_str();
            let prefix  = def.name("prefix").unwrap().as_str();
            let err     = def.name("err"   ).unwrap().as_str();
            let value   = def.name("value" ).unwrap().as_str();

            let (ty, value) = if value == "S_OK" {
                continue // Ok
            } else if let Some(value) = value.strip_prefix_suffix("MAKE_D3DSTATUS(", ")") {
                match value.parse::<u16>() {
                    Ok(value)   => ("SuccessHResult", format!("{}", u32::from(value)+0x08760000)),
                    Err(_) => {
                        mmrbi::error!(at: path, line: line_idx+1, "unexpected value for {}", error);
                        continue
                    },
                }
            } else if let Some(value) = value.strip_prefix_suffix("MAKE_DDSTATUS(", ")") {
                match value.parse::<u16>() {
                    Ok(value)   => ("SuccessHResult", format!("{}", u32::from(value)+0x08760000)),
                    Err(_) => {
                        mmrbi::error!(at: path, line: line_idx+1, "unexpected value for {}", error);
                        continue
                    },
                }
            } else if let Some(value) = value.strip_prefix_suffix("MAKE_D3DHRESULT(", ")") {
                match value.parse::<u16>() {
                    Ok(value)   => ("ErrorHResult", format!("{}", u32::from(value)+0x88760000)),
                    Err(_) => {
                        mmrbi::error!(at: path, line: line_idx+1, "unexpected value for {}", error);
                        continue
                    },
                }
            } else if let Some(value) = value.strip_prefix_suffix("MAKE_DDHRESULT(", ")") {
                match value.parse::<u16>() {
                    Ok(value)   => ("ErrorHResult", format!("{}", u32::from(value)+0x88760000)),
                    Err(_) => {
                        mmrbi::error!(at: path, line: line_idx+1, "unexpected value for {}", error);
                        continue
                    },
                }
            } else {
                mmrbi::error!(at: path, line: line_idx+1, "unexpected value for {}", error);
                continue
            };

            codes.push(Code {
                cpp:        error,
                rs_mod:     prefix,
                rs_id:      err.into(),
                rs_ty:      ty.into(),
                rs_value:   value.into(),
                docs:       Default::default(),
                redundant:  false,
                hide:       false,
            });
        }
    }
}

pub(crate) fn ntstatus_h<'s: 'c, 'c>(header: &'s Header, codes: &mut Codes<'c>) {
    let _path = &header.path;
    let header = header.code.as_str();

    let mut lines = header.lines();
    let re_ntstatus = Regex::new(r##"^\s*#\s*define\s+(?P<error>(?P<prefix>STATUS)_(?P<err>[a-zA-Z0-9_]+))\s+\(\(NTSTATUS\)(?P<value>(0x)?[0-9a-fA-F]+)[L]?\)\s*(//.*)?$"##).expect("re_ntstatus");
    while let Some(line) = lines.next() {
        if let Some(def) = re_ntstatus.captures(line) {
            let error   = def.name("error" ).unwrap().as_str();
            let prefix  = def.name("prefix").unwrap().as_str();
            let err     = def.name("err"   ).unwrap().as_str();
            let value   = def.name("value" ).unwrap().as_str();

            // TODO: harvest MessageText: for doc comments

            codes.push(Code {
                cpp:        error,
                rs_mod:     prefix,
                rs_id:      err.into(),
                rs_ty:      "NTSTATUS".into(),
                rs_value:   value.into(),
                docs:       Default::default(),
                redundant:  (value == "0x00000000") || error == "STATUS_ABANDONED_WAIT_0",
                hide:       false,
            });
        }
    }
}

trait StrExt<'s> : AsRef<str> + 's {
    fn strip_prefix_suffix(&'s self, prefix: &str, suffix: &str) -> Option<&'s str> {
        self.as_ref().strip_prefix(prefix).and_then(|s| s.strip_suffix(suffix))
    }

    fn try_parse_u16(&self) -> Option<u16> {
        let s = self.as_ref();
        if let Some(s) = s.strip_prefix("0x") {
            u16::from_str_radix(s, 16).ok()
        } else if let Some(s) = s.strip_prefix("0") {
            if s.is_empty() {
                Some(0)
            } else {
                u16::from_str_radix(s, 8).ok()
            }
        } else {
            u16::from_str_radix(s, 10).ok()
        }
    }

    fn try_parse_u32(&self) -> Option<u32> {
        let s = self.as_ref();
        if let Some(s) = s.strip_prefix("0x") {
            u32::from_str_radix(s, 16).ok()
        } else if let Some(s) = s.strip_prefix("0") {
            if s.is_empty() {
                Some(0)
            } else {
                u32::from_str_radix(s, 8).ok()
            }
        } else {
            u32::from_str_radix(s, 10).ok()
        }
    }
}

impl<'s, S: AsRef<str> + ?Sized + 's> StrExt<'s> for S {}
