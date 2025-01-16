//! The GERG2008 equation of state.

use crate::composition::{Composition, CompositionError};
use crate::gerg2008const::*;
use crate::DensityError;

/// Implements the GERG2008 equation of state described in
/// AGA Report No. 8, Part 2, First Edition, April 2017.
///
/// The Gerg2008 struct has functions to set the gas composition, pressure and temperature.
/// It also has functions to calculate the density and other properties.
/// After the properties have been calculated they can be read by reading the public Fields of the Gerg2008 struct.
///
/// Before attempting to calculate density and properties, the composition, pressure and temperature should be set.
/// If these are not set to reasonable values, the [`density()`](Gerg2008::density) function will most likely return Err Result, and the property fileds will contain nonsensical values.
///
/// # Example
/// ```
/// use aga8::gerg2008::Gerg2008;
/// use aga8::composition::Composition;
///
/// let mut gerg_test: Gerg2008 = Gerg2008::new();
///
/// // Set the gas composition in mol fraction
/// // The sum of all the components must be 1.0
/// let comp = Composition {
///     methane: 0.778_24,
///     nitrogen: 0.02,
///     carbon_dioxide: 0.06,
///     ethane: 0.08,
///     propane: 0.03,
///     isobutane: 0.001_5,
///     n_butane: 0.003,
///     isopentane: 0.000_5,
///     n_pentane: 0.001_65,
///     hexane: 0.002_15,
///     heptane: 0.000_88,
///     octane: 0.000_24,
///     nonane: 0.000_15,
///     decane: 0.000_09,
///     hydrogen: 0.004,
///     oxygen: 0.005,
///     carbon_monoxide: 0.002,
///     water: 0.000_1,
///     hydrogen_sulfide: 0.002_5,
///     helium: 0.007,
///     argon: 0.001,
/// };
/// gerg_test.set_composition(&comp);
/// // Set pressure in kPa
/// gerg_test.p = 50_000.0;
/// // Set temperature in K
/// gerg_test.t = 400.0;
/// // Run density to calculate the density in mol/l
/// gerg_test.density(0);
/// // Run properties to calculate all of the
/// // output properties
/// gerg_test.properties();
///
/// // Molar density
/// assert!((12.798 - gerg_test.d).abs() < 1.0e-3);
/// // Compressibility factor
/// assert!((1.175 - gerg_test.z).abs() < 1.0e-3);
/// ```
#[derive(Default)]
pub struct Gerg2008 {
    /// Temperature in K
    pub t: f64,
    /// Pressure in kPa
    pub p: f64,
    /// Molar concentration in mol/l
    pub d: f64,
    /// Compressibility factor
    pub z: f64,
    /// Molar mass in g/mol
    pub mm: f64,
    /// First derivative of pressure with respect
    /// to density at constant temperature in kPa/(mol/l)
    pub dp_dd: f64,
    /// Second derivative of pressure with respect
    /// to density at constant temperature in kPa/(mol/l)^2
    pub d2p_dd2: f64,
    /// Second derivative of pressure with respect to
    /// temperature and density in kPa/(mol/l)/K (currently not calculated)
    pub d2p_dtd: f64,
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
    /// Composition in mole fractions
    pub x: [f64; NC_GERG + 1],

    drold: f64,
    trold: f64,
    told: f64,
    trold2: f64,
    xold: [f64; NC_GERG + 1],
    a: f64,
    a0: [f64; 3],
    ar: [[f64; 4]; 4],
    dpddsave: f64,
    taup: [[f64; MAXTRMP + 1]; MAXFLDS + 1],
    taupijk: [[f64; MAXTRMM + 1]; MAXFLDS + 1],
}

impl Gerg2008 {
    /// Creates a new instance of the Gerg2008 struct.
    pub fn new() -> Self {
        Default::default()
    }

