use crate::scan;



pub fn codes(codes: &scan::Codes) {
    let types = "ErrorCodeMicrosoft SuccessCodeMicrosoft ErrorHResult SuccessHResult HRESULT NTSTATUS";

    let _ = std::fs::create_dir_all("crates/winerr/src/gen/codes/ERROR");

    for (rs_mod, codes) in codes.mods.iter() {
        let path = format!("crates/winerr/src/gen/codes/{}.rs", rs_mod.replace("::", "/"));
        mmrbi::fs::write_if_modified_with(path, |rs|{
            use std::io::{Write as _};

            writeln!(rs, "// WARNING: this file is auto-generated by xtask gen and may be overwritten")?;
            writeln!(rs)?;
            writeln!(rs, "use super::*;")?;
            writeln!(rs)?;
            let mut prev_doc = false;
            for code in codes.iter() {
                #[allow(unused_variables)] let scan::Code { cpp, rs_mod, rs_id, rs_ty, rs_value, docs, redundant, hide } = &code;

                if prev_doc || (!hide && !docs.is_empty()) { writeln!(rs)?; }

                if *hide {
                    write!(rs, "#[doc(hidden)] ")?;
                    prev_doc = false;
                } else {
                    for doc in docs.iter() { writeln!(rs, "/// {doc}")?; }
                    prev_doc = !docs.is_empty();
                }
                writeln!(rs, "pub const {rs_id} : {rs_ty} = {rs_ty}::from_constant({rs_value}); // {cpp}")?;
            }

            Ok(())
        }).unwrap();
    }

    let _ = std::fs::create_dir_all("crates/winerr-core/src/gen");
    mmrbi::fs::write_if_modified_with("crates/winerr-core/src/gen/debug.rs", |rs|{
        use std::io::{Write as _};

        writeln!(rs, "// WARNING: this file is auto-generated by xtask gen and may be overwritten")?;

        writeln!(rs)?;
        writeln!(rs, r#"impl Debug for FacilityHrMicrosoft {{"#)?;
        writeln!(rs, r#"    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {{"#)?;
        writeln!(rs, r#"        #[allow(unreachable_patterns)] let s = match self.0 {{"#)?;
        {
            macro_rules! microsoft_hresult_facilities {($(
                #define $prefix:ident $f:ident $value:literal
            )*) => {{$(
                let value = $value;
                let cpp = concat!(stringify!($prefix), stringify!($f));
                writeln!(rs, r#"            {value} => "{cpp}","#)?;
            )*}}}
            include!("../../../../../crates/winerr/src/hresult/extra.facilities.rs");
            include!("../../../../../crates/winerr/src/hresult/winerror.facilities.rs");
        }
        writeln!(rs, r#"            v => return write!(fmt, "FACILITY::??? ({{v}})")"#)?;
        writeln!(rs, r#"        }};"#)?;
        writeln!(rs, r#"        write!(fmt, "{{}}", s)"#)?;
        writeln!(rs, r#"    }}"#)?;
        writeln!(rs, r#"}}"#)?;

        writeln!(rs)?;
        writeln!(rs, r#"impl Debug for FacilityNtStatusMicrosoft {{"#)?;
        writeln!(rs, r#"    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {{"#)?;
        writeln!(rs, r#"        #[allow(unreachable_patterns)] let s = match self.0 {{"#)?;
        {
            macro_rules! microsoft_ntstatus_facilities {($(
                #define $prefix:ident $f:ident $value:literal
            )*) => {{$(
                let value = $value;
                let cpp = concat!(stringify!($prefix), stringify!($f));
                writeln!(rs, r#"            {value} => "{cpp}","#)?;
            )*}}}
            include!("../../../../../crates/winerr/src/ntstatus/facilities.rs");
        }
        writeln!(rs, r#"            v => return write!(fmt, "FACILITY::??? ({{v}})")"#)?;
        writeln!(rs, r#"        }};"#)?;
        writeln!(rs, r#"        write!(fmt, "{{}}", s)"#)?;
        writeln!(rs, r#"    }}"#)?;
        writeln!(rs, r#"}}"#)?;

        for ty in types.split(' ') {
            writeln!(rs)?;
            writeln!(rs, r#"impl Debug for {ty} {{"#)?;
            writeln!(rs, r#"    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {{"#)?;
            writeln!(rs, r#"        let s = match self.0 {{"#)?;
            match ty {
                "NTSTATUS"              => writeln!(rs, r#"            0 => "STATUS::SUCCESS","#)?,
                "ErrorCodeMicrosoft"    => writeln!(rs, r#"            0 => "ERROR::SUCCESS","#)?,
                _                       => {},
            }
            for (_rs_mod, codes) in codes.mods.iter() {
                for code in codes.iter() {
                    if code.redundant { continue }
                    if !code.matches_ty(ty) { continue }
                    #[allow(unused_variables)] let scan::Code { cpp, rs_mod, rs_id, rs_ty, rs_value, docs, redundant, hide } = &code;
                    if *hide { continue }
                    writeln!(rs, r#"            {rs_value} => "{rs_mod}::{rs_id}","#)?;
                }
            }
            match ty {
                "HRESULT"               => writeln!(rs, r#"            v => return write!(fmt, "HRESULT({{v:#X}})")"#)?,
                "NTSTATUS"              => writeln!(rs, r#"            v => return write!(fmt, "NTSTATUS({{v:#X}})")"#)?,
                "ErrorCodeMicrosoft"    => writeln!(rs, r#"            v => return write!(fmt, "ERROR::??? ({{v}})")"#)?,
                "SuccessCodeMicrosoft"  => writeln!(rs, r#"            v => return write!(fmt, "S::??? ({{v}})")"#)?,
                _                       => writeln!(rs, r#"            v => return write!(fmt, "ERROR::??? ({{v}})")"#)?,
            }
            writeln!(rs, r#"        }};"#)?;
            writeln!(rs, r#"        write!(fmt, "{{}}", s)"#)?;
            writeln!(rs, r#"    }}"#)?;
            writeln!(rs, r#"}}"#)?;
        }

        Ok(())
    }).unwrap();

    mmrbi::fs::write_if_modified_with("crates/winerr/src/errors.natvis", |nv|{
        use std::io::{Write as _};

        writeln!(nv, r#"<?xml version="1.0" encoding="utf-8"?>"#)?;
        writeln!(nv, r#"<!-- WARNING: this file is auto-generated by xtask gen and may be overwritten -->"#)?;
        writeln!(nv)?;
        writeln!(nv, r#"<AutoVisualizer xmlns="http://schemas.microsoft.com/vstudio/debugger/natvis/2010">"#)?;

        writeln!(nv)?;
        writeln!(nv, r#"    <Type Name="winerr_core::FacilityHrMicrosoft">"#)?;
        {
            macro_rules! microsoft_hresult_facilities {($(
                #define $prefix:ident $f:ident $value:literal
            )*) => {{$(
                let value = $value;
                let cpp = concat!(stringify!($prefix), stringify!($f));
                writeln!(nv, r#"        <DisplayString Condition="__0 == {value}">{cpp}</DisplayString>"#)?;
            )*}}}
            include!("../../../../../crates/winerr/src/hresult/extra.facilities.rs");
            include!("../../../../../crates/winerr/src/hresult/winerror.facilities.rs");
        }
        writeln!(nv, r#"        <DisplayString>FACILITY_??? ({{__0}})</DisplayString>"#)?;
        writeln!(nv, r#"    </Type>"#)?;

        writeln!(nv)?;
        writeln!(nv, r#"    <Type Name="winerr_core::FacilityNtStatusMicrosoft">"#)?;
        {
            macro_rules! microsoft_ntstatus_facilities {($(
                #define $prefix:ident $f:ident $value:literal
            )*) => {{$(
                let value = $value;
                let cpp = concat!(stringify!($prefix), stringify!($f));
                writeln!(nv, r#"        <DisplayString Condition="__0 == {value}">{cpp}</DisplayString>"#)?;
            )*}}}
            include!("../../../../../crates/winerr/src/ntstatus/facilities.rs");
        }
        writeln!(nv, r#"        <DisplayString>FACILITY_??? ({{__0}})</DisplayString>"#)?;
        writeln!(nv, r#"    </Type>"#)?;

        for ty in types.split(' ') {
            writeln!(nv)?;
            writeln!(nv, r#"    <Type Name="winerr_core::{ty}">"#)?;
            match ty {
                "HRESULT"               => writeln!(nv, r#"        <DisplayString Condition="__0 == 0">S::OK</DisplayString>"#)?,
                "SuccessHResult"        => writeln!(nv, r#"        <DisplayString Condition="__0 == 0">S::OK</DisplayString>"#)?,
                "ErrorHResult"          => writeln!(nv, r#"        <DisplayString Condition="__0 == 0">E::??? (invalid: value 0, but ErrorHResult s start at 0x8000_0000</DisplayString>"#)?,
                "NTSTATUS"              => writeln!(nv, r#"        <DisplayString Condition="__0 == 0">STATUS::SUCCESS</DisplayString>"#)?,
                "ErrorCodeMicrosoft"    => writeln!(nv, r#"        <DisplayString Condition="__0 == 0">ERROR::SUCCESS (invalid: value 0, ErrorCodeMicrosoft s should never be successful</DisplayString>"#)?,
                _                       => {},
            }
            for (_rs_mod, codes) in codes.mods.iter() {
                for code in codes.iter() {
                    if code.redundant { continue }
                    if !code.matches_ty(ty) { continue }
                    #[allow(unused_variables)] let scan::Code { cpp, rs_mod, rs_id, rs_ty, rs_value, docs, redundant, hide } = &code;
                    if *hide { continue }
                    writeln!(nv, r#"        <DisplayString Condition="__0 == {rs_value}">{rs_mod}::{rs_id}</DisplayString>"#)?;
                }
            }

            match ty {
                // https://docs.microsoft.com/en-us/openspecs/windows_protocols/ms-erref/0642cb2f-2075-4469-918c-4441e69c548a
                "HRESULT" | "SuccessHResult" | "ErrorHResult" => {
                    writeln!(nv, r#"        <DisplayString>{ty}({{__0,X}})</DisplayString>"#)?;
                    writeln!(nv, r#"        <Expand>"#)?;
                    writeln!(nv, r#"            <Item Name="S (failure)" >((__0 &amp; 0x80000000) != 0)</Item>"#)?;
                    writeln!(nv, r#"            <Item Name="R (reserved)">((__0 &amp; 0x40000000) != 0)</Item>"#)?;
                    writeln!(nv, r#"            <Item Name="C (customer)">((__0 &amp; 0x20000000) != 0)</Item>"#)?;
                    writeln!(nv, r#"            <Item Name="N (NTSTATUS)">((__0 &amp; 0x10000000) != 0)</Item>"#)?;
                    writeln!(nv)?;
                    writeln!(nv, r#"            <!-- HRESULT -->"#)?;
                    writeln!(nv, r#"            <Synthetic Name="Facility" Condition="(__0 &amp; 0x30000000) == 0x00000000">"#)?;
                    {
                        macro_rules! microsoft_hresult_facilities {($(
                            #define $prefix:ident $f:ident $value:literal
                        )*) => {{$(
                            let value = $value;
                            let cpp = concat!(stringify!($prefix), stringify!($f));
                            writeln!(nv, r#"                <DisplayString Condition="((__0 >> 16) &amp; 0xFFF) == {value}">{cpp}</DisplayString>"#)?;
                        )*}}}
                        include!("../../../../../crates/winerr/src/hresult/extra.facilities.rs");
                        include!("../../../../../crates/winerr/src/hresult/winerror.facilities.rs");
                        writeln!(nv, r#"                <DisplayString>{{(__0 >> 16) &amp; 0xFFF}}</DisplayString>"#)?;
                    }
                    writeln!(nv, r#"            </Synthetic>"#)?;
                    writeln!(nv)?;
                    writeln!(nv, r#"            <!-- NTSTATUS -->"#)?;
                    writeln!(nv, r#"            <Synthetic Name="Facility" Condition="(__0 &amp; 0x30000000) == 0x10000000">"#)?;
                    {
                        macro_rules! microsoft_ntstatus_facilities {($(
                            #define $prefix:ident $f:ident $value:literal
                        )*) => {{$(
                            let value = $value;
                            let cpp = concat!(stringify!($prefix), stringify!($f));
                            writeln!(nv, r#"                <DisplayString Condition="((__0 >> 16) &amp; 0xFFF) == {value}">{cpp}</DisplayString>"#)?;
                        )*}}}
                        include!("../../../../../crates/winerr/src/ntstatus/facilities.rs");
                        writeln!(nv, r#"                <DisplayString>{{(__0 >> 16) &amp; 0xFFF}}</DisplayString>"#)?;
                    }
                    writeln!(nv, r#"            </Synthetic>"#)?;
                    writeln!(nv)?;
                    writeln!(nv, r#"            <!-- Customer -->"#)?;
                    writeln!(nv, r#"            <Item      Name="Facility" Condition="(__0 &amp; 0x20000000) == 0x20000000">((__0 &amp; 0x0FFF0000) >>16)</Item>"#)?;
                    writeln!(nv)?;
                    writeln!(nv, r#"            <Item Name="Code"        >((__0 &amp; 0x0000FFFF) >> 0)</Item>"#)?;
                    writeln!(nv, r#"        </Expand>"#)?;
                },
                // https://docs.microsoft.com/en-us/openspecs/windows_protocols/ms-erref/87fba13e-bf06-450e-83b1-9241dc81e781
                "NTSTATUS" => {
                    writeln!(nv, r#"        <DisplayString>{ty}({{__0,X}})</DisplayString>"#)?;
                    writeln!(nv, r#"        <Expand>"#)?;
                    //writeln!(nv, r#"            <Item Name="Sev"         >((__0 &amp; 0xC0000000) >>30)</Item>"#)?;
                    writeln!(nv, r#"            <Synthetic Name="Sev">"#)?;
                    writeln!(nv, r#"                <DisplayString Condition="(__0 >> 30) == 0">STATUS::SEVERITY::SUCCESS</DisplayString>"#)?;
                    writeln!(nv, r#"                <DisplayString Condition="(__0 >> 30) == 1">STATUS::SEVERITY::INFORMATIONAL</DisplayString>"#)?;
                    writeln!(nv, r#"                <DisplayString Condition="(__0 >> 30) == 2">STATUS::SEVERITY::WARNING</DisplayString>"#)?;
                    writeln!(nv, r#"                <DisplayString Condition="(__0 >> 30) == 3">STATUS::SEVERITY::ERROR</DisplayString>"#)?;
                    writeln!(nv, r#"            </Synthetic>"#)?;
                    writeln!(nv, r#"            <Item Name="C (customer)">((__0 &amp; 0x20000000) != 0)</Item>"#)?;
                    writeln!(nv, r#"            <Item Name="N (reserved)">((__0 &amp; 0x10000000) != 0)</Item>"#)?; // "must be set to 0", but surely this is set to 1...? maybe?
                    writeln!(nv)?;
                    writeln!(nv, r#"            <!-- NTSTATUS -->"#)?;
                    writeln!(nv, r#"            <Synthetic Name="Facility" Condition="(__0 &amp; 0x20000000) == 0">"#)?;
                    {
                        macro_rules! microsoft_ntstatus_facilities {($(
                            #define $prefix:ident $f:ident $value:literal
                        )*) => {{$(
                            let value = $value;
                            let cpp = concat!(stringify!($prefix), stringify!($f));
                            writeln!(nv, r#"                <DisplayString Condition="((__0 >> 16) &amp; 0xFFF) == {value}">{cpp}</DisplayString>"#)?;
                        )*}}}
                        include!("../../../../../crates/winerr/src/ntstatus/facilities.rs");
                        writeln!(nv, r#"                <DisplayString>{{(__0 >> 16) &amp; 0xFFF}}</DisplayString>"#)?;
                    }
                    writeln!(nv, r#"            </Synthetic>"#)?;
                    writeln!(nv)?;
                    writeln!(nv, r#"            <!-- Customer -->"#)?;
                    writeln!(nv, r#"            <Item      Name="Facility" Condition="(__0 &amp; 0x20000000) != 0">((__0 &amp; 0x0FFF0000) >>16)</Item>"#)?;
                    writeln!(nv)?;
                    writeln!(nv, r#"            <Item Name="Code"        >((__0 &amp; 0x0000FFFF) >> 0)</Item>"#)?;
                    writeln!(nv, r#"        </Expand>"#)?;
                },
                "ErrorCodeMicrosoft" => {
                    writeln!(nv, r#"        <DisplayString>ERROR::??? ({{__0}})</DisplayString>"#)?;
                },
                "SuccessCodeMicrosoft" => {
                    writeln!(nv, r#"        <DisplayString>S::??? ({{__0}})</DisplayString>"#)?;
                },
                _ => {
                },
            }

            writeln!(nv, r#"    </Type>"#)?;
        }

        writeln!(nv)?;
        writeln!(nv, r#"</AutoVisualizer>"#)?;

        Ok(())
    }).unwrap();
}

pub fn readme() {
    let winerr_doc_intro_md = std::fs::read_to_string("crates/winerr/doc/intro.md"  ).expect("crates/winerr/doc/intro.md");
    let license_md          = std::fs::read_to_string("LICENSE.md"                  ).expect("LICENSE.md");

    for path in "Readme.md crates/winerr/Readme.md".split(' ') {
        mmrbi::fs::write_if_modified_with(path, |md|{
            use std::io::Write as _;
            writeln!(md, "<!-- WARNING: auto generated by xtask gen, this file will be overwritten -->")?;
            writeln!(md)?;
            writeln!(md)?;
            writeln!(md)?;
            writeln!(md, "<!-- crates/winerr/doc/intro.md -->")?;
            writeln!(md)?;
            writeln!(md, "{winerr_doc_intro_md}")?;
            writeln!(md)?;
            writeln!(md)?;
            writeln!(md)?;
            writeln!(md, "<!-- LICENSE.md -->")?;
            writeln!(md)?;
            write!(md, "{license_md}")
        }).unwrap();
    }
}
