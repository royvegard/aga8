use aga8::composition::Composition;
use aga8::gerg2008::Gerg2008;

const COMP_FULL: Composition = Composition {
    methane: 0.778_24,
    nitrogen: 0.02,
    carbon_dioxide: 0.06,
    ethane: 0.08,
    propane: 0.03,
    isobutane: 0.001_5,
    n_butane: 0.003,
    isopentane: 0.000_5,
    n_pentane: 0.001_65,
    hexane: 0.002_15,
    heptane: 0.000_88,
    octane: 0.000_24,
    nonane: 0.000_15,
    decane: 0.000_09,
    hydrogen: 0.004,
    oxygen: 0.005,
    carbon_monoxide: 0.002,
    water: 0.000_1,
    hydrogen_sulfide: 0.002_5,
    helium: 0.007,
    argon: 0.001,
};

const COMP_PARTIAL: Composition = Composition {
    methane: 0.965,
    nitrogen: 0.003,
    carbon_dioxide: 0.006,
    ethane: 0.018,
    propane: 0.004_5,
    isobutane: 0.001,
    n_butane: 0.001,
    isopentane: 0.000_5,
    n_pentane: 0.000_3,
    hexane: 0.000_7,
    heptane: 0.0,
    octane: 0.0,
    nonane: 0.0,
    decane: 0.0,
    hydrogen: 0.0,
    oxygen: 0.0,
    carbon_monoxide: 0.0,
    water: 0.0,
    hydrogen_sulfide: 0.0,
    helium: 0.0,
    argon: 0.0,
};

#[test]
fn gerg_demo_example() {
    let mut gerg_test: Gerg2008 = Gerg2008::new();

    gerg_test.set_composition(&COMP_FULL).unwrap();

    gerg_test.molar_mass();

    gerg_test.t = 400.0;
    gerg_test.p = 50000.0;
    gerg_test.d = 6.36570;
    gerg_test.z = 0.0;

    gerg_test.density(0).unwrap();
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

#[test]
fn gerg_test_01() {
    let mut gerg_test: Gerg2008 = Gerg2008::new();

    gerg_test.set_composition(&COMP_PARTIAL).unwrap();

    gerg_test.t = 18.0 + 273.15;
    gerg_test.p = 14601.325;

    gerg_test.density(0).unwrap();
    gerg_test.properties();

    println!("{}", gerg_test.d);
    println!("{}", gerg_test.mm);
    println!("{}", gerg_test.d * gerg_test.mm);

    assert!(f64::abs(gerg_test.d - 7.730_483_295_277_388) < 1.0e-10);
    assert!(f64::abs(gerg_test.mm - 16.803_030_286) < 1.0e-10);
}

#[test]
fn gerg_calculate_pressure() {
    let mut gerg_test: Gerg2008 = Gerg2008::new();

    gerg_test.set_composition(&COMP_FULL).unwrap();

    gerg_test.t = 18.0 + 273.15;
    gerg_test.d = 7.558_334;

    let p = gerg_test.pressure();

    println!("{}", gerg_test.d);
    println!("{}", p);

    assert!(f64::abs(p - 13_050.037_472_144) < 1.0e-10);
}

#[cfg(feature = "extern")]
#[test]
fn gerg_api_test_02() {
    use aga8::DensityError;
    use aga8::{composition::CompositionError, ffi::gerg2008::*};

    let temperature = 400.0;
    let pressure = 50_000.0;

    unsafe {
        let g_test = gerg_new();
        let mut err: CompositionError = CompositionError::Ok;
        let mut dens_err: DensityError = DensityError::Ok;
        gerg_set_composition(g_test, &COMP_FULL, &mut err);
        if err != CompositionError::Ok {
            panic!("Invalid composition: {:?}", err);
        }
        gerg_set_temperature(g_test, temperature);
        gerg_set_pressure(g_test, pressure);
        gerg_calculate_density(g_test, 0, &mut dens_err);
        if dens_err != DensityError::Ok {
            panic!("Density calculation failed: {:?}", dens_err);
        }
        gerg_calculate_properties(g_test);

        let results = gerg_get_properties(g_test);
        assert!(f64::abs(results.d - 12.798_286_260_820_62) < 1.0e-10);

        gerg_free(g_test);
    }
}

#[cfg(feature = "extern")]
#[test]
fn gerg_api_calculate_molar_mass() {
    use aga8::{composition::CompositionError, ffi::gerg2008::*};

    unsafe {
        let g_test = gerg_new();
        let mut err: CompositionError = CompositionError::Ok;
        gerg_set_composition(g_test, &COMP_FULL, &mut err);
        if err != CompositionError::Ok {
            panic!("Invalid composition: {:?}", err);
        }

        assert!(f64::abs(gerg_calculate_molar_mass(g_test) - 20.542_744_501_6) < 1.0e-10);

        gerg_free(g_test);
    }
}

#[test]
#[should_panic]
fn gerg_zero_composition() {
    let mut gerg_test = Gerg2008::new();

    let comp = Composition {
        methane: 0.0,
        ..Default::default()
    };

    gerg_test.set_composition(&comp).unwrap();
}
