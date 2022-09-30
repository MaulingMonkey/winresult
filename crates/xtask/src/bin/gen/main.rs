mod crlf;
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
    pub fn read(sdk_include: &Path, path_h: &str) -> Self {
        let path = sdk_include.join(path_h);
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

    let mut codes   = scan::Codes::default();

    macro_rules! headers { ( $( $path:literal : $scan_fn:path ),* $(,)? ) => {$(
        let header = Header::read(&sdk.include, $path);
        $scan_fn(&header, &mut codes);
    )*}}

    headers! {
        r"shared\ntstatus.h":   scan::ntstatus_h,
        r"shared\winerror.h":   scan::winerror_h,
        //r"um\fltWinError.h":  scan::winerror_h, // already covered by winerror.h
        //r"um\Msi.h":          scan::winerror_h, // already covered by winerror.h

        r"shared\d3d9.h":       scan::d3d,
        r"um\d3d.h":            scan::d3d,
        r"um\d3d9helper.h":     scan::d3d,
        r"um\d3dhal.h":         scan::d3d,

        r"um\dhcpsapi.h":       scan::winerror_h,
        r"um\MprError.h":       scan::winerror_h,
        r"um\NetSh.h":          scan::winerror_h,
        r"um\PatchApi.h":       scan::winerror_h,
        r"um\PatchWiz.h":       scan::winerror_h,
        r"um\RasError.h":       scan::winerror_h,
        r"um\SetupAPI.h":       scan::winerror_h,
        r"um\TCError.h":        scan::winerror_h,
        r"um\WerApi.h":         scan::winerror_h,
        r"um\winhttp.h":        scan::winerror_h,
        r"um\WinInet.h":        scan::winerror_h,
        r"um\Winineti.h":       scan::winerror_h,
        //r"um\winioctl.h":     scan::winerror_h, // will need to be handled manually, too many bespoke error types
        r"um\winspool.h":       scan::winerror_h,
        r"um\wsmerror.h":       scan::winerror_h,
    }

    let d3d_sdk = Path::new(r"C:\Program Files (x86)\Microsoft DirectX SDK (June 2010)\Include");
    if !d3d_sdk.exists() { mmrbi::fatal!("DirectX SDK is not installed or cannot be found.\n  Install from https://www.microsoft.com/en-us/download/details.aspx?id=6812\n  {}\\", d3d_sdk.display()) }

    macro_rules! headers { ( $( $path:literal : $scan_fn:path ),* $(,)? ) => {$(
        let header = Header::read(d3d_sdk, $path);
        $scan_fn(&header, &mut codes);
    )*}}

    headers! {
        //r"D2Derr.h":            scan::d3d,
        //r"d3d9.h":              scan::d3d, // already covered by the windows sdk
        r"d3dx9.h":             scan::d3d,
        r"d3dx9xof.h":          scan::d3d,
        //r"D3DX10.h":            scan::d3d,
        //r"D3DX10core.h":        scan::d3d,
        //r"D3DX11.h":            scan::d3d,
        //r"D3DX11core.h":        scan::d3d,
        //r"dinput.h":            scan::d3d,
        //r"dinputd.h":           scan::d3d,
        //r"dsetup.h":            scan::d3d,
        //r"dsound.h":            scan::d3d,
        //r"DWrite.h":            scan::d3d,
        //r"DxErr.h":             scan::d3d, // tracing macros only?
        //r"dxfile.h":            scan::d3d,
    }

    gen::codes(&codes);
    gen::readme();
    gen::licenses();
}
