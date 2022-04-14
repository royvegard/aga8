/*!
Provides methods to calculate thermodynamic properties inlcuding compressibility factors and densities of natural gases.
It includes the AGA8 DETAIL and the GERG2008 equations of state described in AGA Report No. 8, Part 1, Third Edition, April 2017.

This crate is a Rust port of NIST's [AGA8 code](https://github.com/usnistgov/AGA8).

# Quick Start
To use the AGA8 DETAIL and GERG equiations of state you typically create a struct with `new()`.
Then you set the gas composition `x`, the pressure `p` and the temperature `t`.
Lastly you call the `density()` and `properties()` functions to calculate the molar density and the rest of the properies.

All of the calculation results are public fields in the struct that was created with `new()`.

```
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

# Crate features
* **extern** - Builds external ffi functions. These functions can be used by other programming languages.
*/

pub mod detail;
pub mod gerg2008;

/// A complete gas composition made up of gas components.
///
/// A gas composition contains 21 gas components named by the field names in the struct.
/// The unit for each component is *mole fraction*, so the sum of all components should be `1.0`.
///
/// # Example
/// ```
/// let air = aga8::Composition {
///     nitrogen: 0.78,
///     oxygen: 0.21,
///     argon: 0.009,
///     carbon_dioxide: 0.000_4,
///     water: 0.000_6,
///     ..Default::default()
///     };
///
/// assert!((air.sum() - 1.0).abs() < 1.0e-10);
/// ```
#[repr(C)]
#[derive(Default)]
pub struct Composition {
    pub methane: f64,
    pub nitrogen: f64,
    pub carbon_dioxide: f64,
    pub ethane: f64,
    pub propane: f64,
    pub isobutane: f64,
    pub n_butane: f64,
    pub isopentane: f64,
    pub n_pentane: f64,
    pub hexane: f64,
    pub heptane: f64,
    pub octane: f64,
    pub nonane: f64,
    pub decane: f64,
    pub hydrogen: f64,
    pub oxygen: f64,
    pub carbon_monoxide: f64,
    pub water: f64,
    pub hydrogen_sulfide: f64,
    pub helium: f64,
    pub argon: f64,
}

impl Composition {
    /// Compute the sum of all components.
    ///
    /// # Example
    /// ```
    /// let comp = aga8::Composition {
    ///     methane: 50.0,
    ///     ethane: 25.0,
    ///     carbon_dioxide: 25.0,
    ///     ..Default::default()
    /// };
    ///
    /// assert!((comp.sum() - 100.0).abs() < 1.0e-10);
    /// ```
    pub fn sum(&self) -> f64 {
        self.methane
            + self.nitrogen
            + self.carbon_dioxide
            + self.ethane
            + self.propane
            + self.isobutane
            + self.n_butane
            + self.isopentane
            + self.n_pentane
            + self.hexane
            + self.heptane
            + self.octane
            + self.nonane
            + self.decane
            + self.hydrogen
            + self.oxygen
            + self.carbon_monoxide
            + self.water
            + self.hydrogen_sulfide
            + self.helium
            + self.argon
    }

    /// Normalizes the composition sum to 1.0.
    ///
    /// # Example
    /// ```
    /// let mut comp = aga8::Composition {
    ///     methane: 50.0,
    ///     ethane: 50.0,
    ///     ..Default::default()
    /// };
    ///
    /// comp.normalize();
    ///
    /// assert!((comp.ethane - 0.5).abs() < 1.0e-10);
    /// assert!((comp.methane - 0.5).abs() < 1.0e-10);
    /// ```
    pub fn normalize(&mut self) {
        let factor = 1.0 / self.sum();

        self.methane *= factor;
        self.nitrogen *= factor;
        self.carbon_dioxide *= factor;
        self.ethane *= factor;
        self.propane *= factor;
        self.isobutane *= factor;
        self.n_butane *= factor;
        self.isopentane *= factor;
        self.n_pentane *= factor;
        self.hexane *= factor;
        self.heptane *= factor;
        self.octane *= factor;
        self.nonane *= factor;
        self.decane *= factor;
        self.hydrogen *= factor;
        self.oxygen *= factor;
        self.carbon_monoxide *= factor;
        self.water *= factor;
        self.hydrogen_sulfide *= factor;
        self.helium *= factor;
        self.argon *= factor;
    }

    /// Checks that the composition is valid.
    ///
    /// # Example
    /// ```
    /// let mut comp = aga8::Composition {
    ///     methane: 0.5,
    ///     ethane: 0.5,
    ///     ..Default::default()
    /// };
    ///
    /// assert!(comp.check());
    /// ```
    pub fn check(&self) -> bool {
        if (self.sum() - 0.0).abs() < 1.0e-10 {
            return false;
        }
        if (self.sum() - 1.0).abs() > 1.0e-10 {
            return false;
        }
        true
    }
}

#[cfg(feature = "extern")]
pub mod ffi;
