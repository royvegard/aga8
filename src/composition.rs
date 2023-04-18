//! Gas composition

/// A complete gas composition made up of gas components.
///
/// A gas composition contains 21 gas components named by the field names in the struct.
/// The unit for each component is *mole fraction*, so the sum of all components should be `1.0`.
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
        if (self.sum() - 1.0).abs() > 1.0e-5 {
            return Err(CompositionError::BadSum);
        }
        Ok(())
    }
}

#[repr(C)]
#[derive(Debug, PartialEq, Eq)]
pub enum CompositionError {
    Ok = 0,
    Empty,
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

        comp.normalize();

        assert_eq!(comp.sum(), 1.0);
    }
}
