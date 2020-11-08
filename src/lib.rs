//
// A rust binding for the GSL library by Guillaume Gomez (guillaume1.gomez@gmail.com)
//

//! A __Rust__ binding for the [GSL library][] (the GNU Scientific Library).
//!
//! ## Installation
//!
//! This binding requires the [GSL library] library to be installed.
//!
//! This crate works with Cargo and is on [crates.io].  Just add the
//! following to your `Cargo.toml` file:
//!
//! ```toml
//! [dependencies]
//! GSL = "*"
//! ```
//!
//! Add the following line to your source code:
//!
//! ```rust
//! extern crate rgsl;
//! ```
//!
//! ## Documentation
//!
//! You can access the __rgsl__ documentation locally, just build it:
//!
//! ```Shell
//! > cargo doc --open
//! ```
//!
//! Then open this file with an internet browser:
//! `file:///{rgsl_location}/target/doc/rgsl/index.html`
//!
//! You can also access the latest build of the documentation via the internet
//! [here](https://docs.rs/crate/GSL/).
//!
//! ## License
//! __rust-GSL__ is a wrapper for __GSL__, therefore inherits the
//! [GPL license](http://www.gnu.org/copyleft/gpl.html).
//!
//! [crates.io]: https://crates.io/crates/GSL
//! [GSL library]: http://www.gnu.org/software/gsl/
//!
//! Here is the list of all modules :

#![crate_name = "rgsl"]
#![crate_type = "rlib"]
#![crate_type = "dylib"]
#![cfg_attr(feature = "dox", feature(doc_cfg))]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::excessive_precision)]
#![allow(clippy::many_single_char_names)]
#![allow(clippy::should_implement_trait)]
#![allow(clippy::type_complexity)]

extern crate gsl_sys as sys;
extern crate paste;

pub use types::*;

pub use elementary::Elementary;
pub use pow::Pow;
pub use trigonometric::Trigonometric;
pub use types::rng;
pub use utilities::IOStream;

// enums part
pub use self::enums::*;

mod enums;
mod macros;
mod utilities;

#[doc(hidden)]
pub mod ffi;

pub mod randist;
pub mod types;

pub mod airy;
pub mod bessel;
pub mod blas;
pub mod cblas;
pub mod clausen;
pub mod coulomb;
pub mod coupling_coefficients;
pub mod dawson;
pub mod debye;
pub mod dilogarithm;
pub mod eigen;
pub mod elementary;
pub mod elementary_operations;
pub mod elliptic;
pub mod error;
pub mod exponential;
pub mod exponential_integrals;
pub mod fermi_dirac;
pub mod fft;
#[cfg(feature = "v2_5")]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_5")))]
pub mod filter;
pub mod fit;
pub mod gamma_beta;
pub mod gegenbauer;
pub mod hypergeometric;
pub mod integration;
pub mod interpolation;
pub mod jacobian_elliptic;
pub mod laguerre;
pub mod lambert_w;
pub mod legendre;
pub mod linear_algebra;
pub mod logarithm;
pub mod minimizer;
pub mod multifit;
#[cfg(feature = "v2_1")]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_1")))]
pub mod multilarge;
pub mod multilinear;
pub mod numerical_differentiation;
pub mod physical_constant;
pub mod polynomials;
pub mod pow;
pub mod power;
pub mod psi;
pub mod roots;
pub mod sort;
pub mod statistics;
pub mod stats;
pub mod synchrotron;
pub mod transport;
pub mod trigonometric;
pub mod util;
pub mod wavelet_transforms;
pub mod zeta;

/// The maximum x such that gamma(x) is not considered an overflow.
pub static SF_GAMMA_XMAX: f64 = 171.0;
/// The maximum n such that gsl_sf_fact(n) does not give an overflow.
pub static SF_FACT_NMAX: f64 = 170.0;
/// The maximum n such that gsl_sf_doublefact(n) does not give an overflow.
pub static SF_DOUBLEFACT_NMAX: f64 = 297.0;

pub static SF_MATHIEU_COEFF: u32 = 100;

pub static DBL_EPSILON: f64 = 2.220_446_049_250_313_1e-16;
pub static SQRT_DBL_EPSILON: f64 = 1.490_116_119_384_765_6e-08;
pub static ROOT3_DBL_EPSILON: f64 = 6.055_454_452_393_342_9e-06;
pub static ROOT4_DBL_EPSILON: f64 = 1.220_703_125_000_000_0e-04;
pub static ROOT5_DBL_EPSILON: f64 = 7.400_959_797_414_050_5e-04;
pub static ROOT6_DBL_EPSILON: f64 = 2.460_783_300_575_925_1e-03;

pub static DBL_MIN: f64 = 2.225_073_858_507_201_4e-308;
pub static SQRT_DBL_MIN: f64 = 1.491_668_146_240_041_3e-154;
pub static ROOT3_DBL_MIN: f64 = 2.812_644_285_236_299_6e-103;
pub static ROOT4_DBL_MIN: f64 = 1.221_338_669_755_462_0e-77;
pub static ROOT5_DBL_MIN: f64 = 2.947_602_296_969_176_3e-62;
pub static ROOT6_DBL_MIN: f64 = 5.303_436_890_579_821_8e-52;

pub static DBL_MAX: f64 = ::std::f64::MAX; //1.7976931348623156e+308;
pub static SQRT_DBL_MAX: f64 = 1.340_780_792_994_259_6e+154;
pub static ROOT3_DBL_MAX: f64 = 5.643_803_094_122_289_7e+102;
pub static ROOT4_DBL_MAX: f64 = 1.157_920_892_373_162_0e+77;
pub static ROOT5_DBL_MAX: f64 = 4.476_546_622_757_270_7e+61;
pub static ROOT6_DBL_MAX: f64 = 2.375_668_978_229_561_2e+51;
pub static LOG_DBL_MAX: f64 = 7.097_827_128_933_839_7e+02;
