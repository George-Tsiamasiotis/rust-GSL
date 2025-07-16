//
// A rust binding for the GSL library by Guillaume Gomez (guillaume1.gomez@gmail.com)
//

/*!
# 2D Interpolation

Given a set of x coordinates x_1, ..., x_m and a set of y coordinates y_1, ..., y_m, each in increasing
order, plus a set of function values z_ij for each grid point (x_i, x_j), the routines described in this
section compute a continuous interpolation function z(x,y) such that z(x_i, y_j) = z_ij.

## References and Further Reading

Descriptions of the interpolation algorithms and further references can be found in the following books:

C.W. Ueberhuber, Numerical Computation (Volume 1), Chapter 9 “Interpolation”, Springer (1997), ISBN 3-540-62058-3.
D.M. Young, R.T. Gregory A Survey of Numerical Mathematics (Volume 1), Chapter 6.8, Dover (1988), ISBN 0-486-65691-8.
!*/

use crate::ffi::FFI;
use crate::Error;

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
}

ffi_wrapper!(Interp2dType, *const sys::gsl_interp2d_type);

impl Interp2dType {}
