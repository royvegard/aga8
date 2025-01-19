#![allow(dead_code)]

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

fn main() {
    let mut gerg_test: Gerg2008 = Gerg2008::new();

    gerg_test.set_composition(&COMP_FULL).unwrap();

    gerg_test.molar_mass();

    gerg_test.t = 400.0;
    gerg_test.p = 50000.0;
    gerg_test.d = 6.36570;
    gerg_test.z = 0.0;

    println!("Inputs-----");
    println!(
        "Temperature [K]:                    400.0000000000000 != {}",
        gerg_test.t
    );
    println!(
        "Pressure [kPa]:                     50000.00000000000 != {}",
        gerg_test.p
    );

    gerg_test.density(0).unwrap();

    gerg_test.properties();

    println!("Outputs-----");
    println!(
        "Molar mass [g/mol]:                 20.54274450160000 != {}",
        gerg_test.mm
    );
    println!(
        "Molar density [mol/l]:              12.79828626082062 != {}",
        gerg_test.d
    );
    println!(
        "Pressure [kPa]:                     50000.00000000001 != {}",
        gerg_test.p
    );
    println!(
        "Compressibility factor:             1.174690666383717 != {}",
        gerg_test.z
    );
    println!(
        "d(P)/d(rho) [kPa/(mol/l)]:          7000.694030193327 != {}",
        gerg_test.dp_dd
    );
    println!(
        "d^2(P)/d(rho)^2 [kPa/(mol/l)^2]:    1130.481239114938 != {}",
        gerg_test.d2p_dd2
    );
    println!(
        "d(P)/d(T) [kPa/K]:                  235.9832292593096 != {}",
        gerg_test.dp_dt
    );
    println!(
        "Energy [J/mol]:                     -2746.492901212530 != {}",
        gerg_test.u
    );
    println!(
        "Enthalpy [J/mol]:                   1160.280160510973 != {}",
        gerg_test.h
    );
    println!(
        "Entropy [J/mol-K]:                  -38.57590392409089 != {}",
        gerg_test.s
    );
    println!(
        "Isochoric heat capacity [J/mol-K]:  39.02948218156372 != {}",
        gerg_test.cv
    );
    println!(
        "Isobaric heat capacity [J/mol-K]:   58.45522051000366 != {}",
        gerg_test.cp
    );
    println!(
        "Speed of sound [m/s]:               714.4248840596024 != {}",
        gerg_test.w
    );
    println!(
        "Gibbs energy [J/mol]:               16590.64173014733 != {}",
        gerg_test.g
    );
    println!(
        "Joule-Thomson coefficient [K/kPa]:  7.155629581480913E-05 != {}",
        gerg_test.jt
    );
    println!(
        "Isentropic exponent:                2.683820255058032 != {}",
        gerg_test.kappa
    );
}
