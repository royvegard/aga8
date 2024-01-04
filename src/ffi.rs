//! # Foreign Function Interface
//! The foreign function interface modules have functions that can be used by
//! other programming languages.

/// Return type
#[repr(C)]
pub struct Properties {
    /// Molar concentration in mol/l
    pub d: f64,
    /// Molar mass in g/mol
    pub mm: f64,
    /// Compressibility factor
    pub z: f64,
    /// First derivative of pressure with respect
    /// to density at constant temperature in kPa/(mol/l)
    pub dp_dd: f64,
    /// Second derivative of pressure with respect to
    /// temperature and density in kPa/(mol/l)/K (currently not calculated)
    pub d2p_dd2: f64,
    /// First derivative of pressure with respect to
    /// temperature at constant density in kPa/K
    pub dp_dt: f64,
    /// Internal energy in J/mol
    pub u: f64,
    /// Enthalpy in J/mol
    pub h: f64,
    /// Entropy in J/(mol-K)
    pub s: f64,
    /// Isochoric heat capacity in J/(mol-K)
    pub cv: f64,
    /// Isobaric heat capacity in J/(mol-K)
    pub cp: f64,
    /// Speed of sound in m/s
    pub w: f64,
    /// Gibbs energy in J/mol
    pub g: f64,
    /// Joule-Thomson coefficient in K/kPa
    pub jt: f64,
    /// Isentropic Exponent
    pub kappa: f64,
}

/// # AGA8 detail functions
pub mod detail {
    use super::*;
    use crate::composition::{Composition, CompositionError};
    use crate::detail::Detail;
    use crate::DensityError;

    /// Alocates memory for an aga8 Detail struct.
    /// This handle is used when calling the rest of the aga8 detail functions.
    #[no_mangle]
    pub extern "C" fn aga8_new() -> *mut Detail {
        Box::into_raw(Box::new(Detail::new()))
    }

    /// # Frees the memory used by ptr.
    /// # Safety
    ///
    #[no_mangle]
    pub unsafe extern "C" fn aga8_free(ptr: *mut Detail) {
        if ptr.is_null() {
            return;
        }
        drop(Box::from_raw(ptr));
    }

    /// # Safety
    ///
    #[no_mangle]
    pub unsafe extern "C" fn aga8_set_composition(
        ptr: *mut Detail,
        composition: &Composition,
        _err: &mut CompositionError,
    ) {
        assert!(!ptr.is_null());
        let aga8 = &mut *ptr;

        match aga8.set_composition(composition) {
            Ok(_) => *_err = CompositionError::Ok,
            Err(e) => *_err = e,
        }
    }

    /// # Safety
    ///
    #[no_mangle]
    pub unsafe extern "C" fn aga8_set_pressure(ptr: *mut Detail, pressure: f64) {
        assert!(!ptr.is_null());
        let aga8 = &mut *ptr;
        aga8.p = pressure;
    }

    /// # Safety
    ///
    #[no_mangle]
    pub unsafe extern "C" fn aga8_get_pressure(ptr: *mut Detail) -> f64 {
        assert!(!ptr.is_null());
        let aga8 = &mut *ptr;
        aga8.p
    }

    /// # Safety
    ///
    #[no_mangle]
    pub unsafe extern "C" fn aga8_set_temperature(ptr: *mut Detail, temperature: f64) {
        assert!(!ptr.is_null());
        let aga8 = &mut *ptr;
        aga8.t = temperature;
    }

    /// # Safety
    ///
    #[no_mangle]
    pub unsafe extern "C" fn aga8_get_temperature(ptr: *mut Detail) -> f64 {
        assert!(!ptr.is_null());
        let aga8 = &mut *ptr;
        aga8.t
    }

    /// # Safety
    ///
    #[no_mangle]
    pub unsafe extern "C" fn aga8_set_density(ptr: *mut Detail, density: f64) {
        assert!(!ptr.is_null());
        let aga8 = &mut *ptr;
        aga8.d = density;
    }

    /// # Safety
    ///
    #[no_mangle]
    pub unsafe extern "C" fn aga8_get_density(ptr: *mut Detail) -> f64 {
        assert!(!ptr.is_null());
        let aga8 = &mut *ptr;
        aga8.d
    }

    /// # Safety
    ///
    #[no_mangle]
    pub unsafe extern "C" fn aga8_get_properties(ptr: *const Detail) -> Properties {
        assert!(!ptr.is_null());
        let aga8 = &*ptr;
        Properties {
            d: aga8.d, // Molar concentration [mol/l]
            mm: aga8.mm,
            z: aga8.z,
            dp_dd: aga8.dp_dd,
            d2p_dd2: aga8.d2p_dd2,
            dp_dt: aga8.dp_dt,
            u: aga8.u,
            h: aga8.h,
            s: aga8.s,
            cv: aga8.cv,
            cp: aga8.cp,
            w: aga8.w,
            g: aga8.g,
            jt: aga8.jt,
            kappa: aga8.kappa,
        }
    }

    /// # Safety
    ///
    #[no_mangle]
    pub unsafe extern "C" fn aga8_calculate_pressure(ptr: *mut Detail) -> f64 {
        assert!(!ptr.is_null());
        let aga8 = &mut *ptr;
        aga8.pressure()
    }

    /// # Safety
    ///
    #[no_mangle]
    pub unsafe extern "C" fn aga8_calculate_molar_mass(ptr: *mut Detail) -> f64 {
        assert!(!ptr.is_null());
        let aga8 = &mut *ptr;
        aga8.molar_mass();
        aga8.mm
    }

