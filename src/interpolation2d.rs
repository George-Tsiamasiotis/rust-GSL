//
// A rust binding for the GSL library by Guillaume Gomez (guillaume1.gomez@gmail.com)
//

use crate::ffi::FFI;
use crate::Error;

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

/// This function returns the interpolated value of d=dz/dx (partial) for a given point (x, y), using
/// the interpolation object interp, data arrays xa, ya, and za and the accelerators xacc and yacc.
/// When x is outside the range of xa or y is outside the range of ya, the error code crate::Dom is
/// returned with a value of rgsl::NAN for d.
#[doc(alias = "gsl_interp2d_eval_deriv_x")]
pub fn eval_deriv_x(
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
        sys::gsl_interp2d_eval_deriv_x(
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

/// This function returns the interpolated value of d=dz/dx (partial) for a given point (x, y), using
/// the interpolation object interp, data arrays xa, ya, and za and the accelerators xacc and yacc.
/// When x is outside the range of xa or y is outside the range of ya, the error code crate::Dom is
/// returned with a value of rgsl::NAN for d.
///
/// Returns 'd'.
#[doc(alias = "gsl_interp2d_eval_deriv_x_e")]
pub fn eval_deriv_x_e(
    interp2d: &crate::Interp2d,
    xa: &[f64],
    ya: &[f64],
    za: &[f64],
    x: f64,
    y: f64,
    xacc: &mut crate::InterpAccel,
    yacc: &mut crate::InterpAccel,
) -> Result<f64, Error> {
    let mut d = 0.;
    let ret = unsafe {
        sys::gsl_interp2d_eval_deriv_x_e(
            interp2d.unwrap_shared(),
            xa.as_ptr(),
            ya.as_ptr(),
            za.as_ptr(),
            x,
            y,
            &mut xacc.0,
            &mut yacc.0,
            &mut d,
        )
    };
    Error::handle(ret, d)
}

/// This function returns the interpolated value of d=dz/dy (partial) for a given point (x, y), using
/// the interpolation object interp, data arrays xa, ya, and za and the accelerators xacc and yacc.
/// When x is outside the range of xa or y is outside the range of ya, the error code crate::Dom is
/// returned with a value of rgsl::NAN for d.
#[doc(alias = "gsl_interp2d_eval_deriv_y")]
pub fn eval_deriv_y(
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
        sys::gsl_interp2d_eval_deriv_y(
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

/// This function returns the interpolated value of d=dz/dy (partial) for a given point (x, y), using
/// the interpolation object interp, data arrays xa, ya, and za and the accelerators xacc and yacc.
/// When x is outside the range of xa or y is outside the range of ya, the error code crate::Dom is
/// returned with a value of rgsl::NAN for d.
///
/// Returns 'd'.
#[doc(alias = "gsl_interp2d_eval_deriv_y_e")]
pub fn eval_deriv_y_e(
    interp2d: &crate::Interp2d,
    xa: &[f64],
    ya: &[f64],
    za: &[f64],
    x: f64,
    y: f64,
    xacc: &mut crate::InterpAccel,
    yacc: &mut crate::InterpAccel,
) -> Result<f64, Error> {
    let mut d = 0.;
    let ret = unsafe {
        sys::gsl_interp2d_eval_deriv_y_e(
            interp2d.unwrap_shared(),
            xa.as_ptr(),
            ya.as_ptr(),
            za.as_ptr(),
            x,
            y,
            &mut xacc.0,
            &mut yacc.0,
            &mut d,
        )
    };
    Error::handle(ret, d)
}

/// This function returns the interpolated value of d=d^2z/dx^2 (partial) for a given point (x, y), using
/// the interpolation object interp, data arrays xa, ya, and za and the accelerators xacc and yacc.
/// When x is outside the range of xa or y is outside the range of ya, the error code crate::Dom is
/// returned with a value of rgsl::NAN for d.
#[doc(alias = "gsl_interp2d_eval_deriv_xx")]
pub fn eval_deriv_xx(
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
        sys::gsl_interp2d_eval_deriv_xx(
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

/// This function returns the interpolated value of d=d^2z/dx^2 (partial) for a given point (x, y), using
/// the interpolation object interp, data arrays xa, ya, and za and the accelerators xacc and yacc.
/// When x is outside the range of xa or y is outside the range of ya, the error code crate::Dom is
/// returned with a value of rgsl::NAN for d.
///
/// Returns 'd'.
#[doc(alias = "gsl_interp2d_eval_deriv_xx_e")]
pub fn eval_deriv_xx_e(
    interp2d: &crate::Interp2d,
    xa: &[f64],
    ya: &[f64],
    za: &[f64],
    x: f64,
    y: f64,
    xacc: &mut crate::InterpAccel,
    yacc: &mut crate::InterpAccel,
) -> Result<f64, Error> {
    let mut d = 0.;
    let ret = unsafe {
        sys::gsl_interp2d_eval_deriv_xx_e(
            interp2d.unwrap_shared(),
            xa.as_ptr(),
            ya.as_ptr(),
            za.as_ptr(),
            x,
            y,
            &mut xacc.0,
            &mut yacc.0,
            &mut d,
        )
    };
    Error::handle(ret, d)
}

/// This function returns the interpolated value of d=d^2z/dy^2 (partial) for a given point (x, y), using
/// the interpolation object interp, data arrays xa, ya, and za and the accelerators xacc and yacc.
/// When x is outside the range of xa or y is outside the range of ya, the error code crate::Dom is
/// returned with a value of rgsl::NAN for d.
#[doc(alias = "gsl_interp2d_eval_deriv_yy")]
pub fn eval_deriv_yy(
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
        sys::gsl_interp2d_eval_deriv_yy(
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

/// This function returns the interpolated value of d=d^2z/dy^2 (partial) for a given point (x, y), using
/// the interpolation object interp, data arrays xa, ya, and za and the accelerators xacc and yacc.
/// When x is outside the range of xa or y is outside the range of ya, the error code crate::Dom is
/// returned with a value of rgsl::NAN for d.
///
/// Returns 'd'.
#[doc(alias = "gsl_interp2d_eval_deriv_yy_e")]
pub fn eval_deriv_yy_e(
    interp2d: &crate::Interp2d,
    xa: &[f64],
    ya: &[f64],
    za: &[f64],
    x: f64,
    y: f64,
    xacc: &mut crate::InterpAccel,
    yacc: &mut crate::InterpAccel,
) -> Result<f64, Error> {
    let mut d = 0.;
    let ret = unsafe {
        sys::gsl_interp2d_eval_deriv_yy_e(
            interp2d.unwrap_shared(),
            xa.as_ptr(),
            ya.as_ptr(),
            za.as_ptr(),
            x,
            y,
            &mut xacc.0,
            &mut yacc.0,
            &mut d,
        )
    };
    Error::handle(ret, d)
}

/// This function returns the interpolated value of d=d^2z/dxdy (partial) for a given point (x, y), using
/// the interpolation object interp, data arrays xa, ya, and za and the accelerators xacc and yacc.
/// When x is outside the range of xa or y is outside the range of ya, the error code crate::Dom is
/// returned with a value of rgsl::NAN for d.
#[doc(alias = "gsl_interp2d_eval_deriv_xy")]
pub fn eval_deriv_xy(
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
        sys::gsl_interp2d_eval_deriv_xy(
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

/// This function returns the interpolated value of d=d^2z/dxdy (partial) for a given point (x, y), using
/// the interpolation object interp, data arrays xa, ya, and za and the accelerators xacc and yacc.
/// When x is outside the range of xa or y is outside the range of ya, the error code crate::Dom is
/// returned with a value of rgsl::NAN for d.
///
/// Returns 'd'.
#[doc(alias = "gsl_interp2d_eval_deriv_xy_e")]
pub fn eval_deriv_xy_e(
    interp2d: &crate::Interp2d,
    xa: &[f64],
    ya: &[f64],
    za: &[f64],
    x: f64,
    y: f64,
    xacc: &mut crate::InterpAccel,
    yacc: &mut crate::InterpAccel,
) -> Result<f64, Error> {
    let mut d = 0.;
    let ret = unsafe {
        sys::gsl_interp2d_eval_deriv_xy_e(
            interp2d.unwrap_shared(),
            xa.as_ptr(),
            ya.as_ptr(),
            za.as_ptr(),
            x,
            y,
            &mut xacc.0,
            &mut yacc.0,
            &mut d,
        )
    };
    Error::handle(ret, d)
}
