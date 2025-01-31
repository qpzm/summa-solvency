pub use inclusion_verifier::*;
/// This module was auto-generated with ethers-rs Abigen.
/// More information at: <https://github.com/gakonst/ethers-rs>
#[allow(
    clippy::enum_variant_names,
    clippy::too_many_arguments,
    clippy::upper_case_acronyms,
    clippy::type_complexity,
    dead_code,
    non_camel_case_types,
)]
pub mod inclusion_verifier {
    #[rustfmt::skip]
    const __ABI: &str = "[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"vk\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"proofs\",\"type\":\"bytes\",\"components\":[]},{\"internalType\":\"uint256[]\",\"name\":\"challenges\",\"type\":\"uint256[]\",\"components\":[]},{\"internalType\":\"uint256[]\",\"name\":\"values\",\"type\":\"uint256[]\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"verifyProof\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]}]";
    ///The parsed JSON ABI of the contract.
    pub static INCLUSIONVERIFIER_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(|| {
        ::ethers::core::utils::__serde_json::from_str(__ABI)
            .expect("ABI is always valid")
    });
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = &[
        96,
        128,
        96,
        64,
        82,
        52,
        128,
        21,
        97,
        0,
        16,
        87,
        96,
        0,
        128,
        253,
        91,
        80,
        97,
        4,
        89,
        128,
        97,
        0,
        32,
        96,
        0,
        57,
        96,
        0,
        243,
        254,
        96,
        128,
        96,
        64,
        82,
        52,
        128,
        21,
        97,
        0,
        16,
        87,
        96,
        0,
        128,
        253,
        91,
        80,
        96,
        4,
        54,
        16,
        97,
        0,
        43,
        87,
        96,
        0,
        53,
        96,
        224,
        28,
        128,
        99,
        71,
        246,
        181,
        170,
        20,
        97,
        0,
        48,
        87,
        91,
        96,
        0,
        128,
        253,
        91,
        97,
        0,
        67,
        97,
        0,
        62,
        54,
        96,
        4,
        97,
        3,
        60,
        86,
        91,
        97,
        0,
        87,
        86,
        91,
        96,
        64,
        81,
        144,
        21,
        21,
        129,
        82,
        96,
        32,
        1,
        96,
        64,
        81,
        128,
        145,
        3,
        144,
        243,
        91,
        96,
        0,
        97,
        1,
        53,
        86,
        91,
        96,
        32,
        130,
        1,
        53,
        131,
        129,
        16,
        131,
        53,
        133,
        129,
        16,
        132,
        22,
        145,
        144,
        145,
        22,
        145,
        133,
        96,
        3,
        129,
        128,
        133,
        128,
        9,
        133,
        9,
        8,
        134,
        130,
        131,
        9,
        20,
        131,
        22,
        146,
        80,
        80,
        80,
        147,
        146,
        80,
        80,
        80,
        86,
        91,
        129,
        96,
        192,
        82,
        130,
        96,
        224,
        82,
        96,
        0,
        96,
        64,
        96,
        128,
        128,
        96,
        128,
        96,
        6,
        90,
        250,
        144,
        145,
        22,
        147,
        146,
        80,
        80,
        80,
        86,
        91,
        129,
        96,
        192,
        82,
        96,
        0,
        96,
        64,
        96,
        128,
        96,
        96,
        96,
        128,
        96,
        7,
        90,
        250,
        144,
        145,
        22,
        146,
        145,
        80,
        80,
        86,
        91,
        129,
        96,
        0,
        82,
        130,
        96,
        32,
        82,
        97,
        2,
        128,
        81,
        96,
        64,
        82,
        97,
        2,
        160,
        81,
        96,
        96,
        82,
        97,
        2,
        192,
        81,
        96,
        128,
        82,
        97,
        2,
        224,
        81,
        96,
        160,
        82,
        131,
        96,
        192,
        82,
        132,
        96,
        224,
        82,
        97,
        3,
        0,
        81,
        97,
        1,
        0,
        82,
        97,
        3,
        32,
        81,
        97,
        1,
        32,
        82,
        97,
        3,
        64,
        81,
        97,
        1,
        64,
        82,
        97,
        3,
        96,
        81,
        97,
        1,
        96,
        82,
        96,
        0,
        96,
        32,
        96,
        0,
        97,
        1,
        128,
        96,
        0,
        96,
        8,
        90,
        250,
        96,
        0,
        81,
        146,
        22,
        144,
        145,
        22,
        149,
        148,
        80,
        80,
        80,
        80,
        80,
        86,
        91,
        127,
        48,
        100,
        78,
        114,
        225,
        49,
        160,
        41,
        184,
        80,
        69,
        182,
        129,
        129,
        88,
        93,
        151,
        129,
        106,
        145,
        104,
        113,
        202,
        141,
        60,
        32,
        140,
        22,
        216,
        124,
        253,
        71,
        127,
        48,
        100,
        78,
        114,
        225,
        49,
        160,
        41,
        184,
        80,
        69,
        182,
        129,
        129,
        88,
        93,
        40,
        51,
        232,
        72,
        121,
        185,
        112,
        145,
        67,
        225,
        245,
        147,
        240,
        0,
        0,
        1,
        96,
        1,
        96,
        192,
        97,
        1,
        96,
        97,
        2,
        64,
        141,
        60,
        96,
        132,
        53,
        96,
        128,
        129,
        6,
        96,
        0,
        20,
        130,
        22,
        145,
        80,
        129,
        97,
        1,
        155,
        87,
        96,
        0,
        128,
        253,
        91,
        96,
        32,
        129,
        96,
        132,
        1,
        1,
        128,
        53,
        96,
        4,
        129,
        20,
        132,
        22,
        147,
        80,
        131,
        97,
        1,
        182,
        87,
        96,
        0,
        128,
        253,
        91,
        96,
        32,
        130,
        1,
        53,
        97,
        3,
        0,
        82,
        96,
        64,
        130,
        1,
        53,
        97,
        3,
        32,
        82,
        96,
        96,
        130,
        1,
        53,
        97,
        3,
        64,
        82,
        96,
        128,
        130,
        1,
        53,
        97,
        3,
        96,
        82,
        96,
        32,
        128,
        130,
        2,
        131,
        1,
        1,
        145,
        80,
        80,
        128,
        53,
        96,
        32,
        129,
        2,
        131,
        4,
        96,
        4,
        20,
        132,
        22,
        147,
        80,
        131,
        97,
        1,
        254,
        87,
        96,
        0,
        128,
        253,
        91,
        96,
        0,
        91,
        129,
        129,
        16,
        21,
        97,
        2,
        226,
        87,
        97,
        2,
        64,
        81,
        96,
        128,
        82,
        97,
        2,
        96,
        81,
        96,
        160,
        82,
        96,
        32,
        129,
        129,
        2,
        132,
        1,
        1,
        53,
        134,
        3,
        96,
        192,
        129,
        144,
        82,
        96,
        64,
        130,
        2,
        97,
        2,
        53,
        130,
        136,
        97,
        0,
        174,
        86,
        91,
        135,
        22,
        150,
        80,
        134,
        97,
        2,
        67,
        87,
        96,
        0,
        128,
        253,
        91,
        128,
        96,
        2,
        135,
        4,
        96,
        164,
        1,
        1,
        145,
        80,
        97,
        2,
        89,
        137,
        131,
        137,
        97,
        0,
        94,
        86,
        91,
        150,
        80,
        134,
        97,
        2,
        101,
        87,
        96,
        0,
        128,
        253,
        91,
        96,
        32,
        130,
        1,
        53,
        145,
        53,
        97,
        2,
        119,
        131,
        130,
        138,
        97,
        0,
        143,
        86,
        91,
        151,
        80,
        80,
        134,
        97,
        2,
        132,
        87,
        96,
        0,
        128,
        253,
        91,
        96,
        128,
        81,
        97,
        2,
        0,
        82,
        96,
        160,
        81,
        97,
        2,
        32,
        82,
        96,
        164,
        1,
        144,
        80,
        97,
        2,
        162,
        136,
        130,
        136,
        97,
        0,
        94,
        86,
        91,
        149,
        80,
        133,
        97,
        2,
        174,
        87,
        96,
        0,
        128,
        253,
        91,
        128,
        53,
        96,
        32,
        130,
        1,
        53,
        145,
        80,
        97,
        2,
        202,
        130,
        130,
        97,
        2,
        32,
        81,
        97,
        2,
        0,
        81,
        139,
        97,
        0,
        201,
        86,
        91,
        135,
        22,
        150,
        80,
        80,
        80,
        132,
        97,
        2,
        218,
        87,
        96,
        0,
        128,
        253,
        91,
        96,
        1,
        1,
        97,
        2,
        1,
        86,
        91,
        80,
        80,
        80,
        80,
        128,
        96,
        0,
        82,
        96,
        32,
        96,
        0,
        243,
        91,
        96,
        0,
        128,
        131,
        96,
        31,
        132,
        1,
        18,
        97,
        3,
        2,
        87,
        96,
        0,
        128,
        253,
        91,
        80,
        129,
        53,
        103,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        129,
        17,
        21,
        97,
        3,
        26,
        87,
        96,
        0,
        128,
        253,
        91,
        96,
        32,
        131,
        1,
        145,
        80,
        131,
        96,
        32,
        130,
        96,
        5,
        27,
        133,
        1,
        1,
        17,
        21,
        97,
        3,
        53,
        87,
        96,
        0,
        128,
        253,
        91,
        146,
        80,
        146,
        144,
        80,
        86,
        91,
        96,
        0,
        128,
        96,
        0,
        128,
        96,
        0,
        128,
        96,
        0,
        96,
        128,
        136,
        138,
        3,
        18,
        21,
        97,
        3,
        87,
        87,
        96,
        0,
        128,
        253,
        91,
        135,
        53,
        96,
        1,
        96,
        1,
        96,
        160,
        27,
        3,
        129,
        22,
        129,
        20,
        97,
        3,
        110,
        87,
        96,
        0,
        128,
        253,
        91,
        150,
        80,
        96,
        32,
        136,
        1,
        53,
        103,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        128,
        130,
        17,
        21,
        97,
        3,
        139,
        87,
        96,
        0,
        128,
        253,
        91,
        129,
        138,
        1,
        145,
        80,
        138,
        96,
        31,
        131,
        1,
        18,
        97,
        3,
        159,
        87,
        96,
        0,
        128,
        253,
        91,
        129,
        53,
        129,
        129,
        17,
        21,
        97,
        3,
        174,
        87,
        96,
        0,
        128,
        253,
        91,
        139,
        96,
        32,
        130,
        133,
        1,
        1,
        17,
        21,
        97,
        3,
        192,
        87,
        96,
        0,
        128,
        253,
        91,
        96,
        32,
        131,
        1,
        152,
        80,
        128,
        151,
        80,
        80,
        96,
        64,
        138,
        1,
        53,
        145,
        80,
        128,
        130,
        17,
        21,
        97,
        3,
        222,
        87,
        96,
        0,
        128,
        253,
        91,
        97,
        3,
        234,
        139,
        131,
        140,
        1,
        97,
        2,
        240,
        86,
        91,
        144,
        150,
        80,
        148,
        80,
        96,
        96,
        138,
        1,
        53,
        145,
        80,
        128,
        130,
        17,
        21,
        97,
        4,
        3,
        87,
        96,
        0,
        128,
        253,
        91,
        80,
        97,
        4,
        16,
        138,
        130,
        139,
        1,
        97,
        2,
        240,
        86,
        91,
        152,
        155,
        151,
        154,
        80,
        149,
        152,
        80,
        147,
        150,
        146,
        149,
        146,
        147,
        80,
        80,
        80,
        86,
        254,
        162,
        100,
        105,
        112,
        102,
        115,
        88,
        34,
        18,
        32,
        126,
        94,
        18,
        135,
        237,
        192,
        106,
        176,
        138,
        117,
        148,
        175,
        188,
        5,
        29,
        238,
        102,
        42,
        95,
        89,
        105,
        55,
        42,
        182,
        85,
        202,
        193,
        11,
        170,
        77,
        47,
        178,
        100,
        115,
        111,
        108,
        99,
        67,
        0,
        8,
        18,
        0,
        51,
    ];
    ///The bytecode of the contract.
    pub static INCLUSIONVERIFIER_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = &[
        96,
        128,
        96,
        64,
        82,
        52,
        128,
        21,
        97,
        0,
        16,
        87,
        96,
        0,
        128,
        253,
        91,
        80,
        96,
        4,
        54,
        16,
        97,
        0,
        43,
        87,
        96,
        0,
        53,
        96,
        224,
        28,
        128,
        99,
        71,
        246,
        181,
        170,
        20,
        97,
        0,
        48,
        87,
        91,
        96,
        0,
        128,
        253,
        91,
        97,
        0,
        67,
        97,
        0,
        62,
        54,
        96,
        4,
        97,
        3,
        60,
        86,
        91,
        97,
        0,
        87,
        86,
        91,
        96,
        64,
        81,
        144,
        21,
        21,
        129,
        82,
        96,
        32,
        1,
        96,
        64,
        81,
        128,
        145,
        3,
        144,
        243,
        91,
        96,
        0,
        97,
        1,
        53,
        86,
        91,
        96,
        32,
        130,
        1,
        53,
        131,
        129,
        16,
        131,
        53,
        133,
        129,
        16,
        132,
        22,
        145,
        144,
        145,
        22,
        145,
        133,
        96,
        3,
        129,
        128,
        133,
        128,
        9,
        133,
        9,
        8,
        134,
        130,
        131,
        9,
        20,
        131,
        22,
        146,
        80,
        80,
        80,
        147,
        146,
        80,
        80,
        80,
        86,
        91,
        129,
        96,
        192,
        82,
        130,
        96,
        224,
        82,
        96,
        0,
        96,
        64,
        96,
        128,
        128,
        96,
        128,
        96,
        6,
        90,
        250,
        144,
        145,
        22,
        147,
        146,
        80,
        80,
        80,
        86,
        91,
        129,
        96,
        192,
        82,
        96,
        0,
        96,
        64,
        96,
        128,
        96,
        96,
        96,
        128,
        96,
        7,
        90,
        250,
        144,
        145,
        22,
        146,
        145,
        80,
        80,
        86,
        91,
        129,
        96,
        0,
        82,
        130,
        96,
        32,
        82,
        97,
        2,
        128,
        81,
        96,
        64,
        82,
        97,
        2,
        160,
        81,
        96,
        96,
        82,
        97,
        2,
        192,
        81,
        96,
        128,
        82,
        97,
        2,
        224,
        81,
        96,
        160,
        82,
        131,
        96,
        192,
        82,
        132,
        96,
        224,
        82,
        97,
        3,
        0,
        81,
        97,
        1,
        0,
        82,
        97,
        3,
        32,
        81,
        97,
        1,
        32,
        82,
        97,
        3,
        64,
        81,
        97,
        1,
        64,
        82,
        97,
        3,
        96,
        81,
        97,
        1,
        96,
        82,
        96,
        0,
        96,
        32,
        96,
        0,
        97,
        1,
        128,
        96,
        0,
        96,
        8,
        90,
        250,
        96,
        0,
        81,
        146,
        22,
        144,
        145,
        22,
        149,
        148,
        80,
        80,
        80,
        80,
        80,
        86,
        91,
        127,
        48,
        100,
        78,
        114,
        225,
        49,
        160,
        41,
        184,
        80,
        69,
        182,
        129,
        129,
        88,
        93,
        151,
        129,
        106,
        145,
        104,
        113,
        202,
        141,
        60,
        32,
        140,
        22,
        216,
        124,
        253,
        71,
        127,
        48,
        100,
        78,
        114,
        225,
        49,
        160,
        41,
        184,
        80,
        69,
        182,
        129,
        129,
        88,
        93,
        40,
        51,
        232,
        72,
        121,
        185,
        112,
        145,
        67,
        225,
        245,
        147,
        240,
        0,
        0,
        1,
        96,
        1,
        96,
        192,
        97,
        1,
        96,
        97,
        2,
        64,
        141,
        60,
        96,
        132,
        53,
        96,
        128,
        129,
        6,
        96,
        0,
        20,
        130,
        22,
        145,
        80,
        129,
        97,
        1,
        155,
        87,
        96,
        0,
        128,
        253,
        91,
        96,
        32,
        129,
        96,
        132,
        1,
        1,
        128,
        53,
        96,
        4,
        129,
        20,
        132,
        22,
        147,
        80,
        131,
        97,
        1,
        182,
        87,
        96,
        0,
        128,
        253,
        91,
        96,
        32,
        130,
        1,
        53,
        97,
        3,
        0,
        82,
        96,
        64,
        130,
        1,
        53,
        97,
        3,
        32,
        82,
        96,
        96,
        130,
        1,
        53,
        97,
        3,
        64,
        82,
        96,
        128,
        130,
        1,
        53,
        97,
        3,
        96,
        82,
        96,
        32,
        128,
        130,
        2,
        131,
        1,
        1,
        145,
        80,
        80,
        128,
        53,
        96,
        32,
        129,
        2,
        131,
        4,
        96,
        4,
        20,
        132,
        22,
        147,
        80,
        131,
        97,
        1,
        254,
        87,
        96,
        0,
        128,
        253,
        91,
        96,
        0,
        91,
        129,
        129,
        16,
        21,
        97,
        2,
        226,
        87,
        97,
        2,
        64,
        81,
        96,
        128,
        82,
        97,
        2,
        96,
        81,
        96,
        160,
        82,
        96,
        32,
        129,
        129,
        2,
        132,
        1,
        1,
        53,
        134,
        3,
        96,
        192,
        129,
        144,
        82,
        96,
        64,
        130,
        2,
        97,
        2,
        53,
        130,
        136,
        97,
        0,
        174,
        86,
        91,
        135,
        22,
        150,
        80,
        134,
        97,
        2,
        67,
        87,
        96,
        0,
        128,
        253,
        91,
        128,
        96,
        2,
        135,
        4,
        96,
        164,
        1,
        1,
        145,
        80,
        97,
        2,
        89,
        137,
        131,
        137,
        97,
        0,
        94,
        86,
        91,
        150,
        80,
        134,
        97,
        2,
        101,
        87,
        96,
        0,
        128,
        253,
        91,
        96,
        32,
        130,
        1,
        53,
        145,
        53,
        97,
        2,
        119,
        131,
        130,
        138,
        97,
        0,
        143,
        86,
        91,
        151,
        80,
        80,
        134,
        97,
        2,
        132,
        87,
        96,
        0,
        128,
        253,
        91,
        96,
        128,
        81,
        97,
        2,
        0,
        82,
        96,
        160,
        81,
        97,
        2,
        32,
        82,
        96,
        164,
        1,
        144,
        80,
        97,
        2,
        162,
        136,
        130,
        136,
        97,
        0,
        94,
        86,
        91,
        149,
        80,
        133,
        97,
        2,
        174,
        87,
        96,
        0,
        128,
        253,
        91,
        128,
        53,
        96,
        32,
        130,
        1,
        53,
        145,
        80,
        97,
        2,
        202,
        130,
        130,
        97,
        2,
        32,
        81,
        97,
        2,
        0,
        81,
        139,
        97,
        0,
        201,
        86,
        91,
        135,
        22,
        150,
        80,
        80,
        80,
        132,
        97,
        2,
        218,
        87,
        96,
        0,
        128,
        253,
        91,
        96,
        1,
        1,
        97,
        2,
        1,
        86,
        91,
        80,
        80,
        80,
        80,
        128,
        96,
        0,
        82,
        96,
        32,
        96,
        0,
        243,
        91,
        96,
        0,
        128,
        131,
        96,
        31,
        132,
        1,
        18,
        97,
        3,
        2,
        87,
        96,
        0,
        128,
        253,
        91,
        80,
        129,
        53,
        103,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        129,
        17,
        21,
        97,
        3,
        26,
        87,
        96,
        0,
        128,
        253,
        91,
        96,
        32,
        131,
        1,
        145,
        80,
        131,
        96,
        32,
        130,
        96,
        5,
        27,
        133,
        1,
        1,
        17,
        21,
        97,
        3,
        53,
        87,
        96,
        0,
        128,
        253,
        91,
        146,
        80,
        146,
        144,
        80,
        86,
        91,
        96,
        0,
        128,
        96,
        0,
        128,
        96,
        0,
        128,
        96,
        0,
        96,
        128,
        136,
        138,
        3,
        18,
        21,
        97,
        3,
        87,
        87,
        96,
        0,
        128,
        253,
        91,
        135,
        53,
        96,
        1,
        96,
        1,
        96,
        160,
        27,
        3,
        129,
        22,
        129,
        20,
        97,
        3,
        110,
        87,
        96,
        0,
        128,
        253,
        91,
        150,
        80,
        96,
        32,
        136,
        1,
        53,
        103,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        128,
        130,
        17,
        21,
        97,
        3,
        139,
        87,
        96,
        0,
        128,
        253,
        91,
        129,
        138,
        1,
        145,
        80,
        138,
        96,
        31,
        131,
        1,
        18,
        97,
        3,
        159,
        87,
        96,
        0,
        128,
        253,
        91,
        129,
        53,
        129,
        129,
        17,
        21,
        97,
        3,
        174,
        87,
        96,
        0,
        128,
        253,
        91,
        139,
        96,
        32,
        130,
        133,
        1,
        1,
        17,
        21,
        97,
        3,
        192,
        87,
        96,
        0,
        128,
        253,
        91,
        96,
        32,
        131,
        1,
        152,
        80,
        128,
        151,
        80,
        80,
        96,
        64,
        138,
        1,
        53,
        145,
        80,
        128,
        130,
        17,
        21,
        97,
        3,
        222,
        87,
        96,
        0,
        128,
        253,
        91,
        97,
        3,
        234,
        139,
        131,
        140,
        1,
        97,
        2,
        240,
        86,
        91,
        144,
        150,
        80,
        148,
        80,
        96,
        96,
        138,
        1,
        53,
        145,
        80,
        128,
        130,
        17,
        21,
        97,
        4,
        3,
        87,
        96,
        0,
        128,
        253,
        91,
        80,
        97,
        4,
        16,
        138,
        130,
        139,
        1,
        97,
        2,
        240,
        86,
        91,
        152,
        155,
        151,
        154,
        80,
        149,
        152,
        80,
        147,
        150,
        146,
        149,
        146,
        147,
        80,
        80,
        80,
        86,
        254,
        162,
        100,
        105,
        112,
        102,
        115,
        88,
        34,
        18,
        32,
        126,
        94,
        18,
        135,
        237,
        192,
        106,
        176,
        138,
        117,
        148,
        175,
        188,
        5,
        29,
        238,
        102,
        42,
        95,
        89,
        105,
        55,
        42,
        182,
        85,
        202,
        193,
        11,
        170,
        77,
        47,
        178,
        100,
        115,
        111,
        108,
        99,
        67,
        0,
        8,
        18,
        0,
        51,
    ];
    ///The deployed bytecode of the contract.
    pub static INCLUSIONVERIFIER_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct InclusionVerifier<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for InclusionVerifier<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for InclusionVerifier<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for InclusionVerifier<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for InclusionVerifier<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(stringify!(InclusionVerifier)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> InclusionVerifier<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    INCLUSIONVERIFIER_ABI.clone(),
                    client,
                ),
            )
        }
        /// Constructs the general purpose `Deployer` instance based on the provided constructor arguments and sends it.
        /// Returns a new instance of a deployer that returns an instance of this contract after sending the transaction
        ///
        /// Notes:
        /// - If there are no constructor arguments, you should pass `()` as the argument.
        /// - The default poll duration is 7 seconds.
        /// - The default number of confirmations is 1 block.
        ///
        ///
        /// # Example
        ///
        /// Generate contract bindings with `abigen!` and deploy a new contract instance.
        ///
        /// *Note*: this requires a `bytecode` and `abi` object in the `greeter.json` artifact.
        ///
        /// ```ignore
        /// # async fn deploy<M: ethers::providers::Middleware>(client: ::std::sync::Arc<M>) {
        ///     abigen!(Greeter, "../greeter.json");
        ///
        ///    let greeter_contract = Greeter::deploy(client, "Hello world!".to_string()).unwrap().send().await.unwrap();
        ///    let msg = greeter_contract.greet().call().await.unwrap();
        /// # }
        /// ```
        pub fn deploy<T: ::ethers::core::abi::Tokenize>(
            client: ::std::sync::Arc<M>,
            constructor_args: T,
        ) -> ::core::result::Result<
            ::ethers::contract::builders::ContractDeployer<M, Self>,
            ::ethers::contract::ContractError<M>,
        > {
            let factory = ::ethers::contract::ContractFactory::new(
                INCLUSIONVERIFIER_ABI.clone(),
                INCLUSIONVERIFIER_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `verifyProof` (0x47f6b5aa) function
        pub fn verify_proof(
            &self,
            vk: ::ethers::core::types::Address,
            proofs: ::ethers::core::types::Bytes,
            challenges: ::std::vec::Vec<::ethers::core::types::U256>,
            values: ::std::vec::Vec<::ethers::core::types::U256>,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([71, 246, 181, 170], (vk, proofs, challenges, values))
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for InclusionVerifier<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `verifyProof` function with signature `verifyProof(address,bytes,uint256[],uint256[])` and selector `0x47f6b5aa`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "verifyProof",
        abi = "verifyProof(address,bytes,uint256[],uint256[])"
    )]
    pub struct VerifyProofCall {
        pub vk: ::ethers::core::types::Address,
        pub proofs: ::ethers::core::types::Bytes,
        pub challenges: ::std::vec::Vec<::ethers::core::types::U256>,
        pub values: ::std::vec::Vec<::ethers::core::types::U256>,
    }
    ///Container type for all return fields from the `verifyProof` function with signature `verifyProof(address,bytes,uint256[],uint256[])` and selector `0x47f6b5aa`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct VerifyProofReturn(pub bool);
}