    /// # Safety
    ///
    #[no_mangle]
    pub unsafe extern "C" fn aga8_calculate_density(ptr: *mut Detail, _err: &mut DensityError) {
        assert!(!ptr.is_null());
        let aga8 = &mut *ptr;
        match aga8.density() {
            Ok(_) => *_err = DensityError::Ok,
            Err(e) => *_err = e,
        }
    }

    /// # Safety
    ///
    #[no_mangle]
    pub unsafe extern "C" fn aga8_calculate_properties(ptr: *mut Detail) {
        assert!(!ptr.is_null());
        let aga8 = &mut *ptr;
        aga8.properties();
    }
}

/// # Gerg2008 functions
pub mod gerg2008 {
    use super::*;
    use crate::composition::{Composition, CompositionError};
    use crate::gerg2008::Gerg2008;
    use crate::DensityError;

    /// Create a Gerg2008 type
    ///
    #[no_mangle]
    pub extern "C" fn gerg_new() -> *mut Gerg2008 {
        Box::into_raw(Box::new(Gerg2008::new()))
    }

    /// Frees the memory used by the Gerg2008 type
    ///
    /// # Safety
    ///
    #[no_mangle]
    pub unsafe extern "C" fn gerg_free(ptr: *mut Gerg2008) {
        if ptr.is_null() {
            return;
        }
        drop(Box::from_raw(ptr));
    }

    /// # Safety
    ///
    #[no_mangle]
    pub unsafe extern "C" fn gerg_set_composition(
        ptr: *mut Gerg2008,
        composition: &Composition,
        _err: &mut CompositionError,
    ) {
        assert!(!ptr.is_null());
        let gerg = &mut *ptr;

        match gerg.set_composition(composition) {
            Ok(_) => *_err = CompositionError::Ok,
            Err(e) => *_err = e,
        }
    }

    /// # Safety
    ///
    #[no_mangle]
    pub unsafe extern "C" fn gerg_set_pressure(ptr: *mut Gerg2008, pressure: f64) {
        assert!(!ptr.is_null());
        let gerg = &mut *ptr;
        gerg.p = pressure;
    }

    /// # Safety
    ///
    #[no_mangle]
    pub unsafe extern "C" fn gerg_get_pressure(ptr: *mut Gerg2008) -> f64 {
        assert!(!ptr.is_null());
        let gerg = &mut *ptr;
        gerg.p
    }

    /// # Safety
    ///
    #[no_mangle]
    pub unsafe extern "C" fn gerg_set_temperature(ptr: *mut Gerg2008, temperature: f64) {
        assert!(!ptr.is_null());
        let gerg = &mut *ptr;
        gerg.t = temperature;
    }

    /// # Safety
    ///
    #[no_mangle]
    pub unsafe extern "C" fn gerg_get_temperature(ptr: *mut Gerg2008) -> f64 {
        assert!(!ptr.is_null());
        let gerg = &mut *ptr;
        gerg.t
    }

    /// # Safety
    ///
    #[no_mangle]
    pub unsafe extern "C" fn gerg_set_density(ptr: *mut Gerg2008, density: f64) {
        assert!(!ptr.is_null());
        let gerg = &mut *ptr;
        gerg.d = density;
    }

    /// # Safety
    ///
    #[no_mangle]
    pub unsafe extern "C" fn gerg_get_density(ptr: *mut Gerg2008) -> f64 {
        assert!(!ptr.is_null());
        let gerg = &mut *ptr;
        gerg.d
    }

    /// # Safety
    ///
    #[no_mangle]
    pub unsafe extern "C" fn gerg_get_properties(ptr: *const Gerg2008) -> Properties {
        assert!(!ptr.is_null());
        let gerg = &*ptr;
        Properties {
            d: gerg.d, // Molar concentration [mol/l]
            mm: gerg.mm,
            z: gerg.z,
            dp_dd: gerg.dp_dd,
            d2p_dd2: gerg.d2p_dd2,
            dp_dt: gerg.dp_dt,
            u: gerg.u,
            h: gerg.h,
            s: gerg.s,
            cv: gerg.cv,
            cp: gerg.cp,
            w: gerg.w,
            g: gerg.g,
            jt: gerg.jt,
            kappa: gerg.kappa,
        }
    }

    /// # Safety
    ///
    #[no_mangle]
    pub unsafe extern "C" fn gerg_calculate_pressure(ptr: *mut Gerg2008) -> f64 {
        assert!(!ptr.is_null());
        let gerg = &mut *ptr;
        gerg.pressure()
    }

    /// # Safety
    ///
    #[no_mangle]
    pub unsafe extern "C" fn gerg_calculate_molar_mass(ptr: *mut Gerg2008) -> f64 {
        assert!(!ptr.is_null());
        let gerg = &mut *ptr;
        gerg.molar_mass();
        gerg.mm
    }

    /// Calculates the density
    ///
    /// # Safety
    ///
    #[no_mangle]
    pub unsafe extern "C" fn gerg_calculate_density(ptr: *mut Gerg2008, _err: &mut DensityError) {
        assert!(!ptr.is_null());
        let gerg = &mut *ptr;
        match gerg.density(0) {
            Ok(_) => *_err = DensityError::Ok,
            Err(e) => *_err = e,
        }
    }

    /// # Safety
    ///
    #[no_mangle]
    pub unsafe extern "C" fn gerg_calculate_properties(ptr: *mut Gerg2008) {
        assert!(!ptr.is_null());
        let gerg = &mut *ptr;
        gerg.properties();
    }
}
