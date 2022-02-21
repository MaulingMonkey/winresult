use winresult::*;

fn main() {
    let _fhr = dbg!(FACILITY::WIN32);
    let _fnt = dbg!(FACILITY::WIN32K_NTGDI);
    let _err = dbg!(ERROR::ACCESS_DENIED);
    let _d3d = dbg!(D3DERR::NOTFOUND);
    let _pix = dbg!(HResult::from_constant(0x8ABC0000));
    let _unk = dbg!(HResult::from_constant(0x8ABD0000));
    let _er8 = dbg!(HResult::from_constant(0x80010000));
    let _er9 = dbg!(HResult::from_constant(0x90010000));
    let _era = dbg!(HResult::from_constant(0xA0010000));
    let _erb = dbg!(HResult::from_constant(0xB0010000));
    let _nts = dbg!(NTSTATUS::from_constant(0xC0010000));
    let _sf  = dbg!(S::FALSE);
    let _wa  = dbg!(WAIT::OBJECT(42).unwrap());

    let _u1 = dbg!(ErrorHResultOrCode::from(_err));
    let _u2 = dbg!(ErrorHResultOrCode::from(_d3d));

    let _i = 42;
}
