# Info
Provides methods to calculate thermodynamic properties inlcuding compressibility factors and densities of natural gases.
It includes the AGA8 DETAIL and the GERG2008 equations of state described in AGA Report No. 8, Part 1, Third Edition, April 2017.

This crate is a Rust port of NIST's
[AGA8 code](https://github.com/usnistgov/AGA8).

[![Rust](https://github.com/royvegard/aga8/actions/workflows/rust.yml/badge.svg)](https://github.com/royvegard/aga8/actions/workflows/rust.yml)

# Quick Start
To use the AGA8 DETAIL and GERG equiations of state you typically create a struct with `new()`.
Then you set the gas composition `x`, the pressure `p` and the temperature `t`.
Lastly you call the `density()` and `properties()` functions to calculate the molar density and the rest of the properies.

All of the calculation results are public fields in the struct that was created with `new()`.

Note that the gas composition is an array of excactly 21 components that must be in the order shown in the example.

```Rust
use aga8::detail::Detail;
use aga8::Composition;

let mut aga8_test: Detail = Detail::new();

// Set the gas composition in mol fraction
// The sum of all the components must be 1.0
let comp = Composition {
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
aga8_test.set_composition(&comp);
// Set pressure in kPa
aga8_test.p = 50_000.0;
// Set temperature in K
aga8_test.t = 400.0;
// Run density to calculate the density in mol/l
aga8_test.density();
// Run properties to calculate all of the
// output properties
aga8_test.properties();

// Molar density
assert!((12.807 - aga8_test.d).abs() < 1.0e-3);
// Compressibility factor
assert!((1.173 - aga8_test.z).abs() < 1.0e-3);
```
