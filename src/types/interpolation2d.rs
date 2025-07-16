//
// A rust binding for the GSL library by Guillaume Gomez (guillaume1.gomez@gmail.com)
//

/*!
# 2D Interpolation

Given a set of x coordinates x_1, ..., x_m and a set of y coordinates y_1, ..., y_m, each in increasing
order, plus a set of function values z_ij for each grid point (x_i, x_j), the routines described in this
section compute a continuous interpolation function z(x,y) such that z(x_i, y_j) = z_ij.

# 2d Interpolation algorithms

The 2d Interpolation routines access the function values z_ij with the following ordering: z_ij = za[j*xsize + i]

with i=0,...,xsize-1, and j=0,...,ysize-1. However, for ease of use, the functions [`set`], [`get`] and [`idx`]
are provided to add and retrieve elements from the function grid without requiring knowledge of the internal
ordering.

## References and Further Reading

Descriptions of the interpolation algorithms and further references can be found in the following books:

C.W. Ueberhuber, Numerical Computation (Volume 1), Chapter 9 “Interpolation”, Springer (1997), ISBN 3-540-62058-3.
D.M. Young, R.T. Gregory A Survey of Numerical Mathematics (Volume 1), Chapter 6.8, Dover (1988), ISBN 0-486-65691-8.

[`set`]: #method.set.html
[`get`]: #method.get.html
[`idx`]: #method.idx.html
!*/

use crate::ffi::FFI;
use crate::{Error, InterpAccel};

ffi_wrapper!(Interp2d, *mut sys::gsl_interp2d, gsl_interp2d_free);

impl Interp2d {
    /// This function returns a pointer to a newly allocated interpolation object of type T for
    /// xsize grid points in the x direction and ysize grid points in the y direction.
    ///
    /// ```
    /// use crate::rgsl::{Interp2d, Interp2dType};
    ///
    /// let interp2d_type = Interp2dType::bilinear();
    /// let interp2d = Interp2d::new(interp2d_type, 2, 2).expect("Failed to initialize `Interp2d`...");
    /// ```
    #[doc(alias = "gsl_interp2d_alloc")]
    pub fn new(t: Interp2dType, xsize: usize, ysize: usize) -> Option<Interp2d> {
        let tmp = unsafe { sys::gsl_interp2d_alloc(t.unwrap_shared(), xsize, ysize) };

        if tmp.is_null() {
            None
        } else {
            Some(Self::wrap(tmp))
        }
    }

    /// This function initializes the interpolation object interp for the data (xa,ya,za) where xa and
    /// ya are arrays of the x and y grid points of size xsize and ysize respectively, and za is an
    /// array of function values of size xsize*ysize. The interpolation object (gsl_interp2d) does not
    /// save the data arrays xa, ya and za and only stores the static state computed from the data.
    /// The xa and ya data arrays are always assumed to be strictly ordered, with increasing x, y values;
    /// the behavior for other arrangements is not defined.
    ///
    /// Asserts that `ya.len() >= xa.len()`.
    #[doc(alias = "gsl_interp2d_init")]
    pub fn init(&mut self, xa: &[f64], ya: &[f64], za: &[f64]) -> Result<(), Error> {
        assert!(ya.len() >= xa.len());
        let ret = unsafe {
            sys::gsl_interp2d_init(
                self.unwrap_unique(),
                xa.as_ptr(),
                ya.as_ptr(),
                za.as_ptr(),
                xa.len() as _,
                ya.len() as _,
            )
        };
        Error::handle(ret, ())
    }