    /// Checks and sets the composition.
    ///
    /// ## Error
    /// Returns error if the composition is invalid.
    pub fn set_composition(&mut self, comp: &Composition) -> Result<(), CompositionError> {
        comp.check()?;

        self.x[0] = 0.0;
        self.x[1] = comp.methane;
        self.x[2] = comp.nitrogen;
        self.x[3] = comp.carbon_dioxide;
        self.x[4] = comp.ethane;
        self.x[5] = comp.propane;
        self.x[6] = comp.isobutane;
        self.x[7] = comp.n_butane;
        self.x[8] = comp.isopentane;
        self.x[9] = comp.n_pentane;
        self.x[10] = comp.hexane;
        self.x[11] = comp.heptane;
        self.x[12] = comp.octane;
        self.x[13] = comp.nonane;
        self.x[14] = comp.decane;
        self.x[15] = comp.hydrogen;
        self.x[16] = comp.oxygen;
        self.x[17] = comp.carbon_monoxide;
        self.x[18] = comp.water;
        self.x[19] = comp.hydrogen_sulfide;
        self.x[20] = comp.helium;
        self.x[21] = comp.argon;

        Ok(())
    }

    /// Calculates the molar mass of the current composition.
    ///
    /// # Example
    /// ```
    /// use aga8::gerg2008::Gerg2008;
    /// use aga8::composition::Composition;
    ///
    /// let mut gerg_test: Gerg2008 = Gerg2008::new();
    /// let air = aga8::composition::Composition {
    ///     nitrogen: 0.78,
    ///     oxygen: 0.21,
    ///     argon: 0.009,
    ///     carbon_dioxide: 0.000_4,
    ///     water: 0.000_6,
    ///     ..Default::default()
    ///     };
    ///
    /// gerg_test.set_composition(&air);
    /// gerg_test.molar_mass();
    /// assert!((28.958 - gerg_test.mm).abs() < 1.0e-3);
    /// ```
    pub fn molar_mass(&mut self) {
        self.mm = 0.0;
        for (i, mmi_gerg) in MMI_GERG.iter().enumerate().skip(1) {
            self.mm += self.x[i] * mmi_gerg;
        }
    }

    /// Calculate pressure
    pub fn pressure(&mut self) -> f64 {
        self.alphar(0);
        self.z = 1.0 + self.ar[0][1];
        let p = self.d * RGERG * self.t * self.z;
        self.dpddsave = RGERG * self.t * (1.0 + 2.0 * self.ar[0][1] + self.ar[0][2]);
        p
    }

