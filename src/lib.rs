/*!
Provides methods to calculate thermodynamic properties inlcuding
compressibility factors and densities of natural gases. It includes
the AGA8 DETAIL and the GERG2008 equations of state described in
AGA Report No. 8, Part 1, Third Edition, April 2017.

This crate is a Rust port of NIST's
[AGA8 code](https://github.com/usnistgov/AGA8).

# Quick Start
To use the AGA8 DETAIL and GERG equiations of state you typically
start by calling the `new()` and `setup()` functions to initialize
the calculations. Then you set the gas composition `x`, the pressure `p`
and the temperature `t`. Lastly you call the `density()` and
`properties()` functions to calculate the molar density and
the rest of the properies.

All of the calculation results are public fields in the struct that was
created with `new()`.

Note that the gas composition is an array of excactly 21 components that
must be in the order shown in the example.

```
use aga8::detail::Detail;

let mut aga8_test: Detail = Detail::new();

// Run seup() first to set up internal values
aga8_test.setup();
// Set the gas composition in mol fraction
// The sum of all the components must be 1.0
aga8_test.x = [
    0.778_240, // Methane
    0.020_000, // Nitrogen
    0.060_000, // Carbon dioxide
    0.080_000, // Ethane
    0.030_000, // Propane
    0.001_500, // Isobutane
    0.003_000, // n-Butane
    0.000_500, // Isopentane
    0.001_650, // n-Pentane
    0.002_150, // Hexane
    0.000_880, // Heptane
    0.000_240, // Octane
    0.000_150, // Nonane
    0.000_090, // Decane
    0.004_000, // Hydrogen
    0.005_000, // Oxygen
    0.002_000, // Carbon monoxide
    0.000_100, // Water
    0.002_500, // Hydrogen sulfide
    0.007_000, // Helium
    0.001_000, // Argon
];
// Set pressure in kPa
aga8_test.p = 50_000.0;
// Set temperature in K
aga8_test.t = 400.0;
// Run density_detail to calculate the density in mol/l
aga8_test.density_detail();
// Run properties_detail to calculate all of the
// output properties
aga8_test.properties_detail();

// Molar density
assert!((12.807 - aga8_test.d).abs() < 1.0e-3);
// Compressibility factor
assert!((1.173 - aga8_test.z).abs() < 1.0e-3);
```

# Crate features
* **extern** -
  Builds external ffi functions. These functions can be used by
  other programming languages.
*/

pub mod detail;
pub mod gerg2008;

#[cfg(feature = "extern")]
pub mod ffi;
