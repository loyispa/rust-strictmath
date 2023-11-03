/* automatically generated by rust-bindgen 0.65.1 */

pub const DOMAIN: u32 = 1;
pub const SING: u32 = 2;
pub const OVERFLOW: u32 = 3;
pub const UNDERFLOW: u32 = 4;
pub const TLOSS: u32 = 5;
pub const PLOSS: u32 = 6;

extern "C" {
    pub static mut signgam: ::std::os::raw::c_int;
}

pub const fdversion_fdlibm_ieee: fdversion = -1;
pub const fdversion_fdlibm_svid: fdversion = 0;
pub const fdversion_fdlibm_xopen: fdversion = 1;
pub const fdversion_fdlibm_posix: fdversion = 2;

pub type fdversion = ::std::os::raw::c_int;

extern "C" {
    pub static mut _fdlib_version: fdversion;
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct exception {
    pub type_: ::std::os::raw::c_int,
    pub name: *mut ::std::os::raw::c_char,
    pub arg1: f64,
    pub arg2: f64,
    pub retval: f64,
}

#[test]
fn bindgen_test_layout_exception() {
    const UNINIT: ::std::mem::MaybeUninit<exception> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<exception>(),
        40usize,
        concat!("Size of: ", stringify!(exception))
    );
    assert_eq!(
        ::std::mem::align_of::<exception>(),
        8usize,
        concat!("Alignment of ", stringify!(exception))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).type_) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(exception),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).name) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(exception),
            "::",
            stringify!(name)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).arg1) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(exception),
            "::",
            stringify!(arg1)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).arg2) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(exception),
            "::",
            stringify!(arg2)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).retval) as usize - ptr as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(exception),
            "::",
            stringify!(retval)
        )
    );
}

extern "C" {
    pub fn racos(arg1: f64) -> f64;
}

extern "C" {
    pub fn rasin(arg1: f64) -> f64;
}

extern "C" {
    pub fn ratan(arg1: f64) -> f64;
}

extern "C" {
    pub fn ratan2(arg1: f64, arg2: f64) -> f64;
}

extern "C" {
    pub fn rcos(arg1: f64) -> f64;
}

extern "C" {
    pub fn rsin(arg1: f64) -> f64;
}

extern "C" {
    pub fn rtan(arg1: f64) -> f64;
}

extern "C" {
    pub fn rcosh(arg1: f64) -> f64;
}

extern "C" {
    pub fn rsinh(arg1: f64) -> f64;
}

extern "C" {
    pub fn rtanh(arg1: f64) -> f64;
}

extern "C" {
    pub fn rexp(arg1: f64) -> f64;
}

extern "C" {
    pub fn frexp(arg1: f64, arg2: *mut ::std::os::raw::c_int) -> f64;
}

extern "C" {
    pub fn ldexp(arg1: f64, arg2: ::std::os::raw::c_int) -> f64;
}

extern "C" {
    pub fn rlog(arg1: f64) -> f64;
}

extern "C" {
    pub fn rlog10(arg1: f64) -> f64;
}

extern "C" {
    pub fn modf(arg1: f64, arg2: *mut f64) -> f64;
}

extern "C" {
    pub fn rpow(arg1: f64, arg2: f64) -> f64;
}

extern "C" {
    pub fn rsqrt(arg1: f64) -> f64;
}

extern "C" {
    pub fn rceil(arg1: f64) -> f64;
}

extern "C" {
    pub fn rfabs(arg1: f64) -> f64;
}

extern "C" {
    pub fn rfloor(arg1: f64) -> f64;
}

extern "C" {
    pub fn rmod(arg1: f64, arg2: f64) -> f64;
}

extern "C" {
    pub fn erf(arg1: f64) -> f64;
}

extern "C" {
    pub fn erfc(arg1: f64) -> f64;
}

extern "C" {
    pub fn gamma(arg1: f64) -> f64;
}

extern "C" {
    pub fn rhypot(arg1: f64, arg2: f64) -> f64;
}

extern "C" {
    pub fn isnan(arg1: f64) -> ::std::os::raw::c_int;
}

extern "C" {
    pub fn finite(arg1: f64) -> ::std::os::raw::c_int;
}

extern "C" {
    pub fn j0(arg1: f64) -> f64;
}

extern "C" {
    pub fn j1(arg1: f64) -> f64;
}

extern "C" {
    pub fn jn(arg1: ::std::os::raw::c_int, arg2: f64) -> f64;
}

extern "C" {
    pub fn lgamma(arg1: f64) -> f64;
}

extern "C" {
    pub fn y0(arg1: f64) -> f64;
}

extern "C" {
    pub fn y1(arg1: f64) -> f64;
}

extern "C" {
    pub fn yn(arg1: ::std::os::raw::c_int, arg2: f64) -> f64;
}

extern "C" {
    pub fn acosh(arg1: f64) -> f64;
}

extern "C" {
    pub fn asinh(arg1: f64) -> f64;
}

extern "C" {
    pub fn atanh(arg1: f64) -> f64;
}

extern "C" {
    pub fn rcbrt(arg1: f64) -> f64;
}

extern "C" {
    pub fn logb(arg1: f64) -> f64;
}

extern "C" {
    pub fn nextafter(arg1: f64, arg2: f64) -> f64;
}

extern "C" {
    pub fn rremainder(arg1: f64, arg2: f64) -> f64;
}

extern "C" {
    pub fn scalb(arg1: f64, arg2: f64) -> f64;
}

extern "C" {
    pub fn matherr(arg1: *mut exception) -> ::std::os::raw::c_int;
}

extern "C" {
    pub fn significand(arg1: f64) -> f64;
}