    /// Calculate density
    pub fn density(&mut self, iflag: i32) -> Result<(), DensityError> {
        let mut nfail: i32 = 0;
        let mut ifail: i32 = 0;
        const TOLR: f64 = 0.000_000_1;

        let (dcx, _tcx) = self.pseudocriticalpoint();

        if self.d > -EPSILON {
            self.d = self.p / RGERG / self.t;
            if iflag == 2 {
                self.d = dcx * 3.0;
            }
        } else {
            self.d = self.d.abs();
        }

        let plog = self.p.ln();
        let mut vlog = -self.d.ln();

        for it in 1..=50 {
            if !(-7.0..=100.0).contains(&vlog) || it == 20 || it == 30 || it == 40 || ifail == 1 {
                //Current state is bad or iteration is taking too long.  Restart with completely different initial state
                ifail = 0;
                if nfail > 2 {
                    // Iteration failed (above loop did not find a solution or checks made below indicate possible 2-phase state)
                    //herr = "Calculation failed to converge in GERG method, ideal gas density returned.";
                    self.d = self.p / RGERG / self.t;
                    return Err(DensityError::IterationFail);
                }
                nfail += 1;
                if nfail == 1 {
                    self.d = dcx * 3.0; // If vapor phase search fails, look for root in liquid region
                } else if nfail == 2 {
                    self.d = dcx * 2.5; // If liquid phase search fails, look for root between liquid and critical regions
                } else if nfail == 3 {
                    self.d = dcx * 2.0; // If search fails, look for root in critical region
                }
                vlog = -self.d.ln();
            }
            self.d = (-vlog).exp();
            let p2 = self.pressure();
            if self.dpddsave < EPSILON || p2 < EPSILON {
                // Current state is 2-phase, try locating a different state that is single phase
                let mut vinc = if self.d > dcx { -0.1 } else { 0.1 };
                if it > 5 {
                    vinc /= 2.0;
                }
                if it > 10 && it < 20 {
                    vinc /= 5.0;
                }
                vlog += vinc;
            } else {
                // Find the next density with a first order Newton's type iterative scheme, with
                // log(P) as the known variable and log(v) as the unknown property.
                // See AGA 8 publication for further information.
                let dpdlv = -self.d * self.dpddsave; // d(p)/d[log(v)]
                let vdiff = (p2.ln() - plog) * p2 / dpdlv;
                vlog += -vdiff;
                if vdiff.abs() < TOLR {
                    // Check to see if state is possibly 2-phase, and if so restart
                    if self.dpddsave < 0.0 {
                        ifail = 1;
                    } else {
                        self.d = (-vlog).exp();

                        // If requested, check to see if point is possibly 2-phase
                        if iflag > 0
                            && ((self.properties() <= 0.0
                                || self.dp_dd <= 0.0
                                || self.d2p_dtd <= 0.0)
                                || (self.cv <= 0.0 || self.cp <= 0.0 || self.w <= 0.0))
                        {
                            // Iteration failed (above loop did find a solution or checks made below indicate possible 2-phase state)
                            //herr = "Calculation failed to converge in GERG method, ideal gas density returned.";
                            self.d = self.p / RGERG / self.t;
                            return Err(DensityError::IterationFail);
                        }
                        return Ok(()); // Iteration converged
                    }
                }
            }
        }
        // Iteration failed (above loop did not find a solution or checks made below indicate possible 2-phase state)
        //herr = "Calculation failed to converge in GERG method, ideal gas density returned.";
        self.d = self.p / RGERG / self.t;
        Err(DensityError::IterationFail)
    }

    /// Calculate properties
    pub fn properties(&mut self) -> f64 {
        self.molar_mass();
        self.alpha0();
        self.alphar(1);

        let rt = RGERG * self.t;
        self.z = 1.0 + self.ar[0][1];
        let p = self.d * rt * self.z;
        self.dp_dd = rt * (1.0 + 2.0 * self.ar[0][1] + self.ar[0][2]);
        self.dp_dt = self.d * RGERG * (1.0 + self.ar[0][1] - self.ar[1][1]);
        self.d2p_dtd = RGERG
            * (1.0 + 2.0 * self.ar[0][1] + self.ar[0][2] - 2.0 * self.ar[1][1] - self.ar[1][2]);
        self.a = rt * self.a0[0] + self.ar[0][0];
        self.g = rt * (1.0 + self.ar[0][1] + self.a0[0] + self.ar[0][0]);
        self.u = rt * (self.a0[1] + self.ar[1][0]);
        self.h = rt * (1.0 + self.ar[0][1] + self.a0[1] + self.ar[1][0]);
        self.s = RGERG * (self.a0[1] + self.ar[1][0] - self.a0[0] - self.ar[0][0]);
        self.cv = -RGERG * (self.a0[2] + self.ar[2][0]);
        if self.d > EPSILON {
            self.cp = self.cv + self.t * (self.dp_dt / self.d) * (self.dp_dt / self.d) / self.dp_dd;
            self.d2p_dd2 =
                rt * (2.0 * self.ar[0][1] + 4.0 * self.ar[0][2] + self.ar[0][3]) / self.d;
            self.jt = (self.t / self.d * self.dp_dt / self.dp_dd - 1.0) / self.cp / self.d;
        //  '=(dB/dT*T-B)/Cp for an ideal gas, but dB/dT is not known
        } else {
            self.cp = self.cv + RGERG;
            self.d2p_dd2 = 0.0;
            self.jt = 1E+20;
        }
        self.w = 1000.0 * self.cp / self.cv * self.dp_dd / self.mm;
        if self.w < 0.0 {
            self.w = 0.0;
        }
        self.w = self.w.sqrt();
        self.kappa = self.w.powi(2) * self.mm / (rt * 1000.0 * self.z);
        p
    }

