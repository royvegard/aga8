use aga8::Composition;

#[test]
fn detail_demo_example() {
    let mut aga_test = aga8::detail::Detail::new();

    aga_test.setup();

    let comp = Composition {
        methane: 0.77824,
        nitrogen: 0.02,
        carbon_dioxide: 0.06,
        ethane: 0.08,
        propane: 0.03,
        isobutane: 0.0015,
        n_butane: 0.003,
        isopentane: 0.0005,
        n_pentane: 0.00165,
        hexane: 0.00215,
        heptane: 0.00088,
        octane: 0.00024,
        nonane: 0.00015,
        decane: 0.00009,
        hydrogen: 0.004,
        oxygen: 0.005,
        carbon_monoxide: 0.002,
        water: 0.0001,
        hydrogen_sulfide: 0.0025,
        helium: 0.007,
        argon: 0.001,
    };

    aga_test.set_composition(&comp);

    aga_test.molar_mass();

    aga_test.t = 400.0;
    aga_test.p = 50000.0;
    aga_test.d = 6.36570;
    aga_test.z = 0.0;

    aga_test.density();
    aga_test.properties();

    assert!(f64::abs(aga_test.d - 12.807_924_036_488_01) < 1.0e-10);
    assert!(f64::abs(aga_test.mm - 20.543_330_51) < 1.0e-10);
    assert!(f64::abs(aga_test.p - 50_000.0) < 1.0e-10);
    assert!(f64::abs(aga_test.z - 1.173_801_364_147_326) < 1.0e-10);
    assert!(f64::abs(aga_test.dp_dd - 6_971.387_690_924_09) < 1.0e-10);
    assert!(f64::abs(aga_test.d2p_dd2 - 1_118.803_636_639_52) < 1.0e-10);
    assert!(f64::abs(aga_test.dp_dt - 235.664_149_306_821_2) < 1.0e-10);
    assert!(f64::abs(aga_test.u - -2_739.134_175_817_231) < 1.0e-10);
    assert!(f64::abs(aga_test.h - 1_164.699_096_269_404) < 1.0e-10);
    assert!(f64::abs(aga_test.s - -38.548_826_846_771_11) < 1.0e-10);
    assert!(f64::abs(aga_test.cv - 39.120_761_544_303_32) < 1.0e-10);
    assert!(f64::abs(aga_test.cp - 58.546_176_723_806_67) < 1.0e-10);
    assert!(f64::abs(aga_test.w - 712.639_368_405_790_3) < 1.0e-10);
    assert!(f64::abs(aga_test.g - 16_584.229_834_977_85) < 1.0e-10);
    assert!(f64::abs(aga_test.jt - 7.432_969_304_794_577E-5) < 1.0e-10);
    assert!(f64::abs(aga_test.kappa - 2.672_509_225_184_606) < 1.0e-10);
}

#[cfg(feature = "extern")]
#[test]
fn detail_api_test_01() {
    let composition: [f64; 21] = [
        0.965, 0.003, 0.006, 0.018, 0.0045, 0.001, 0.001, 0.0005, 0.0003, 0.0007, 0.0, 0.0, 0.0,
        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
    ];

    let temperature = 18.0 + 273.15;
    let pressure = 14601.325;

    unsafe {
        let result = aga8::ffi::detail::aga8_2017(&composition[0], pressure, temperature);
        assert!(f64::abs(result.d - 7.731_358_744_220) < 1.0e-10);
    }
}
