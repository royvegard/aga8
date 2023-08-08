//! Gas composition

/// A complete gas composition made up of gas components.
///
/// A gas composition contains 21 gas components named by the field names in the struct.
/// The unit for each component is *mole fraction*, so the sum of all components should be `1.0`.
/// If the initial sum of components is not `1.0`, you can use the normalize function to scale it to `1.0`.
///
/// # Example
/// ```
/// let air = aga8::composition::Composition {
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
    /// Methane CH<sub>4</sub>
    pub methane: f64,
    /// Nitrogen N
    pub nitrogen: f64,
    /// Carbon Dioxide CO<sub>2</sub>
    pub carbon_dioxide: f64,
    /// Ethane C<sub>2</sub>H<sub>6</sub>
    pub ethane: f64,
    /// Propane C<sub>3</sub>H<sub>8</sub>
    pub propane: f64,
    /// Isobutane C<sub>4</sub>H<sub>10</sub>
    pub isobutane: f64,
    /// Butane C<sub>4</sub>H<sub>10</sub>
    pub n_butane: f64,
    /// Isopentane C<sub>5</sub>H<sub>12</sub>
    pub isopentane: f64,
    /// Pentane C<sub>5</sub>H<sub>12</sub>
    pub n_pentane: f64,
    /// Isopentane C<sub>6</sub>H<sub>14</sub>
    pub hexane: f64,
    /// Heptane C<sub>7</sub>H<sub>16</sub>
    pub heptane: f64,
    /// Octane C<sub>8</sub>H<sub>18</sub>
    pub octane: f64,
    /// Nonane C<sub>9</sub>H<sub>20</sub>
    pub nonane: f64,
    /// Decane C<sub>10</sub>H<sub>22</sub>
    pub decane: f64,
    /// Hydrogen H
    pub hydrogen: f64,
    /// Oxygen O
    pub oxygen: f64,
    /// Carbon monoxide CO
    pub carbon_monoxide: f64,
    /// Water H<sub>2</sub>O
    pub water: f64,
    /// Hydrogen sulfide H<sub>2</sub>S
    pub hydrogen_sulfide: f64,
    /// Helium He
    pub helium: f64,
    /// Argon Ar
    pub argon: f64,
}

impl Composition {
    /// Compute the sum of all components.
    ///
    /// # Example
    /// ```
    /// let comp = aga8::composition::Composition {
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
    /// let mut comp = aga8::composition::Composition {
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
    pub fn normalize(&mut self) -> Result<(), CompositionError> {
        let sum = self.sum();
        if sum > 0.0 {
            let factor = 1.0 / sum;

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
        } else {
            return Err(CompositionError::Empty);
        }
        Ok(())
    }

    /// Checks that the composition is valid.
    ///
    /// # Example
    /// ```
    /// let mut comp = aga8::composition::Composition {
    ///     methane: 0.5,
    ///     ethane: 0.5,
    ///     ..Default::default()
    /// };
    ///
    /// assert_eq!(comp.check(), Ok(()));
    /// ```
    pub fn check(&self) -> Result<(), CompositionError> {
        if (self.sum() - 0.0).abs() < 1.0e-10 {
            return Err(CompositionError::Empty);
        }
        if (self.sum() - 1.0).abs() > 1.0e-4 {
            return Err(CompositionError::BadSum);
        }
        Ok(())
    }
}

/// Error conditions for composition
#[repr(C)]
#[derive(Debug, PartialEq, Eq)]
pub enum CompositionError {
    /// Composition is valid
    Ok = 0,
    /// Composition is empty, i.e. all component values are zero.
    Empty,
    /// The sum of the components is not 1.0000
    BadSum,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_is_ok() {
        let comp = Composition {
            methane: 0.5,
            ethane: 0.5,
            ..Default::default()
        };

        assert_eq!(comp.check(), Ok(()));
    }

    #[test]
    fn empty_is_error() {
        let comp = Composition {
            ..Default::default()
        };

        assert_eq!(comp.check(), Err(CompositionError::Empty));
    }

    #[test]
    fn too_big_is_error() {
        let comp = Composition {
            ethane: 0.5,
            methane: 0.6,
            propane: 0.4,
            ..Default::default()
        };

        assert_eq!(comp.check(), Err(CompositionError::BadSum));
    }

    #[test]
    fn normalized_is_1() {
        let mut comp = Composition {
            methane: 83.0,
            ethane: 17.0,
            ..Default::default()
        };

        comp.normalize().unwrap();

        assert_eq!(comp.sum(), 1.0);
    }

    #[test]
    fn normalize_empty_is_error() {
        let mut comp = Composition {
            ..Default::default()
        };

        assert_eq!(comp.normalize(), Err(CompositionError::Empty));
    }
}