    fn reducingparameters(&mut self) -> (f64, f64) {
        let mut dr: f64 = 0.0;
        let mut tr: f64 = 0.0;
        let mut vr: f64 = 0.0;
        let mut xij: f64;
        let mut f: f64;
        let mut icheck: i32 = 0;

        // Check to see if a component fraction has changed.  If x is the same as the previous call, then exit.
        for i in 1..=NC_GERG {
            if (self.x[i] - self.xold[i]).abs() > 0.000_000_1 {
                icheck = 1;
            }
            self.xold[i] = self.x[i];
        }
        if icheck == 0 {
            return (self.drold, self.trold);
        }
        self.told = 0.0;
        self.trold2 = 0.0;

        // Calculate reducing variables for T and D
        for i in 1..=NC_GERG {
            if self.x[i] > EPSILON {
                f = 1.0;
                for j in i..=NC_GERG {
                    if self.x[j] > EPSILON {
                        xij = f * (self.x[i] * self.x[j]) * (self.x[i] + self.x[j]);
                        vr += xij * GVIJ[i][j] / (BVIJ[i][j] * self.x[i] + self.x[j]);
                        tr += xij * GTIJ[i][j] / (BTIJ[i][j] * self.x[i] + self.x[j]);
                        f = 2.0;
                    }
                }
            }
        }
        if vr > EPSILON {
            dr = 1.0 / vr;
        }
        self.drold = dr;
        self.trold = tr;
        (dr, tr)
    }

    fn alpha0(&mut self) {
        let mut loghyp: f64;
        let mut th0t: f64;
        let mut logxd: f64;
        let mut sumhyp0: f64;
        let mut sumhyp1: f64;
        let mut sumhyp2: f64;
        let mut em: f64;
        let mut ep: f64;
        let mut hcn: f64;
        let mut hsn: f64;

        self.a0[0] = 0.0;
        self.a0[1] = 0.0;
        self.a0[2] = 0.0;
        let logd = if self.d > EPSILON {
            self.d.ln()
        } else {
            EPSILON.ln()
        };
        let logt = self.t.ln();
        for (i, th0i) in TH0I.iter().enumerate().skip(1) {
            if self.x[i] > EPSILON {
                logxd = logd + self.x[i].ln();
                sumhyp0 = 0.0;
                sumhyp1 = 0.0;
                sumhyp2 = 0.0;
                for (j, th0ij) in th0i.iter().enumerate().take(8).skip(4) {
                    if th0ij > &EPSILON {
                        th0t = th0ij / self.t;
                        ep = th0t.exp();
                        em = 1.0 / ep;
                        hsn = (ep - em) / 2.0;
                        hcn = (ep + em) / 2.0;
                        if j == 4 || j == 6 {
                            loghyp = hsn.abs().ln();
                            sumhyp0 += N0I[i][j] * loghyp;
                            sumhyp1 += N0I[i][j] * th0t * hcn / hsn;
                            sumhyp2 += N0I[i][j] * (th0t / hsn) * (th0t / hsn);
                        } else {
                            loghyp = hcn.abs().ln();
                            sumhyp0 -= N0I[i][j] * loghyp;
                            sumhyp1 -= N0I[i][j] * th0t * hsn / hcn;
                            sumhyp2 += N0I[i][j] * (th0t / hcn) * (th0t / hcn);
                        }
                    }
                }
                self.a0[0] += self.x[i]
                    * (logxd + N0I[i][1] + N0I[i][2] / self.t - N0I[i][3] * logt + sumhyp0);
                self.a0[1] += self.x[i] * (N0I[i][3] + N0I[i][2] / self.t + sumhyp1);
                self.a0[2] += -self.x[i] * (N0I[i][3] + sumhyp2);
            }
        }
    }

