use crate::Header;

use regex::Regex;

use std::borrow::*;
use std::collections::*;

const ERROR_PREFIX_TO_RUSTY : &'static [(&'static str, &'static str)] = &[
    // cpp prefix           mod
    ("ERROR_BIDI_",         "ERROR::BIDI",          ),
    ("ERROR_CLOUD_FILE_",   "ERROR::CLOUD_FILE",    ),
    ("ERROR_CLUSTER_",      "ERROR::CLUSTER",       ),
    ("ERROR_DBG_",          "ERROR::DBG",           ),
    ("ERROR_DDS_",          "ERROR::DDS",           ),
    ("ERROR_DHCP_",         "ERROR::DHCP",          ),
    ("ERROR_DS_",           "ERROR::DS",            ),
    ("ERROR_EVT_",          "ERROR::EVT",           ),
    ("ERROR_FLT_",          "ERROR::FLT",           ),
    ("ERROR_FTP_",          "ERROR::FTP",           ),
    ("ERROR_GRAPHICS_",     "ERROR::GRAPHICS",      ),
    ("ERROR_GOPHER_",       "ERROR::GOPHER",        ),
    ("ERROR_HTTP_",         "ERROR::HTTP",          ),
    ("ERROR_INTERNET_",     "ERROR::INTERNET",      ),
    ("ERROR_IPSEC_",        "ERROR::IPSEC",         ),
    ("ERROR_MRM_",          "ERROR::MRM",           ),
    ("ERROR_MUI_",          "ERROR::MUI",           ),
    ("ERROR_NDIS_",         "ERROR::NDIS",          ),
    ("ERROR_PATCH_",        "ERROR::PATCH",         ),
    ("ERROR_PCW_",          "ERROR::PCW",           ),
    ("ERROR_PRI_MERGE_",    "ERROR::PRI_MERGE",     ),
    ("ERROR_SECUREBOOT_",   "ERROR::SECUREBOOT",    ),
    ("ERROR_SERVER_",       "ERROR::SERVER",        ),
    ("ERROR_SERVICE_",      "ERROR::SERVICE",       ),
    ("ERROR_SVHDX_",        "ERROR::SVHDX",         ),
    ("ERROR_SXS_",          "ERROR::SXS",           ),
    ("ERROR_VHD_",          "ERROR::VHD",           ),
    ("ERROR_WINHTTP_",      "ERROR::WINHTTP",       ),
    ("ERROR_WMI_",          "ERROR::WMI",           ),
    ("ERROR_WSMAN_",        "ERROR::WSMAN",         ),
    ("ERROR_WINRS_",        "ERROR::WINRS",         ),
];

const PREFIX_TO_SKIP : &'static [&'static str] = &[
    "DEVICE_DSM_RANGE_",
    "FLAGS_ERROR_UI_",
    "HTTP_QUERY_X_",
    "INTERNET_OPTION_",
    "PRINTER_ERROR_",       // TODO: unique error namespace?
];

const MIDFIX_TO_SKIP : &'static [&'static str] = &[
    "_ERROR_MASK_",
];

const POSTFIX_TO_SKIP : &'static [&'static str] = &[
    "_E_FIRST",
    "_E_LAST",
    "_S_FIRST",
    "_S_LAST",
    "_X_EXTENT",
];

const POSTFIX_TO_SKIP_WARN : &'static [&'static str] = &[
    "_BASE",
    "_END",
    "_MASK",
    "_FIRST",
    "_LAST",
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
        match (self.rs_ty,          ty,                     ) {
            ("SuccessHResult",      "HRESULT",              ) => true,
            ("ErrorHResult",        "HRESULT",              ) => true,
            ("ErrorHResult",        "ErrorHResultOrCode",   ) => true,
            ("ErrorCodeMicrosoft",  "ErrorHResultOrCode",   ) => true,
            (x,                     y,                      ) => x == y,
        }
    }
}

