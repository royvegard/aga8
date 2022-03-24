/*!
# AGA8 equations of state
Provides methods to calculate thermodynamic properties inlcuding
compressibility factors and densities of natural gases.

# Crate features
* **extern** -
  Builds the external ffi functions.
*/

pub mod detail;
pub mod gerg2008;

#[cfg(feature = "extern")]
pub mod ffi;
