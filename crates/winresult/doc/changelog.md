# Changelog

Version numbers are for the main crate, `winresult`.

## 0.1.3
*   New error codes:    `XAUDIO2::E_*`
*   Improved interop:   `impl From<Infallible> for ...`

## 0.1.2
*   New error codes
    *   `D3DXERR::*`
    *   `D3DXFERR::*`
    *   Windows SDK `10.0.19041.0` -> `10.0.22621.0`
*   Improved interop
    *   `impl From<HResultSuccess> for i32`
    *   `impl From<i32> for HResult`
    *   `impl PartialEq<...> for ...` for a whole bunch of stuff

## 0.1.1
*   Fix docs.rs / linux doc builds

## 0.1.0
*   Initial release!