pub(crate) fn winerror_h<'s: 'c, 'c>(header: &'s Header, codes: &mut Codes<'c>) {
    let mut lines = header.lines();
    let re_define_error = Regex::new(r##"^#\s*define\s+(?P<error>(?P<prefix>([A-Z0-9_]+?_)?(S|E|X|ERROR))_(?P<err>[a-zA-Z0-9_]+))\s+(?P<value>.+?)[L]?\s*(//.*)?$"##).expect("re_define_error");
    let re_placeholders = Regex::new(r#"(0x)?%[0-9a-zA-Z_]+"#).expect("re_placeholders");
    let re_url          = Regex::new(r#"([ "]|^)(http[s]?://[^" ]+)"#).expect("re_url");
    let re_escape       = Regex::new(r#"[*\[\]]"#).expect("re_escape");

    let mut docs = Vec::new();
    while let Some(line) = lines.next() {
        let text = line.text.trim();

        if text.is_empty() || text.contains("vailable") && text.contains("error codes") { // XXX: simplify
            docs.clear();
            continue;
        }

        if let Some(define_error) = re_define_error.captures(text) {
            let error   = define_error.name("error" ).unwrap().as_str();
            let prefix  = define_error.name("prefix").unwrap().as_str();
            let err     = define_error.name("err"   ).unwrap().as_str();
            let value   = define_error.name("value" ).unwrap().as_str();
            let success = prefix == "S" || prefix.ends_with("_S");

            // DO NOT EXPOSE THIS MESS AS IS.  See [doc/ept-and-rpc-codes-are-evil.md](https://github.com/MaulingMonkey/winresult/blob/5094a8a5568392ef855babd8bc62458f29153e46/crates/winresult/doc/ept-and-rpc-codes-are-evil.md) for details.
            if error.starts_with("EPT_") { docs.clear(); continue }
            if error.starts_with("RPC_") { docs.clear(); continue }

            // Handle suffixes not really meant as codes, but used for ranges of codes, masks, bases, etc.
            // Prefer ranges or range detection fns?
            let skip = match error {
                "ERROR_IPSEC_IKE_NEG_STATUS_END"            => true, // these seem to be not quite error codes, but can't quite tell 100%
                "ERROR_IPSEC_IKE_NEG_STATUS_EXTENDED_END"   => true, // these seem to be not quite error codes, but can't quite tell 100%
                "ERROR_PCW_BASE"                            => true,
                "FWP_E_INVALID_NET_MASK"                    => true,
                "NETSH_ERROR_BASE"                          => true,
                "TPM_E_ERROR_MASK"                          => true,
                "TPM_E_PCP_ERROR_MASK"                      => true,

                // "Backward compatibility--do not use."
                "ERROR_NO_DEFAULT_INTERFACE_DEVICE"         => true,
                "ERROR_INTERFACE_DEVICE_ACTIVE"             => true,
                "ERROR_INTERFACE_DEVICE_REMOVED"            => true,
                "ERROR_NO_SUCH_INTERFACE_DEVICE"            => true,

                "ERROR_IMAGE_NOT_AT_BASE"                   => false,
                "ERROR_IMAGE_AT_DIFFERENT_BASE"             => false,
                "UI_E_START_KEYFRAME_AFTER_END"             => false,
                "WS_S_END"                                  => false,

                _ if PREFIX_TO_SKIP .iter().copied().any(|f| error.starts_with  (f)) => true,
                _ if POSTFIX_TO_SKIP.iter().copied().any(|f| error.ends_with    (f)) => true,
                _ if MIDFIX_TO_SKIP .iter().copied().any(|f| error.contains     (f)) => true,

                _ if POSTFIX_TO_SKIP_WARN.iter().copied().any(|s| error.ends_with(s)) => {
                    if !docs.is_empty() { mmrbi::warning!(at: &header.path, line: line.no(), "{} is documented? not skipping...", error) }
                    true
                },
                _ => false
            };

            if skip {
                docs.clear();
                continue
            }


            let (value, mut redundant) = match value {
                "NO_ERROR"                  => { docs.clear(); continue },
                "ERROR_INVALID_DATA"        => ("13",   true),
                "ERROR_OUTOFMEMORY"         => ("14",   true),
                "ERROR_NOT_SUPPORTED"       => ("50",   true),
                "ERROR_INVALID_NAME"        => ("123",  true),
                "DNS_ERROR_RCODE_BADTIME"   => ("9018", true),
                "ERROR_INTERNET_DISCONNECTED"=>("12163",true),

                "CACHE_E_NOCACHE_UPDATED"   => (value,  true),
                "CAT_E_CATIDNOEXIST"        => (value,  true),
                "CAT_E_NODESCRIPTION"       => (value,  true),
                "CLASS_E_NOAGGREGATION"     => (value,  true),

                _                           => (value,  false),
            };

            redundant |= match error {
                "SEC_E_OK"                  => true,
                "ERROR_BIDI_STATUS_OK"      => true,
                _                           => false,
            };

            if !codes.cpp_processed.insert(error) {
                match error {
                    // normal: these COM-related defines get redefined on _MAC (OS X), just ignore the second def
                    "E_NOTIMPL" | "E_OUTOFMEMORY" | "E_INVALIDARG" | "E_NOINTERFACE" | "E_POINTER" | "E_HANDLE" | "E_ABORT" | "E_FAIL" | "E_ACCESSDENIED" => {},
                    _ => {
                        mmrbi::warning!(at: &header.path, line: line.no(), "{} found multiple times", error);
                    },
                }
                docs.clear();
                continue;
            }

            let mut rs_value    : Cow<'c, str> = "".into();
            let mut rs_ty       : &'static str = "";

            for (pre,                                               post,   ty,                     p_redundant,  base, pattern_header, ) in [
                ("_NDIS_ERROR_TYPEDEF_(",                           "L)",   "HRESULT",              false,           0, "",             ),
                ("_HRESULT_TYPEDEF_(",                              "L)",   "HRESULT",              false,           0, "",             ),
                ("((HRESULT)",                                      "L)",   "HRESULT",              false,           0, "",             ),
                ("(RASBASE+",                                       ")",    "ErrorCodeMicrosoft",   true,          600, "RasError.h",   ),
                ("(RASBASE + ",                                     ")",    "ErrorCodeMicrosoft",   true,          600, "RasError.h",   ),
                ("(ROUTEBASE+",                                     ")",    "ErrorCodeMicrosoft",   false,         900, "MprError.h",   ),
                ("(TCBASE+",                                        ")",    "ErrorCodeMicrosoft",   true,         7500, "TCError.h",    ),
                ("(WINHTTP_ERROR_BASE + ",                          ")",    "ErrorCodeMicrosoft",   false,       12000, "winhttp.h",    ),
                ("(INTERNET_ERROR_BASE + ",                         ")",    "ErrorCodeMicrosoft",   true,        12000, "",             ), // conflicts: with itself -_-
                ("(ERROR_BIDI_ERROR_BASE + ",                       ")",    "ErrorCodeMicrosoft",   true,        13000, "winspool.h",   ), // conflicts: with ipsec
                ("(INTERNET_INTERNAL_ERROR_BASE + ",                ")",    "ErrorCodeMicrosoft",   true,  900 + 12000, "Winineti.h"    ),
                ("(NETSH_ERROR_BASE + ",                            ")",    "ErrorCodeMicrosoft",   true,        15000, "NetSh.h",      ), // conflicts: ERROR_EVT_INVALID_CHANNEL_PATH 15000 (winerror.h)
                ("(ERROR_PCW_BASE + ",                              ")",    "ErrorHResult",         false,  0xC00E5101, "PatchWiz.h",   ),
                ("(APPLICATION_ERROR_MASK|ERROR_SEVERITY_ERROR|",   ")",    "ErrorHResult",         false,  0xE0000000, "",             ),
            ].iter().copied() {
                if !pattern_header.is_empty() && !header.path.ends_with(pattern_header) { continue }
                if let Some(value) = value.strip_prefix_suffix(pre, post).and_then(|v| v.try_parse_u32()) {
                    let value = value + base;
                    rs_ty = ty;
                    rs_value = match ty {
                        "HRESULT" | "ErrorHResult"  => format!("0x{:08X}", value),
                        _                           => format!("{}", value),
                    }.into();
                    redundant |= p_redundant;
                    break;
                }
            }

            let ty = if !rs_ty.is_empty() {
                rs_ty
            } else if value.starts_with("HRESULT_FROM_WIN32(ERROR_") {
                continue // XXX
            } else if value.starts_with("SEC_E_") {
                continue // XXX
            } else if let Some(_value16) = value.try_parse_u16() {
                rs_value = value.into();
                if success { "SuccessCodeMicrosoft" } else { "ErrorCodeMicrosoft" }
            } else if let Some(_value32) = value.try_parse_u32() {
                rs_value = value.into();
                "HRESULT"
            } else {
                mmrbi::error!(at: &header.path, line: line.no(), "unexpected value for `{}`: `{}`", error, value);
                continue
            };

            let ty = if ty == "HRESULT" {
                if let Some(value) = rs_value.try_parse_u32() {
                    if value >= 0x8000_0000 {
                        "ErrorHResult"
                    } else {
                        "SuccessHResult"
                    }
                } else {
                    mmrbi::error!(at: &header.path, line: line.no(), "unable to parse integer value for `{}`: `{}`", error, value);
                    continue
                }
            } else {
                ty
            };

            let ty = match error {
                "S_OK" | "S_FALSE" => "SuccessHResult",
                "ERROR_SUCCESS"    => "SuccessCodeMicrosoft",
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
                    rs_value,
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
                    rs_value,
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
                    rs_value,
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
                            rs_value:   rs_value.clone(),
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
                        rs_value:   rs_value,
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
                        rs_value,
                        docs:       docs.iter().cloned().collect(),
                        redundant,
                        hide,
                    });
                }
            }
        } else if let Some(comment) = text.strip_prefix("//") {
            let comment = comment.trim_start();
            if comment.is_empty() { continue }
            if comment.starts_with("MessageId:") || comment == "MessageText:" || comment.starts_with("#define") { docs.clear(); continue }
            assert!(!comment.starts_with("MessageText:"), "MessageText text on same line as header");
            if let Some(header) = comment.strip_prefix_suffix("{", "}") {
                docs.push(format!("### {header}").into());
            } else {
                let comment = re_placeholders   .replace_all(&comment, "`$0`");
                let comment = re_url            .replace_all(&comment, "$1<$2>");
                let comment = re_escape         .replace_all(&comment, "\\$0");
                docs.push(comment.into_owned().into());
            }
        } else {
            docs.clear();
        }
    }
}

