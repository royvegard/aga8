use aga8::composition::Composition;
use aga8::{gerg2008::Gerg2008, DensityError};
use rand::prelude::*;
use std::fs::OpenOptions;
use std::io::{prelude::*, BufWriter};

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

    gerg_test.set_composition(&COMP_PARTIAL).unwrap();

    gerg_test.molar_mass();

    let file = OpenOptions::new()
        .append(true)
        .create_new(true)
        .open("data.csv")
        .unwrap();
    let mut writer = BufWriter::new(file);

    let file_2 = OpenOptions::new()
        .append(true)
        .create(true)
        .open("data_2.csv")
        .unwrap();
    let mut writer_2 = BufWriter::new(file_2);

    writeln!(writer, "# Temperature, Pressure, MolarConsentration").unwrap();

    let mut rng = thread_rng();
    let iterations = 250_000;
    for i in 0..iterations {
        if (i % 10_000) == 0 {
            writer.flush().unwrap();
            writer_2.flush().unwrap();
            println!("{}% flush", i * 100 / iterations);
        }

        gerg_test.p = rng.gen_range(1.0..20_000.0);
        gerg_test.t = rng.gen_range(90.0..200.0);
        let e = gerg_test.density(0);
        match e {
            Ok(_) | Err(DensityError::Ok) => {
                writeln!(writer, "{}, {}, {}", gerg_test.t, gerg_test.p, gerg_test.d).unwrap()
            }
            Err(DensityError::IterationFail) => {
                writeln!(
                    writer,
                    "# Iteration error: t={} p={}",
                    gerg_test.t, gerg_test.p
                )
                .unwrap();
                writeln!(writer_2, "{}, {}, 0.0", gerg_test.t, gerg_test.p).unwrap();
            }
            Err(DensityError::PressureTooLow) => {
                writeln!(
                    writer,
                    "# Pressure too low: t={} p={}",
                    gerg_test.t, gerg_test.p
                )
                .unwrap();
                writeln!(writer_2, "{}, {}, 0.0", gerg_test.t, gerg_test.p).unwrap();
            }
        }
    }
}