extern "C" {
    pub fn copysign(arg1: f64, arg2: f64) -> f64;
}

extern "C" {
    pub fn ilogb(arg1: f64) -> ::std::os::raw::c_int;
}

extern "C" {
    pub fn rint(arg1: f64) -> f64;
}

extern "C" {
    pub fn scalbn(arg1: f64, arg2: ::std::os::raw::c_int) -> f64;
}

extern "C" {
    pub fn rexpm1(arg1: f64) -> f64;
}

extern "C" {
    pub fn rlog1p(arg1: f64) -> f64;
}

extern "C" {
    pub fn __r__ieee754_sqrt(arg1: f64) -> f64;
}

extern "C" {
    pub fn __r__ieee754_acos(arg1: f64) -> f64;
}

extern "C" {
    pub fn __ieee754_acosh(arg1: f64) -> f64;
}

extern "C" {
    pub fn __r__ieee754_log(arg1: f64) -> f64;
}

extern "C" {
    pub fn __r__ieee754_atanh(arg1: f64) -> f64;
}

extern "C" {
    pub fn __r__ieee754_asin(arg1: f64) -> f64;
}

extern "C" {
    pub fn __r__ieee754_atan2(arg1: f64, arg2: f64) -> f64;
}

extern "C" {
    pub fn __r__ieee754_exp(arg1: f64) -> f64;
}

extern "C" {
    pub fn __r__ieee754_cosh(arg1: f64) -> f64;
}

extern "C" {
    pub fn __r__ieee754_fmod(arg1: f64, arg2: f64) -> f64;
}

extern "C" {
    pub fn __r__ieee754_pow(arg1: f64, arg2: f64) -> f64;
}

extern "C" {
    pub fn __ieee754_lgamma_r(arg1: f64, arg2: *mut ::std::os::raw::c_int) -> f64;
}

extern "C" {
    pub fn __ieee754_gamma_r(arg1: f64, arg2: *mut ::std::os::raw::c_int) -> f64;
}

extern "C" {
    pub fn __ieee754_lgamma(arg1: f64) -> f64;
}

extern "C" {
    pub fn __ieee754_gamma(arg1: f64) -> f64;
}

extern "C" {
    pub fn __r__ieee754_log10(arg1: f64) -> f64;
}

extern "C" {
    pub fn __r__ieee754_sinh(arg1: f64) -> f64;
}

extern "C" {
    pub fn __r__ieee754_hypot(arg1: f64, arg2: f64) -> f64;
}

extern "C" {
    pub fn __ieee754_j0(arg1: f64) -> f64;
}

extern "C" {
    pub fn __ieee754_j1(arg1: f64) -> f64;
}

extern "C" {
    pub fn __ieee754_y0(arg1: f64) -> f64;
}

extern "C" {
    pub fn __ieee754_y1(arg1: f64) -> f64;
}

extern "C" {
    pub fn __ieee754_jn(arg1: ::std::os::raw::c_int, arg2: f64) -> f64;
}

extern "C" {
    pub fn __ieee754_yn(arg1: ::std::os::raw::c_int, arg2: f64) -> f64;
}

extern "C" {
    pub fn __r__ieee754_remainder(arg1: f64, arg2: f64) -> f64;
}

extern "C" {
    pub fn __r__ieee754_rem_pio2(arg1: f64, arg2: *mut f64) -> ::std::os::raw::c_int;
}

extern "C" {
    pub fn __r__ieee754_scalb(arg1: f64, arg2: f64) -> f64;
}

extern "C" {
    pub fn __r__kernel_standard(arg1: f64, arg2: f64, arg3: ::std::os::raw::c_int) -> f64;
}

extern "C" {
    pub fn __r__kernel_sin(arg1: f64, arg2: f64, arg3: ::std::os::raw::c_int) -> f64;
}

extern "C" {
    pub fn __r__kernel_cos(arg1: f64, arg2: f64) -> f64;
}

extern "C" {
    pub fn __r__kernel_tan(arg1: f64, arg2: f64, arg3: ::std::os::raw::c_int) -> f64;
}

extern "C" {
    pub fn __r__kernel_rem_pio2(
        arg1: *mut f64,
        arg2: *mut f64,
        arg3: ::std::os::raw::c_int,
        arg4: ::std::os::raw::c_int,
        arg5: ::std::os::raw::c_int,
        arg6: *const ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}

pub fn acos(n: f64) -> f64 {
    unsafe { racos(n) }
}

pub fn asin(n: f64) -> f64 {
    unsafe { rasin(n) }
}

pub fn atan(n: f64) -> f64 {
    unsafe { ratan(n) }
}

pub fn atan2(a: f64, b: f64) -> f64 {
    unsafe { ratan2(a, b) }
}

pub fn cbrt(n: f64) -> f64 {
    unsafe { rcbrt(n) }
}

pub fn cosh(n: f64) -> f64 {
    unsafe { rcosh(n) }
}

pub fn expm1(n: f64) -> f64 {
    unsafe { rexpm1(n) }
}

pub fn hypot(x: f64, y: f64) -> f64 {
    unsafe { rhypot(x, y) }
}

pub fn log1p(n: f64) -> f64 {
    unsafe { rlog1p(n) }
}

pub fn sinh(n: f64) -> f64 {
    unsafe { rsinh(n) }
}

pub fn tan(n: f64) -> f64 {
    unsafe { rtan(n) }
}

pub fn tanh(n: f64) -> f64 {
    unsafe { rtanh(n) }
}

pub fn log(n: f64) -> f64 {
    unsafe { rlog(n) }
}

pub fn sqrt(n: f64) -> f64 {
    unsafe { rsqrt(n) }
}
