use std::ffi::CStr;

#[derive(Clone, Copy, Debug)]
#[allow(dead_code, non_camel_case_types)]
pub enum Target {
    // FL 11.0 / 11.1
    cs_5_0,
    ds_5_0,
    gs_5_0,
    hs_5_0,
    ps_5_0,
    vs_5_0,

    // FL 10.1
    cs_4_1,
    gs_4_1,
    ps_4_1,
    vs_4_1,

    // FL 10.0
    cs_4_0,
    gs_4_0,
    ps_4_0,
    vs_4_0,

    // FL 9.x
    ps_4_0_level_9_1,
    ps_4_0_level_9_3,
    vs_4_0_level_9_1,
    vs_4_0_level_9_3,

    // D3D9
    ps_3_0,
    ps_3_sw,
    vs_3_0,
    vs_3_sw,
    ps_2_0,
    ps_2_a,
    ps_2_b,
    ps_2_sw,
    vs_2_0,
    vs_2_a,
    vs_2_sw,
    tx_1_0,
    vs_1_1,

    // Effects
    fx_2_0,
    fx_4_0,
    fx_4_1,
    fx_5_0,
}

impl Target {
    pub fn to_cstr (self) -> &'static CStr {
        CStr::from_bytes_with_nul(match self {
            // FL 11.0 / 11.1
            Target::cs_5_0              => b"cs_5_0\0",
            Target::ds_5_0              => b"ds_5_0\0",
            Target::gs_5_0              => b"gs_5_0\0",
            Target::hs_5_0              => b"hs_5_0\0",
            Target::ps_5_0              => b"ps_5_0\0",
            Target::vs_5_0              => b"vs_5_0\0",

            // FL 10.1
            Target::cs_4_1              => b"cs_4_1\0",
            Target::gs_4_1              => b"gs_4_1\0",
            Target::ps_4_1              => b"ps_4_1\0",
            Target::vs_4_1              => b"vs_4_1\0",

            // fl 10.0
            Target::cs_4_0              => b"cs_4_0\0",
            Target::gs_4_0              => b"gs_4_0\0",
            Target::ps_4_0              => b"ps_4_0\0",
            Target::vs_4_0              => b"vs_4_0\0",

            // FL 9.X
            Target::ps_4_0_level_9_1    => b"ps_4_0_level_9_1\0",
            Target::ps_4_0_level_9_3    => b"ps_4_0_level_9_3\0",
            Target::vs_4_0_level_9_1    => b"vs_4_0_level_9_1\0",
            Target::vs_4_0_level_9_3    => b"vs_4_0_level_9_3\0",

            // D3D9
            Target::ps_3_0              => b"ps_3_0\0",
            Target::ps_3_sw             => b"ps_3_sw\0",
            Target::vs_3_0              => b"vs_3_0\0",
            Target::vs_3_sw             => b"vs_3_sw\0",
            Target::ps_2_0              => b"ps_2_0\0",
            Target::ps_2_a              => b"ps_2_a\0",
            Target::ps_2_b              => b"ps_2_b\0",
            Target::ps_2_sw             => b"ps_2_sw\0",
            Target::vs_2_0              => b"vs_2_0\0",
            Target::vs_2_a              => b"vs_2_a\0",
            Target::vs_2_sw             => b"vs_2_sw\0",
            Target::tx_1_0              => b"tx_1_0\0",
            Target::vs_1_1              => b"vs_1_1\0",

            // Effects
            Target::fx_2_0              => b"fx_2_0\0",
            Target::fx_4_0              => b"fx_4_0\0",
            Target::fx_4_1              => b"fx_4_1\0",
            Target::fx_5_0              => b"fx_5_0\0",
        }).unwrap()
    }
}