    fn alphar(&mut self, itau: i32) {
        let mut ex: f64;
        let mut ex2: f64;
        let mut ex3: f64;
        let mut cij0: f64;
        let mut eij0: f64;
        let mut ndt: f64;
        let mut ndtd: f64;
        let mut ndtt: f64;
        let mut xijf: f64;
        let mut delp: [f64; 7 + 1] = [0.0; 7 + 1];
        let mut expd: [f64; 7 + 1] = [0.0; 7 + 1];

        for i in 0..=3 {
            for j in 0..=3 {
                self.ar[i][j] = 0.0;
            }
        }

        //Set up del, tau, log(tau), and the first 7 calculations for del^i
        let (dr, tr) = self.reducingparameters();
        let del = self.d / dr;
        let tau = tr / self.t;
        let lntau = tau.ln();
        delp[1] = del;
        expd[1] = (-delp[1]).exp();
        for i in 2..8 {
            delp[i] = delp[i - 1] * del;
            expd[i] = (-delp[i]).exp();
        }

        // If temperature has changed, calculate temperature dependent parts
        if (self.t - self.told).abs() > 0.000_000_1 || (tr - self.trold2).abs() > 0.000_000_1 {
            self.tterms(lntau);
        }
        self.told = self.t;
        self.trold2 = tr;

        // Calculate pure fluid contributions
        for i in 1..=NC_GERG {
            if self.x[i] > EPSILON {
                for k in 1..=KPOL[i] {
                    ndt = self.x[i] * delp[DOIK[i][k]] * self.taup[i][k];
                    ndtd = ndt * DOIK[i][k] as f64;
                    self.ar[0][1] += ndtd;
                    self.ar[0][2] += ndtd * (DOIK[i][k] as f64 - 1.0);
                    if itau > 0 {
                        ndtt = ndt * TOIK[i][k];
                        self.ar[0][0] += ndt;
                        self.ar[1][0] += ndtt;
                        self.ar[2][0] += ndtt * (TOIK[i][k] - 1.0);
                        self.ar[1][1] += ndtt * DOIK[i][k] as f64;
                        self.ar[1][2] += ndtt * DOIK[i][k] as f64 * (DOIK[i][k] as f64 - 1.0);
                        self.ar[0][3] +=
                            ndtd * (DOIK[i][k] as f64 - 1.0) * (DOIK[i][k] as f64 - 2.0);
                    }
                }
                for k in 1 + KPOL[i]..=KPOL[i] + KEXP[i] {
                    ndt = self.x[i] * delp[DOIK[i][k]] * self.taup[i][k] * expd[COIK[i][k]];
                    ex = COIK[i][k] as f64 * delp[COIK[i][k]];
                    ex2 = DOIK[i][k] as f64 - ex;
                    ex3 = ex2 * (ex2 - 1.0);
                    self.ar[0][1] += ndt * ex2;
                    self.ar[0][2] += ndt * (ex3 - COIK[i][k] as f64 * ex);
                    if itau > 0 {
                        ndtt = ndt * TOIK[i][k];
                        self.ar[0][0] += ndt;
                        self.ar[1][0] += ndtt;
                        self.ar[2][0] += ndtt * (TOIK[i][k] - 1.0);
                        self.ar[1][1] += ndtt * ex2;
                        self.ar[1][2] += ndtt * (ex3 - COIK[i][k] as f64 * ex);
                        self.ar[0][3] += ndt
                            * (ex3 * (ex2 - 2.0)
                                - ex * (3.0 * ex2 - 3.0 + COIK[i][k] as f64) * COIK[i][k] as f64);
                    }
                }
            }
        }

        // Calculate mixture contributions
        for i in 1..NC_GERG {
            if self.x[i] > EPSILON {
                for j in i + 1..=NC_GERG {
                    if self.x[j] > EPSILON {
                        let mn = MNUMB[i][j];
                        if mn > 0 {
                            xijf = self.x[i] * self.x[j] * FIJ[i][j];
                            for k in 1..=KPOLIJ[mn] {
                                ndt = xijf * delp[DIJK[mn][k]] * self.taupijk[mn][k];
                                ndtd = ndt * DIJK[mn][k] as f64;
                                self.ar[0][1] += ndtd;
                                self.ar[0][2] += ndtd * (DIJK[mn][k] as f64 - 1.0);
                                if itau > 0 {
                                    ndtt = ndt * TIJK[mn][k];
                                    self.ar[0][0] += ndt;
                                    self.ar[1][0] += ndtt;
                                    self.ar[2][0] += ndtt * (TIJK[mn][k] - 1.0);
                                    self.ar[1][1] += ndtt * DIJK[mn][k] as f64;
                                    self.ar[1][2] +=
                                        ndtt * DIJK[mn][k] as f64 * (DIJK[mn][k] as f64 - 1.0);
                                    self.ar[0][3] += ndtd
                                        * (DIJK[mn][k] as f64 - 1.0)
                                        * (DIJK[mn][k] as f64 - 2.0);
                                }
                            }
                            for k in 1 + KPOLIJ[mn]..=KPOLIJ[mn] + KEXPIJ[mn] {
                                cij0 = CIJK[mn][k] * delp[2];
                                eij0 = EIJK[mn][k] * del;
                                ndt = xijf
                                    * NIJK[mn][k]
                                    * delp[DIJK[mn][k]]
                                    * (cij0 + eij0 + GIJK[mn][k] + TIJK[mn][k] * lntau).exp();
                                ex = DIJK[mn][k] as f64 + 2.0 * cij0 + eij0;
                                ex2 = ex * ex - DIJK[mn][k] as f64 + 2.0 * cij0;
                                self.ar[0][1] += ndt * ex;
                                self.ar[0][2] += ndt * ex2;
                                if itau > 0 {
                                    ndtt = ndt * TIJK[mn][k];
                                    self.ar[0][0] += ndt;
                                    self.ar[1][0] += ndtt;
                                    self.ar[2][0] += ndtt * (TIJK[mn][k] - 1.0);
                                    self.ar[1][1] += ndtt * ex;
                                    self.ar[1][2] += ndtt * ex2;
                                    self.ar[0][3] += ndt
                                        * (ex * (ex2 - 2.0 * (DIJK[mn][k] as f64 - 2.0 * cij0))
                                            + 2.0 * DIJK[mn][k] as f64);
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    fn tterms(&mut self, lntau: f64) {
        let i: usize = 5;
        let mut taup0: [f64; 12 + 1] = [0.0; 12 + 1];

        //i = 5;  // Use propane to get exponents for short form of EOS
        for (k, taup) in taup0.iter_mut().enumerate().skip(1) {
            *taup = (TOIK[i][k] * lntau).exp();
        }
        for i in 1..=NC_GERG {
            if self.x[i] > EPSILON {
                if i > 4 && i != 15 && i != 18 && i != 20 {
                    for (k, taup) in taup0.iter().enumerate().skip(1) {
                        self.taup[i][k] = NOIK[i][k] * taup;
                    }
                } else {
                    for k in 1..=KPOL[i] + KEXP[i] {
                        self.taup[i][k] = NOIK[i][k] * (TOIK[i][k] * lntau).exp();
                    }
                }
            }
        }

        for (i, mnumbi) in MNUMB.iter().enumerate().skip(1) {
            if self.x[i] > EPSILON {
                for (j, mnumbij) in mnumbi.iter().enumerate().skip(i + 1) {
                    if self.x[j] > EPSILON {
                        let mn = *mnumbij;
                        if mn > 0 {
                            for k in 1..=KPOLIJ[mn] {
                                self.taupijk[mn][k] = NIJK[mn][k] * (TIJK[mn][k] * lntau).exp();
                            }
                        }
                    }
                }
            }
        }
    }

    fn pseudocriticalpoint(&self) -> (f64, f64) {
        let mut dcx = 0.0;
        let mut tcx = 0.0;
        let mut vcx: f64 = 0.0;

        for i in 1..=NC_GERG {
            tcx += self.x[i] * TC[i];
            vcx += self.x[i] / DC[i];
        }
        if vcx > EPSILON {
            dcx = 1.0 / vcx;
        }
        (dcx, tcx)
    }
}
