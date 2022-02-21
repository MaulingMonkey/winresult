#![no_std]

mod hresult;    pub use hresult::*;
mod code;       pub use code::*;
mod ntstatus;   pub use ntstatus::*;
mod wait;       pub use wait::*;
mod unions;     pub use unions::*;

mod debug;
