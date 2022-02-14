use winerr::*;

fn main() {
    let _fhr = dbg!(FACILITY::WIN32);
    let _fnt = dbg!(FACILITY::WIN32K_NTGDI);
    let _err = dbg!(ERROR::ACCESS_DENIED);
    let _d3d = dbg!(D3DERR::NOTFOUND);
    let _pix = dbg!(HRESULT::from_constant(0x8ABC0000));
    let _unk = dbg!(HRESULT::from_constant(0x8ABD0000));
    let _er8 = dbg!(HRESULT::from_constant(0x80010000));
    let _er9 = dbg!(HRESULT::from_constant(0x90010000));
    let _era = dbg!(HRESULT::from_constant(0xA0010000));
    let _erb = dbg!(HRESULT::from_constant(0xB0010000));
    let _nts = dbg!(NTSTATUS::from_constant(0xC0010000));
    let _i = 42;
}