    /// This function returns the name of the interpolation type used by interp. For example,
    ///
    /// ```
    /// use crate::rgsl::{Interp2d, Interp2dType};
    ///
    /// let interp2d_type = Interp2dType::bilinear();
    /// let interp2d = Interp2d::new(interp2d_type, 2, 2).expect("Failed to initialize `Interp2d`...");
    /// println!("interp uses '{}' interpolation.", interp2d.name());
    /// ```
    ///
    /// would print something like :
    ///
    /// ```Shell
    /// interp uses 'bilinear' interpolation.
    /// ```
    #[doc(alias = "gsl_interp2d_name")]
    pub fn name(&self) -> String {
        let tmp = unsafe { sys::gsl_interp2d_name(self.unwrap_shared()) };

        if tmp.is_null() {
            String::new()
        } else {
            unsafe { String::from_utf8_lossy(std::ffi::CStr::from_ptr(tmp).to_bytes()).to_string() }
        }
    }

    /// This function returns the minimum number of points required by the interpolation object
    /// interp or interpolation type T. For example, bicubic interpolation requires a minimum
    /// of 4 points.
    #[doc(alias = "gsl_interp2d_min_size")]
    pub fn min_size(&self) -> usize {
        unsafe { sys::gsl_interp2d_min_size(self.unwrap_shared()) }
    }

    /// This function sets the value z_ij for grid point (i,j) of the array za to z.
    #[doc(alias = "gsl_interp2d_set")]
    pub fn set(&mut self, za: &mut [f64], i: usize, j: usize, z: f64) {
        unsafe { sys::gsl_interp2d_set(self.unwrap_shared(), za.as_mut_ptr(), i, j, z) };
    }

    /// This function returns the value z_ij for grid point (i,j) stored in the array za.
    #[doc(alias = "gsl_interp2d_set")]
    pub fn get(&mut self, za: &mut [f64], i: usize, j: usize) -> f64 {
        unsafe { sys::gsl_interp2d_get(self.unwrap_shared(), za.as_mut_ptr(), i, j) }
    }

    /// This function returns the index corresponding to the grid point (i,j). The index is given by
    /// z*xsize + i.
    #[doc(alias = "gsl_interp2d_set")]
    pub fn idx(&mut self, i: usize, j: usize) -> usize {
        unsafe { sys::gsl_interp2d_idx(self.unwrap_shared(), i, j) }
    }
}

ffi_wrapper!(Interp2dType, *const sys::gsl_interp2d_type);

impl Interp2dType {
    /// This function returns the minimum number of points required by the interpolation object
    /// interp or interpolation type T. For example, bicubic interpolation requires a minimum
    /// of 4 points.
    #[doc(alias = "gsl_interp2d_type_min_size")]
    pub fn min_size(&self) -> usize {
        unsafe { sys::gsl_interp2d_type_min_size(self.unwrap_shared()) }
    }

    /// Bilinear interpolation. This interpolation method does not require any additional memory.
    #[doc(alias = "gsl_interp2d_bilinear")]
    pub fn bilinear() -> Interp2dType {
        ffi_wrap!(gsl_interp2d_bilinear)
    }

    /// Bicubib interpolation.
    #[doc(alias = "gsl_interp2d_bicubic")]
    pub fn bicubic() -> Interp2dType {
        ffi_wrap!(gsl_interp2d_bicubic)
    }
}

ffi_wrapper!(
    Spline2d,
    *mut sys::gsl_spline2d,
    gsl_spline2d_free,
    "General interpolation object."
);

impl Spline2d {
    #[doc(alias = "gsl_spline2d_alloc")]
    pub fn new(t: Interp2dType, xsize: usize, ysize: usize) -> Option<Spline2d> {
        let tmp = unsafe { sys::gsl_spline2d_alloc(t.unwrap_shared(), xsize, ysize) };

        if tmp.is_null() {
            None
        } else {
            Some(Self::wrap(tmp))
        }
    }

    #[doc(alias = "gsl_spline2d_init")]
    pub fn init(&mut self, xa: &[f64], ya: &[f64], za: &[f64]) -> Result<(), Error> {
        let ret = unsafe {
            sys::gsl_spline2d_init(
                self.unwrap_unique(),
                xa.as_ptr(),
                ya.as_ptr(),
                za.as_ptr(),
                xa.len() as _,
                ya.len() as _,
            )
        };
        Error::handle(ret, ())
    }

