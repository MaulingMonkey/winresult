mod gen;
mod scan;

use maulingmonkey_windows_sdk_scanner::*;
use std::path::*;



struct Header {
    pub path: PathBuf,
    pub code: String,
}

impl Header {
    pub fn lines<'h>(&'h self) -> impl Iterator<Item = HeaderLine<'h>> + 'h { self.code.lines().enumerate().map(|(idx, text)| HeaderLine { idx, text }) }
}

struct HeaderLine<'s> {
    pub text:   &'s str,
    idx:    usize,
}

#[allow(dead_code)] impl<'s> HeaderLine<'s> {
    pub fn idx(&self) -> usize { self.idx }
    pub fn no(&self) -> usize { self.idx + 1 }
}

fn main() {
    assert!(Path::new(".git").exists(), "expected to be run in the root of this repository");
    let sdk = sdk::WindowsKit::find_latest().expect("sdk");

    let actual_sdk = sdk.sdk_version.to_string();
    let expected_sdk ="10.0.19041.0";
    if actual_sdk != expected_sdk { mmrbi::warning!("expected sdk {} but found sdk {}", expected_sdk, actual_sdk) }


    macro_rules! headers { ( $( $name:ident => $path:literal ),* $(,)? ) => {$(
        let $name = {
            let path = sdk.include.join($path);
            let code = std::fs::read_to_string(&path).expect($path);
            Header { path, code }
        };
    )*}}

    headers! {
        d3d9_h          => r"shared\d3d9.h",
        ntstatus_h      => r"shared\ntstatus.h",
        winerror_h      => r"shared\winerror.h",

        d3d_h           => r"um\d3d.h",
        d3d9helper_h    => r"um\d3d9helper.h",
        d3dhal_h        => r"um\d3dhal.h",
    }

    let mut codes   = scan::Codes::default();
    scan::winerror_h    (&winerror_h,   &mut codes);
    scan::d3d           (&d3d9_h,       &mut codes);
    scan::d3d           (&d3d_h,        &mut codes);
    scan::d3d           (&d3d9helper_h, &mut codes);
    scan::d3d           (&d3dhal_h,     &mut codes);
    scan::ntstatus_h    (&ntstatus_h,   &mut codes);

    gen::codes(&codes);
    gen::readme();
}
