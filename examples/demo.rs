use aga8::detail::Detail;

fn main() {
    let mut aga8_test: Detail = Detail::new();

    // Set the gas composition in mol fraction
    // The sum of all the components must be 1.0
    let composition = aga8::composition::Composition {
        methane: 0.778_240,
        nitrogen: 0.020_000,
        carbon_dioxide: 0.060_000,
        ethane: 0.080_000,
        propane: 0.030_000,
        isobutane: 0.001_500,
        n_butane: 0.003_000,
        isopentane: 0.000_500,
        n_pentane: 0.001_650,
        hexane: 0.002_150,
        heptane: 0.000_880,
        octane: 0.000_240,
        nonane: 0.000_150,
        decane: 0.000_090,
        hydrogen: 0.004_000,
        oxygen: 0.005_000,
        carbon_monoxide: 0.002_000,
        water: 0.000_100,
        hydrogen_sulfide: 0.002_500,
        helium: 0.007_000,
        argon: 0.001_000,
    };

    aga8_test.set_composition(&composition).unwrap();

    // Set pressure in kPA
    aga8_test.p = 50_000.0;
    // Set temperature in K
    aga8_test.t = 400.0;
    // Run density_detail to calculate the density in mol/l
    aga8_test.density();
    // Run properties_detail to calculate all of the
    // output properties mentioned below
    aga8_test.properties();

    println!("Inputs-----");
    println!(
        "Temperature [K]:                    400.0000000000000 != {}",
        aga8_test.t
    );
    println!(
        "Pressure [kPa]:                     50000.00000000000 != {}",
        aga8_test.p
    );
    println!("Outputs-----");
    println!(
        "Molar mass [g/mol]:                 20.54333051000000 != {}",
        aga8_test.mm
    );
    println!(
        "Molar density [mol/l]:              12.80792403648801 != {}",
        aga8_test.d
    );
    println!(
        "Pressure [kPa]:                     50000.00000000004 != {}",
        aga8_test.p
    );
    println!(
        "Compressibility factor:             1.173801364147326 != {}",
        aga8_test.z
    );
    println!(
        "d(P)/d(rho) [kPa/(mol/l)]:          6971.387690924090 != {}",
        aga8_test.dp_dd
    );
    println!(
        "d^2(P)/d(rho)^2 [kPa/(mol/l)^2]:    1118.803636639520 != {}",
        aga8_test.d2p_dd2
    );
    println!(
        "d(P)/d(T) [kPa/K]:                  235.6641493068212 != {}",
        aga8_test.dp_dt
    );
    println!(
        "Energy [J/mol]:                    -2739.134175817231 != {}",
        aga8_test.u
    );
    println!(
        "Enthalpy [J/mol]:                   1164.699096269404 != {}",
        aga8_test.h
    );
    println!(
        "Entropy [J/mol-K]:                 -38.54882684677111 != {}",
        aga8_test.s
    );
    println!(
        "Isochoric heat capacity [J/mol-K]:  39.12076154430332 != {}",
        aga8_test.cv
    );
    println!(
        "Isobaric heat capacity [J/mol-K]:   58.54617672380667 != {}",
        aga8_test.cp
    );
    println!(
        "Speed of sound [m/s]:               712.6393684057903 != {}",
        aga8_test.w
    );
    println!(
        "Gibbs energy [J/mol]:               16584.22983497785 != {}",
        aga8_test.g
    );
    println!(
        "Joule-Thomson coefficient [K/kPa]:  7.432969304794577E-05 != {}",
        aga8_test.jt
    );
    println!(
        "Isentropic exponent:                2.672509225184606 != {}",
        aga8_test.kappa
    );
}