    #[doc(alias = "gsl_spline2d_name")]
    pub fn name(&self) -> String {
        let tmp = unsafe { sys::gsl_spline2d_name(self.unwrap_shared()) };

        if tmp.is_null() {
            String::new()
        } else {
            unsafe { String::from_utf8_lossy(std::ffi::CStr::from_ptr(tmp).to_bytes()).to_string() }
        }
    }

    #[doc(alias = "gsl_spline2d_min_size")]
    pub fn min_size(&self) -> usize {
        unsafe { sys::gsl_spline2d_min_size(self.unwrap_shared()) }
    }

    #[doc(alias = "gsl_spline2d_eval")]
    pub fn eval(&self, x: f64, y: f64, xacc: &mut InterpAccel, yacc: &mut InterpAccel) -> f64 {
        unsafe { sys::gsl_spline2d_eval(self.unwrap_shared(), x, y, &mut xacc.0, &mut yacc.0) }
    }

    /// Returns `z`.
    #[doc(alias = "gsl_spline2d_eval_e")]
    pub fn eval_e(
        &self,
        x: f64,
        y: f64,
        xacc: &mut InterpAccel,
        yacc: &mut InterpAccel,
    ) -> Result<f64, Error> {
        let mut z = 0.;
        let ret = unsafe {
            sys::gsl_spline2d_eval_e(self.unwrap_shared(), x, y, &mut xacc.0, &mut yacc.0, &mut z)
        };
        Error::handle(ret, z)
    }

    #[doc(alias = "gsl_spline2d_eval_extrap")]
    pub fn eval_extrap(
        &self,
        x: f64,
        y: f64,
        xacc: &mut InterpAccel,
        yacc: &mut InterpAccel,
    ) -> f64 {
        unsafe {
            sys::gsl_spline2d_eval_extrap(self.unwrap_shared(), x, y, &mut xacc.0, &mut yacc.0)
        }
    }

    /// Returns `z`.
    #[doc(alias = "gsl_spline2d_eval_extrap_e")]
    pub fn eval_extrap_e(
        &self,
        x: f64,
        y: f64,
        xacc: &mut InterpAccel,
        yacc: &mut InterpAccel,
    ) -> Result<f64, Error> {
        let mut z = 0.;
        let ret = unsafe {
            sys::gsl_spline2d_eval_extrap_e(
                self.unwrap_shared(),
                x,
                y,
                &mut xacc.0,
                &mut yacc.0,
                &mut z,
            )
        };
        Error::handle(ret, z)
    }

    #[doc(alias = "gsl_spline2d_eval_deriv_x")]
    pub fn eval_deriv_x(
        &self,
        x: f64,
        y: f64,
        xacc: &mut InterpAccel,
        yacc: &mut InterpAccel,
    ) -> f64 {
        unsafe {
            sys::gsl_spline2d_eval_deriv_x(self.unwrap_shared(), x, y, &mut xacc.0, &mut yacc.0)
        }
    }

    /// Returns `d`.
    #[doc(alias = "gsl_spline2d_eval_deriv_x_e")]
    pub fn eval_deriv_x_e(
        &self,
        x: f64,
        y: f64,
        xacc: &mut InterpAccel,
        yacc: &mut InterpAccel,
    ) -> Result<f64, Error> {
        let mut d = 0.;
        let ret = unsafe {
            sys::gsl_spline2d_eval_deriv_x_e(
                self.unwrap_shared(),
                x,
                y,
                &mut xacc.0,
                &mut yacc.0,
                &mut d,
            )
        };
        Error::handle(ret, d)
    }

    #[doc(alias = "gsl_spline2d_eval_deriv_y")]
    pub fn eval_deriv_y(
        &self,
        x: f64,
        y: f64,
        xacc: &mut InterpAccel,
        yacc: &mut InterpAccel,
    ) -> f64 {
        unsafe {
            sys::gsl_spline2d_eval_deriv_y(self.unwrap_shared(), x, y, &mut xacc.0, &mut yacc.0)
        }
    }

