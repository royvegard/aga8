use super::detail::{Detail, NC_DETAIL};
use std::slice;

/// Return type
#[repr(C)]
pub struct Properties {
    /// Molar concentration [mol/l]
    pub d: f64,
    /// Molar mass [g/mol]
    pub mm: f64,
    /// Compressibility factor [-]
    pub z: f64,
    /// First derivative of pressure with respect
    /// to density at constant temperature [kPa/(mol/l)]
    pub dp_dd: f64,
    /// Second derivative of pressure with respect to
    /// temperature and density [kPa/(mol/l)/K] (currently not calculated)
    pub d2p_dd2: f64,
    /// First derivative of pressure with respect to
    /// temperature at constant density [kPa/K]
    pub dp_dt: f64,
    /// Internal energy [J/mol]
    pub u: f64,
    /// Enthalpy [J/mol]
    pub h: f64,
    /// Entropy [J/(mol-K)]
    pub s: f64,
    /// Isochoric heat capacity [J/(mol-K)]
    pub cv: f64,
    /// Isobaric heat capacity [J/(mol-K)]
    pub cp: f64,
    /// Speed of sound [m/s]
    pub w: f64,
    /// Gibbs energy [J/mol]
    pub g: f64,
    /// Joule-Thomson coefficient [K/kPa]
    pub jt: f64,
    /// Isentropic Exponent
    pub kappa: f64,
}

/// # Safety
/// composition must be an array of 21 elements.
#[no_mangle]
pub unsafe extern "C" fn aga8_2017(
    composition: *const f64,
    pressure: f64,
    temperature: f64,
) -> Properties {
    let array = {
        assert!(!composition.is_null());
        slice::from_raw_parts(composition, NC_DETAIL)
    };

    let mut aga8_test: Detail = Detail::new();
    aga8_test.setup();

    aga8_test.x[0..NC_DETAIL].clone_from_slice(array);

    aga8_test.t = temperature;
    aga8_test.p = pressure;
    aga8_test.density_detail();
    aga8_test.properties_detail();

    Properties {
        d: aga8_test.d, // Molar concentration [mol/l]
        mm: aga8_test.mm,
        z: aga8_test.z,
        dp_dd: aga8_test.dp_dd,
        d2p_dd2: aga8_test.d2p_dd2,
        dp_dt: aga8_test.dp_dt,
        u: aga8_test.u,
        h: aga8_test.h,
        s: aga8_test.s,
        cv: aga8_test.cv,
        cp: aga8_test.cp,
        w: aga8_test.w,
        g: aga8_test.g,
        jt: aga8_test.jt,
        kappa: aga8_test.kappa,
    }
}

#[no_mangle]
pub extern "C" fn aga8_new() -> *mut Detail {
    Box::into_raw(Box::new(Detail::new()))
}

/// # Safety
///
#[no_mangle]
pub unsafe extern "C" fn aga8_free(ptr: *mut Detail) {
    if ptr.is_null() {
        return;
    }
    Box::from_raw(ptr);
}

/// # Safety
///
#[no_mangle]
pub unsafe extern "C" fn aga8_setup(ptr: *mut Detail) {
    assert!(!ptr.is_null());
    let aga8 = &mut *ptr;
    aga8.setup();
}

/// # Safety
///
#[no_mangle]
pub unsafe extern "C" fn aga8_set_composition(ptr: *mut Detail, composition: *const f64) {
    assert!(!ptr.is_null());
    assert!(!composition.is_null());
    let aga8 = &mut *ptr;
    let array = slice::from_raw_parts(composition, NC_DETAIL);
    aga8.x[0..NC_DETAIL].clone_from_slice(array);
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
pub unsafe extern "C" fn aga8_calculate_pressure(ptr: *mut Detail) {
    assert!(!ptr.is_null());
    let aga8 = &mut *ptr;
    aga8.pressure_detail();
}

/// # Safety
///
#[no_mangle]
pub unsafe extern "C" fn aga8_calculate_density(ptr: *mut Detail) {
    assert!(!ptr.is_null());
    let aga8 = &mut *ptr;
    aga8.density_detail();
}

/// # Safety
///
#[no_mangle]
pub unsafe extern "C" fn aga8_calculate_properties(ptr: *mut Detail) {
    assert!(!ptr.is_null());
    let aga8 = &mut *ptr;
    aga8.properties_detail();
}
