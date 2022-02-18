mod gen;
mod scan;

use maulingmonkey_windows_sdk_scanner::*;
use std::path::*;



struct Header {
    pub path: PathBuf,
    pub code: String,
    pub eols: Vec<usize>,
}

impl Header {
    pub fn read(sdk: &sdk::WindowsKit, path_h: &str) -> Self {
        let path = sdk.include.join(path_h);
        let code = std::fs::read_to_string(&path).expect(path_h);
        let bytes = code.as_bytes();
        let eols = bytes.into_iter().enumerate().filter(|l| *l.1 == b'\n').map(|l| l.0).chain([bytes.len()]).collect();
        Self { path, code, eols }
    }

    pub fn line<'h>(&'h self, idx: usize) -> Option<HeaderLine<'h>> {
        let start = if idx == 0 { 0 } else { *self.eols.get(idx-1)? + 1 };
        let end   = *self.eols.get(idx-0)?;
        Some(HeaderLine { idx, text: &self.code.get(start..end)? })
    }

    pub fn lines<'h>(&'h self) -> impl Iterator<Item = HeaderLine<'h>> + 'h { self.code.lines().enumerate().map(|(idx, text)| HeaderLine { idx, text }) }
}

struct HeaderLine<'s> {
    pub text:   &'s str,
    idx:        usize,
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
        let $name = Header::read(&sdk, $path);
    )*}}

    headers! {
        ntstatus_h      => r"shared\ntstatus.h",
        winerror_h      => r"shared\winerror.h",
        // already covered by winerror.h:
        //  um\fltWinError.h
        //  um\Msi.h

        d3d9_h          => r"shared\d3d9.h",
        d3d_h           => r"um\d3d.h",
        d3d9helper_h    => r"um\d3d9helper.h",
        d3dhal_h        => r"um\d3dhal.h",

        // TODO:
        //  dhcpsapi_h      => r"um\dhcpsapi.h",
        //  mprerror_h      => r"um\MprError.h",
        //  netsh_h         => r"um\NetSh.h",
        //  patchapi_h      => r"um\PatchApi.h",
        //  patchwiz_h      => r"um\PatchWiz.h",
        //  raserror_h      => r"um\RasError.h",
        //  setupapi_h      => r"um\SetupAPI.h",
        //  tcerror_h       => r"um\TCError.h",
        //  winhttp_h       => r"um\winhttp.h",
        //  wininet_h       => r"um\WinInet.h",
        //  winineti_h      => r"um\Winineti.h",
        //  winioctl_h      => r"um\winioctl.h",
        //  winspool_h      => r"um\winspool.h",
        //  wsmerror        => r"um\wsmerror.h",
    }

    let mut codes   = scan::Codes::default();
    scan::ntstatus_h    (&ntstatus_h,   &mut codes);
    scan::winerror_h    (&winerror_h,   &mut codes);

    scan::d3d           (&d3d9_h,       &mut codes);
    scan::d3d           (&d3d_h,        &mut codes);
    scan::d3d           (&d3d9helper_h, &mut codes);
    scan::d3d           (&d3dhal_h,     &mut codes);

    //scan::winerror_h    (&dhcpsapi_h,   &mut codes);
    //scan::winerror_h    (&mprerror_h,   &mut codes);
    //scan::winerror_h    (&netsh_h,      &mut codes);
    //scan::winerror_h    (&patchapi_h,   &mut codes);
    //scan::winerror_h    (&patchwiz_h,   &mut codes);
    //scan::winerror_h    (&raserror_h,   &mut codes);
    //scan::winerror_h    (&setupapi_h,   &mut codes);
    //scan::winerror_h    (&tcerror_h,    &mut codes);
    //scan::winerror_h    (&winhttp_h,    &mut codes);
    //scan::winerror_h    (&wininet_h,    &mut codes);
    //scan::winerror_h    (&winineti_h,   &mut codes);
    //scan::winerror_h    (&winioctl_h,   &mut codes);
    //scan::winerror_h    (&winspool_h,   &mut codes);
    //scan::winerror_h    (&wsmerror,     &mut codes);

    gen::codes(&codes);
    gen::readme();
}
