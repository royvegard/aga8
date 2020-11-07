#[test]
fn detail_demo_example() {
    let mut aga_test = aga8::Detail::new();

    aga_test.setup();

    aga_test.x = [
        0.77824, 0.02, 0.06, 0.08, 0.03, 0.0015, 0.003, 0.0005, 0.00165, 0.00215, 0.00088,
        0.00024, 0.00015, 0.00009, 0.004, 0.005, 0.002, 0.0001, 0.0025, 0.007, 0.001,
    ];

    aga_test.molar_mass_detail();

    aga_test.t = 400.0;
    aga_test.p = 50000.0;
    aga_test.d = 6.36570;
    aga_test.z = 0.0;

    aga_test.density_detail();
    aga_test.properties_detail();

    assert!(f64::abs(aga_test.d - 12.807_924_036_488_01) < 1.0e-10);
    assert!(f64::abs(aga_test.mm - 20.543_330_51) < 1.0e-10);
    assert!(f64::abs(aga_test.p - 50_000.0) < 1.0e-10);
    assert!(f64::abs(aga_test.z - 1.173_801_364_147_326) < 1.0e-10);
    assert!(f64::abs(aga_test.dp_dd - 6_971.387_690_924_090) < 1.0e-10);
    assert!(f64::abs(aga_test.d2p_dd2 - 1_118.803_636_639_520) < 1.0e-10);
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