pub(crate) fn d3d<'s: 'c, 'c>(header: &'s Header, codes: &mut Codes<'c>) {
    let mut lines = header.lines();
    let re_d3d_hr = Regex::new(r##"^\s*#\s*define\s+(?P<error>(?P<prefix>S|D3D(ERR|OK)?)_(?P<err>[a-zA-Z0-9_]+))\s+(?P<value>S_OK|MAKE_D3D.+?|MAKE_DD.+?)$"##).expect("re_d3d_hr");
    while let Some(line) = lines.next() {
        if let Some(def) = re_d3d_hr.captures(line.text) {
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
                        mmrbi::error!(at: &header.path, line: line.no(), "unexpected value for {}", error);
                        continue
                    },
                }
            } else if let Some(value) = value.strip_prefix_suffix("MAKE_DDSTATUS(", ")") {
                match value.parse::<u16>() {
                    Ok(value)   => ("SuccessHResult", format!("{}", u32::from(value)+0x08760000)),
                    Err(_) => {
                        mmrbi::error!(at: &header.path, line: line.no(), "unexpected value for {}", error);
                        continue
                    },
                }
            } else if let Some(value) = value.strip_prefix_suffix("MAKE_D3DHRESULT(", ")") {
                match value.parse::<u16>() {
                    Ok(value)   => ("ErrorHResult", format!("{}", u32::from(value)+0x88760000)),
                    Err(_) => {
                        mmrbi::error!(at: &header.path, line: line.no(), "unexpected value for {}", error);
                        continue
                    },
                }
            } else if let Some(value) = value.strip_prefix_suffix("MAKE_DDHRESULT(", ")") {
                match value.parse::<u16>() {
                    Ok(value)   => ("ErrorHResult", format!("{}", u32::from(value)+0x88760000)),
                    Err(_) => {
                        mmrbi::error!(at: &header.path, line: line.no(), "unexpected value for {}", error);
                        continue
                    },
                }
            } else {
                mmrbi::error!(at: &header.path, line: line.no(), "unexpected value for {}", error);
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
    let mut lines = header.lines();
    let re_ntstatus     = Regex::new(r##"^\s*#\s*define\s+(?P<error>(?P<prefix>STATUS)_(?P<err>[a-zA-Z0-9_]+))\s+\(\(NTSTATUS\)(?P<value>(0x)?[0-9a-fA-F]+)[L]?\)\s*(//.*)?$"##).expect("re_ntstatus");
    let re_placeholders = Regex::new(r#"(0x)?%[0-9a-zA-Z_]+"#).expect("re_placeholders");
    let re_url          = Regex::new(r#"([ "]|^)(http[s]?://[^" ]+)"#).expect("re_url");
    let re_escape       = Regex::new(r#"[*\[\]]"#).expect("re_escape");
    while let Some(line) = lines.next() {
        if let Some(def) = re_ntstatus.captures(line.text) {
            let error   = def.name("error" ).unwrap().as_str();
            let prefix  = def.name("prefix").unwrap().as_str();
            let err     = def.name("err"   ).unwrap().as_str();
            let value   = def.name("value" ).unwrap().as_str();

            let mut comment_start_line_idx = line.idx();
            while let Some(prev_line) = comment_start_line_idx.checked_sub(1).and_then(|l| header.line(l)) {
                let trim_start = prev_line.text.trim_start();
                if !trim_start.starts_with("//") { break }
                if trim_start.starts_with("// MessageText:") { break }
                comment_start_line_idx -= 1;
            }

            let docs = (comment_start_line_idx .. line.idx()).filter_map(|l| Some({
                let comment = header.line(l)?.text;
                let comment = comment.strip_prefix("//").unwrap_or(comment);
                let comment = comment.trim();
                if comment.is_empty() { return None }
                let comment = re_placeholders   .replace_all(&comment, "`$0`");
                let comment = re_url            .replace_all(&comment, "$1<$2>");
                let comment = re_escape         .replace_all(&comment, "\\$0");
                if let Some(header) = comment.strip_prefix_suffix("{", "}") {
                    format!("### {header}").into()
                } else {
                    comment.into_owned().into()
                }
            })).collect();

            codes.push(Code {
                cpp:        error,
                rs_mod:     prefix,
                rs_id:      err.into(),
                rs_ty:      "NTSTATUS".into(),
                rs_value:   value.into(),
                docs,
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
