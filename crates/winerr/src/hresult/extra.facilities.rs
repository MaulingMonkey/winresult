microsoft_hresult_facilities! {
    // These are synthetic FACILITY_* constants that Microsoft uses but didn't give a proper name

    // _FACD3D
    #define FACILITY_ DIRECT3D9     0x876

    // _FACDD - yes, note that this overlaps _FACD3D
    #define FACILITY_ DIRECTDRAW    0x876
}