    /// Returns `d`.
    #[doc(alias = "gsl_spline2d_eval_deriv_y_e")]
    pub fn eval_deriv_y_e(
        &self,
        x: f64,
        y: f64,
        xacc: &mut InterpAccel,
        yacc: &mut InterpAccel,
    ) -> Result<f64, Error> {
        let mut d = 0.;
        let ret = unsafe {
            sys::gsl_spline2d_eval_deriv_y_e(
                self.unwrap_shared(),
                x,
                y,
                &mut xacc.0,
                &mut yacc.0,
                &mut d,
            )
        };
        Error::handle(ret, d)
    }

    #[doc(alias = "gsl_spline2d_eval_deriv_xx")]
    pub fn eval_deriv_xx(
        &self,
        x: f64,
        y: f64,
        xacc: &mut InterpAccel,
        yacc: &mut InterpAccel,
    ) -> f64 {
        unsafe {
            sys::gsl_spline2d_eval_deriv_xx(self.unwrap_shared(), x, y, &mut xacc.0, &mut yacc.0)
        }
    }

    /// Returns `d`.
    #[doc(alias = "gsl_spline2d_eval_deriv_xx_e")]
    pub fn eval_deriv_xx_e(
        &self,
        x: f64,
        y: f64,
        xacc: &mut InterpAccel,
        yacc: &mut InterpAccel,
    ) -> Result<f64, Error> {
        let mut d = 0.;
        let ret = unsafe {
            sys::gsl_spline2d_eval_deriv_xx_e(
                self.unwrap_shared(),
                x,
                y,
                &mut xacc.0,
                &mut yacc.0,
                &mut d,
            )
        };
        Error::handle(ret, d)
    }

    #[doc(alias = "gsl_spline2d_eval_deriv_yy")]
    pub fn eval_deriv_yy(
        &self,
        x: f64,
        y: f64,
        xacc: &mut InterpAccel,
        yacc: &mut InterpAccel,
    ) -> f64 {
        unsafe {
            sys::gsl_spline2d_eval_deriv_yy(self.unwrap_shared(), x, y, &mut xacc.0, &mut yacc.0)
        }
    }

    /// Returns `z`.
    #[doc(alias = "gsl_spline2d_eval_deriv_yy_e")]
    pub fn eval_deriv_yy_e(
        &self,
        x: f64,
        y: f64,
        xacc: &mut InterpAccel,
        yacc: &mut InterpAccel,
    ) -> Result<f64, Error> {
        let mut d = 0.;
        let ret = unsafe {
            sys::gsl_spline2d_eval_deriv_yy_e(
                self.unwrap_shared(),
                x,
                y,
                &mut xacc.0,
                &mut yacc.0,
                &mut d,
            )
        };
        Error::handle(ret, d)
    }

    #[doc(alias = "gsl_spline2d_eval_deriv_xy")]
    pub fn eval_deriv_xy(
        &self,
        x: f64,
        y: f64,
        xacc: &mut InterpAccel,
        yacc: &mut InterpAccel,
    ) -> f64 {
        unsafe {
            sys::gsl_spline2d_eval_deriv_xy(self.unwrap_shared(), x, y, &mut xacc.0, &mut yacc.0)
        }
    }

    /// Returns `z`.
    #[doc(alias = "gsl_spline2d_eval_deriv_xy_e")]
    pub fn eval_deriv_xy_e(
        &self,
        x: f64,
        y: f64,
        xacc: &mut InterpAccel,
        yacc: &mut InterpAccel,
    ) -> Result<f64, Error> {
        let mut d = 0.;
        let ret = unsafe {
            sys::gsl_spline2d_eval_deriv_xy_e(
                self.unwrap_shared(),
                x,
                y,
                &mut xacc.0,
                &mut yacc.0,
                &mut d,
            )
        };
        Error::handle(ret, d)
    }

