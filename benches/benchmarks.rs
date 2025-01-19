#![allow(dead_code)]

use aga8::composition::Composition;
use aga8::detail::Detail;
use aga8::gerg2008::Gerg2008;
use criterion::{criterion_group, criterion_main, Criterion};

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

fn bench_detail_new(c: &mut Criterion) {
    c.bench_function("Detail_new", |b| {
        b.iter(|| {
            let mut _aga8_test: Detail = Detail::new();
        })
    });
}

fn bench_detail_density(c: &mut Criterion) {
    let mut aga8_test: Detail = Detail::new();
    aga8_test.x = [
        0.778_240, 0.020_000, 0.060_000, 0.080_000, 0.030_000, 0.001_500, 0.003_000, 0.000_500,
        0.001_650, 0.002_150, 0.000_880, 0.000_240, 0.000_150, 0.000_090, 0.004_000, 0.005_000,
        0.002_000, 0.000_100, 0.002_500, 0.007_000, 0.001_000,
    ];
    aga8_test.p = 50_000.0;
    aga8_test.t = 400.0;

    c.bench_function("Detail_density", |b| {
        b.iter(|| {
            aga8_test.density().unwrap();
        })
    });
}

fn bench_detail_properties(c: &mut Criterion) {
    let mut aga8_test: Detail = Detail::new();
    aga8_test.x = [
        0.778_240, 0.020_000, 0.060_000, 0.080_000, 0.030_000, 0.001_500, 0.003_000, 0.000_500,
        0.001_650, 0.002_150, 0.000_880, 0.000_240, 0.000_150, 0.000_090, 0.004_000, 0.005_000,
        0.002_000, 0.000_100, 0.002_500, 0.007_000, 0.001_000,
    ];
    aga8_test.p = 50_000.0;
    aga8_test.t = 400.0;
    aga8_test.density().unwrap();

    c.bench_function("Detail_properties", |b| {
        b.iter(|| {
            aga8_test.properties();
        })
    });
}

fn bench_gerg_density(c: &mut Criterion) {
    let mut gerg_test: Gerg2008 = Gerg2008::new();

    gerg_test.set_composition(&COMP_FULL).unwrap();
    gerg_test.molar_mass();
    gerg_test.t = 400.0;
    gerg_test.p = 50000.0;
    gerg_test.d = 6.36570;
    gerg_test.z = 0.0;
    c.bench_function("Gerg_density", |b| {
        b.iter(|| {
            gerg_test.density(0).unwrap();
        })
    });
}

fn bench_gerg_properties(c: &mut Criterion) {
    let mut gerg_test: Gerg2008 = Gerg2008::new();

    gerg_test.set_composition(&COMP_FULL).unwrap();
    gerg_test.molar_mass();
    gerg_test.t = 400.0;
    gerg_test.p = 50000.0;
    gerg_test.d = 6.36570;
    gerg_test.z = 0.0;
    gerg_test.density(0).unwrap();

    c.bench_function("Gerg_properties", |b| {
        b.iter(|| {
            gerg_test.properties();
        })
    });
}

criterion_group!(
    benches,
    bench_detail_new,
    bench_detail_density,
    bench_detail_properties,
    bench_gerg_density,
    bench_gerg_properties,
);
criterion_main!(benches);
