//
// A rust binding for the GSL library by Guillaume Gomez (guillaume1.gomez@gmail.com)
//

use crate::ffi::FFI;
use crate::Error;

pub use crate::interpolation::bsearch;
pub use crate::InterpAccel;

/// This function returns the interpolated value of z for a given point (x,y), using the interpolation
/// object interp, data arrays xa, ya and za and the accelerators xacc and zacc. When x is outside the
/// range of xa or y is outside the range of ya, the error code crate::Dom is returned with a value
/// of rgsl::NAN for z.
#[doc(alias = "gsl_interp2d_eval")]
pub fn eval(
    interp2d: &crate::Interp2d,
    xa: &[f64],
    ya: &[f64],
    za: &[f64],
    x: f64,
    y: f64,
    xacc: &mut crate::InterpAccel,
    yacc: &mut crate::InterpAccel,
) -> f64 {
    unsafe {
        sys::gsl_interp2d_eval(
            interp2d.unwrap_shared(),
            xa.as_ptr(),
            ya.as_ptr(),
            za.as_ptr(),
            x,
            y,
            &mut xacc.0,
            &mut yacc.0,
        )
    }
}

/// This function returns the interpolated value of z for a given point (x,y), using the interpolation
/// object interp, data arrays xa, ya and za and the accelerators xacc and zacc. When x is outside the
/// range of xa or y is outside the range of ya, the error code crate::Dom is returned with a value
/// of rgsl::NAN for z.
///
/// Returns `z`.
#[doc(alias = "gsl_interp2d_eval_e")]
pub fn eval_e(
    interp2d: &crate::Interp2d,
    xa: &[f64],
    ya: &[f64],
    za: &[f64],
    x: f64,
    y: f64,
    xacc: &mut crate::InterpAccel,
    yacc: &mut crate::InterpAccel,
) -> Result<f64, Error> {
    let mut z = 0.;
    let ret = unsafe {
        sys::gsl_interp2d_eval_e(
            interp2d.unwrap_shared(),
            xa.as_ptr(),
            ya.as_ptr(),
            za.as_ptr(),
            x,
            y,
            &mut xacc.0,
            &mut yacc.0,
            &mut z,
        )
    };
    Error::handle(ret, z)
}

/// This function returns the interpolated value of z for a given point (x, y), using the
/// interpolation object interp, data arrays xa, ya, and za and the accelerators xacc and yacc. The
/// functions perform no bounds checking, so when x is outside the range of xa or y is outside the
/// range of ya, extrapolation is performed.
#[doc(alias = "gsl_interp2d_eval_extrap")]
pub fn eval_extrap(
    interp2d: &crate::Interp2d,
    xa: &[f64],
    ya: &[f64],
    za: &[f64],
    x: f64,
    y: f64,
    xacc: &mut crate::InterpAccel,
    yacc: &mut crate::InterpAccel,
) -> f64 {
    unsafe {
        sys::gsl_interp2d_eval_extrap(
            interp2d.unwrap_shared(),
            xa.as_ptr(),
            ya.as_ptr(),
            za.as_ptr(),
            x,
            y,
            &mut xacc.0,
            &mut yacc.0,
        )
    }
}

/// This function returns the interpolated value of z for a given point (x, y), using the
/// interpolation object interp, data arrays xa, ya, and za and the accelerators xacc and yacc. The
/// functions perform no bounds checking, so when x is outside the range of xa or y is outside the
/// range of ya, extrapolation is performed.
///
/// Returns 'z'.
#[doc(alias = "gsl_interp2d_eval_extrap_e")]
pub fn eval_extrap_e(
    interp2d: &crate::Interp2d,
    xa: &[f64],
    ya: &[f64],
    za: &[f64],
    x: f64,
    y: f64,
    xacc: &mut crate::InterpAccel,
    yacc: &mut crate::InterpAccel,
) -> Result<f64, Error> {
    let mut z = 0.;
    let ret = unsafe {
        sys::gsl_interp2d_eval_extrap_e(
            interp2d.unwrap_shared(),
            xa.as_ptr(),
            ya.as_ptr(),
            za.as_ptr(),
            x,
            y,
            &mut xacc.0,
            &mut yacc.0,
            &mut z,
        )
    };
    Error::handle(ret, z)
}
