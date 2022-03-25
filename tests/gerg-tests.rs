use aga8::gerg2008::Gerg2008;

#[test]
fn gerg_demo_example() {
    let mut gerg_test: Gerg2008 = Gerg2008::new();

    gerg_test.setup();

    gerg_test.x = [
        0.0, 0.77824, 0.02, 0.06, 0.08, 0.03, 0.0015, 0.003, 0.0005, 0.00165, 0.00215, 0.00088,
        0.00024, 0.00015, 0.00009, 0.004, 0.005, 0.002, 0.0001, 0.0025, 0.007, 0.001,
    ];

    gerg_test.molar_mass();

    gerg_test.t = 400.0;
    gerg_test.p = 50000.0;
    gerg_test.d = 6.36570;
    gerg_test.z = 0.0;

    gerg_test.density(0);
    gerg_test.properties();

    assert!(f64::abs(gerg_test.d - 12.798_286_260_820_62) < 1.0e-10);
    assert!(f64::abs(gerg_test.mm - 20.542_744_501_6) < 1.0e-10);
    assert!(f64::abs(gerg_test.p - 50_000.0) < 1.0e-10);
    assert!(f64::abs(gerg_test.z - 1.174_690_666_383_717) < 1.0e-10);
    assert!(f64::abs(gerg_test.dp_dd - 7_000.694_030_193_327) < 1.0e-10);
    assert!(f64::abs(gerg_test.d2p_dd2 - 1_129.526_655_214_841) < 1.0e-10);
    assert!(f64::abs(gerg_test.dp_dt - 235.983_229_259_309_6) < 1.0e-10);
    assert!(f64::abs(gerg_test.u - -2_746.492_901_212_53) < 1.0e-10);
    assert!(f64::abs(gerg_test.h - 1_160.280_160_510_973) < 1.0e-10);
    assert!(f64::abs(gerg_test.s - -38.575_903_924_090_89) < 1.0e-10);
    assert!(f64::abs(gerg_test.cv - 39.029_482_181_563_72) < 1.0e-10);
    assert!(f64::abs(gerg_test.cp - 58.455_220_510_003_66) < 1.0e-10);
    assert!(f64::abs(gerg_test.w - 714.424_884_059_602_4) < 1.0e-10);
    assert!(f64::abs(gerg_test.g - 16_590.641_730_147_33) < 1.0e-10);
    assert!(f64::abs(gerg_test.jt - 7.155_629_581_480_913E-5) < 1.0e-10);
    assert!(f64::abs(gerg_test.kappa - 2.683_820_255_058_032) < 1.0e-10);
}

#[cfg(feature = "extern")]
#[test]
fn gerg_api_call() {
    use aga8::ffi::gerg2008::*;

    let composition: [f64; 21] = [
        0.77824, 0.02, 0.06, 0.08, 0.03, 0.0015, 0.003, 0.0005, 0.00165, 0.00215, 0.00088, 0.00024,
        0.00015, 0.00009, 0.004, 0.005, 0.002, 0.0001, 0.0025, 0.007, 0.001,
    ];

    let temperature = 400.0;
    let pressure = 50000.0;

    unsafe {
        let result = gerg_2008(&composition[0], pressure, temperature);
        assert!(f64::abs(result.d - 12.798_286_260_820_62) < 1.0e-10);
    }
}

#[test]
fn gerg_test_01() {
    let mut gerg_test: Gerg2008 = Gerg2008::new();

    gerg_test.setup();

    gerg_test.x = [
        0.0, 0.965, 0.003, 0.006, 0.018, 0.0045, 0.001, 0.001, 0.0005, 0.0003, 0.0007, 0.0, 0.0,
        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
    ];

    gerg_test.t = 18.0 + 273.15;
    gerg_test.p = 14601.325;

    gerg_test.density(0);
    gerg_test.properties();

    println!("{}", gerg_test.d);
    println!("{}", gerg_test.mm);
    println!("{}", gerg_test.d * gerg_test.mm);

    assert!(f64::abs(gerg_test.d - 7.730_483_295_277_388) < 1.0e-10);
    assert!(f64::abs(gerg_test.mm - 16.803_030_286) < 1.0e-10);
}

#[cfg(feature = "extern")]
#[test]
fn gerg_api_test_01() {
    use aga8::ffi::gerg2008::*;

    let composition: [f64; 21] = [
        0.965, 0.003, 0.006, 0.018, 0.0045, 0.001, 0.001, 0.0005, 0.0003, 0.0007, 0.0, 0.0, 0.0,
        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
    ];

    let temperature = 18.0 + 273.15;
    let pressure = 14601.325;

    unsafe {
        let result = gerg_2008(&composition[0], pressure, temperature);

        assert!(f64::abs(result.d - 7.730_483_295_277_388) < 1.0e-10);
    }
}