    /// This function sets the value z_ij for grid point (i,j) of the array za to z.
    #[doc(alias = "gsl_spline2d_set")]
    pub fn set(&mut self, za: &mut [f64], i: usize, j: usize, z: f64) {
        unsafe { sys::gsl_spline2d_set(self.unwrap_shared(), za.as_mut_ptr(), i, j, z) };
    }

    /// This function returns the value z_ij for grid point (i,j) stored in the array za.
    #[doc(alias = "gsl_interp2d_set")]
    pub fn get(&mut self, za: &mut [f64], i: usize, j: usize) -> f64 {
        unsafe { sys::gsl_spline2d_get(self.unwrap_shared(), za.as_mut_ptr(), i, j) }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_inter2d_type() {
        let bilinear = Interp2dType::bilinear();
        let bicubic = Interp2dType::bicubic();

        bilinear.min_size();
        bicubic.min_size();
    }

    #[test]
    fn test_interp2d() {
        let xa = vec![0.0, 1.0];
        let ya = vec![2.0, 3.0];
        let mut za = vec![4.0, 5.0, 6.0, 7.0];

        let interp2d_type = Interp2dType::bilinear();
        let mut interp2d = Interp2d::new(interp2d_type, 2, 2).unwrap();

        interp2d.init(&xa, &ya, &za).unwrap();
        interp2d.name();
        interp2d.min_size();
        interp2d.set(&mut za, 0, 1, 60.0);

        assert_eq!(za, vec![4.0, 5.0, 60.0, 7.0]);
        assert_eq!(interp2d.get(&mut za, 1, 1), 7.0);
        assert_eq!(interp2d.idx(1, 1), 3);
    }

    #[test]
    fn test_spline2d() {
        let interp2d_type = Interp2dType::bilinear();

        let xa = vec![0.0, 1.0];
        let ya = vec![2.0, 3.0];
        let mut za = vec![4.0, 5.0, 6.0, 7.0];
        let mut xacc = InterpAccel::new();
        let mut yacc = InterpAccel::new();

        let mut spline2d = Spline2d::new(interp2d_type, 2, 2).unwrap();
        spline2d.init(&xa, &ya, &za).unwrap();
        spline2d.name();
        spline2d.min_size();

        spline2d.eval(0.5, 2.5, &mut xacc, &mut yacc);
        spline2d.eval_extrap(0.5, 2.5, &mut xacc, &mut yacc);
        spline2d.eval_deriv_x(0.5, 2.5, &mut xacc, &mut yacc);
        spline2d.eval_deriv_y(0.5, 2.5, &mut xacc, &mut yacc);
        spline2d.eval_deriv_xx(0.5, 2.5, &mut xacc, &mut yacc);
        spline2d.eval_deriv_yy(0.5, 2.5, &mut xacc, &mut yacc);
        spline2d.eval_deriv_xy(0.5, 2.5, &mut xacc, &mut yacc);

        spline2d.eval_e(0.5, 2.5, &mut xacc, &mut yacc).unwrap();
        spline2d
            .eval_extrap_e(0.5, 2.5, &mut xacc, &mut yacc)
            .unwrap();
        spline2d
            .eval_deriv_x_e(0.5, 2.5, &mut xacc, &mut yacc)
            .unwrap();
        spline2d
            .eval_deriv_y_e(0.5, 2.5, &mut xacc, &mut yacc)
            .unwrap();
        spline2d
            .eval_deriv_xx_e(0.5, 2.5, &mut xacc, &mut yacc)
            .unwrap();
        spline2d
            .eval_deriv_yy_e(0.5, 2.5, &mut xacc, &mut yacc)
            .unwrap();
        spline2d
            .eval_deriv_xy_e(0.5, 2.5, &mut xacc, &mut yacc)
            .unwrap();

        spline2d.name();
        spline2d.min_size();
        spline2d.set(&mut za, 0, 1, 60.0);

        assert_eq!(za, vec![4.0, 5.0, 60.0, 7.0]);
        assert_eq!(spline2d.get(&mut za, 1, 1), 7.0);
    }
}
