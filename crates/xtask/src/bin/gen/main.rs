mod gen;
mod scan;

use maulingmonkey_windows_sdk_scanner::*;
use std::path::*;



fn main() {
    assert!(Path::new(".git").exists(), "expected to be run in the root of this repository");
    let sdk = sdk::WindowsKit::find_latest().expect("sdk");

    let actual_sdk = sdk.sdk_version.to_string();
    let expected_sdk ="10.0.19041.0";
    if actual_sdk != expected_sdk { mmrbi::warning!("expected sdk {} but found sdk {}", expected_sdk, actual_sdk) }

    let d3d9_h      = std::fs::read_to_string(sdk.include.join(r"shared\d3d9.h"     )).expect("d3d9.h"      );
    let ntstatus_h  = std::fs::read_to_string(sdk.include.join(r"shared\ntstatus.h" )).expect("ntstatus.h"  );
    let winerror_h  = std::fs::read_to_string(sdk.include.join(r"shared\winerror.h" )).expect("winerror.h"  );

    let mut codes   = scan::Codes::default();
    scan::hardcoded     (               &mut codes);
    scan::winerror_h    (&winerror_h,   &mut codes);
    scan::d3d9_h        (&d3d9_h,       &mut codes);
    scan::ntstatus_h    (&ntstatus_h,   &mut codes);

    gen::codes(&codes);
    gen::readme();
}
