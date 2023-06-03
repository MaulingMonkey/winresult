# 🦀 winresult - windows result codes 🦀

Debug-friendly types for windows result codes.

[![GitHub](https://img.shields.io/github/stars/MaulingMonkey/winresult.svg?label=GitHub&style=social)](https://github.com/MaulingMonkey/winresult)
[![crates.io](https://img.shields.io/crates/v/winresult.svg)](https://crates.io/crates/winresult)
[![docs.rs](https://img.shields.io/docsrs/winresult)](https://docs.rs/winresult)
[![License](https://img.shields.io/crates/l/winresult.svg)](https://github.com/MaulingMonkey/winresult)



### Why?

*   `u32` error codes are annoying to `dbg!(...)`.  `winresult` has awesome [`Debug`] impls.
*   `u32` error codes are annoying to view in your debugger.  `winresult` has awesome \*.natvis files.  Use [`natvis-pdbs`]!
*   typoing `ERROR_WHATEVER` in a `match` is a mere warning. `ERROR::WHATEVER` is a hard error. <br> (I'd still use `#![deny(unreachable_patterns)]` anyways.)
*   `ERROR_INVALID_FUNCTION` == `S_FALSE` (== `1`.)  Lame!
*   `ERROR_FILE_NOT_FOUND` (2) is a mess.  A function or [`GetLastError`] might return:

    | label                                         | value         | notes |
    | --------------------------------------------- | ------------- | ----- |
    | `ERROR_FILE_NOT_FOUND`                        | `0x00000002`  | Not an [`HRESULT`] (would be "successful") |
    | `HRESULT_FROM_WIN32(ERROR_FILE_NOT_FOUND)`    | `0x80070002`  | [hresult.info](https://www.hresult.info/Search?q=ERROR_FILE_NOT_FOUND) "incorrectly" labels `ERROR_*` as this |
    | `NTSTATUS_FROM_WIN32(ERROR_FILE_NOT_FOUND)`   | `0xC0070002`  | IDK how frequently [`NtStatus`]es get shoved into [`GetLastError`], but I've seen weirder
    | `D3D10_ERROR_FILE_NOT_FOUND`                  | `0x88790002`  | Different facility, same code |
    | `D3D11_ERROR_FILE_NOT_FOUND`                  | `0x887C0002`  | Different facility, same code |

*   `ERROR_*` is a mixture of [`HRESULT`]s and non-[`HRESULT`]s.  Can you keep them straight?  No.  No you cannot.  Stop lying.

[`Debug`]:          https://doc.rust-lang.org/std/fmt/trait.Debug.html
[`GetLastError`]:   https://learn.microsoft.com/en-us/windows/win32/api/errhandlingapi/nf-errhandlingapi-getlasterror
[`HRESULT`]:        https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-erref/0642cb2f-2075-4469-918c-4441e69c548a
[`natvis-pdbs`]:    https://crates.io/crates/natvis-pdbs
