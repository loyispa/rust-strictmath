mod strict_math;

use strict_math::*;

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

pub fn cos(n: f64) -> f64 {
    unsafe { rcos(n) }
}

pub fn cosh(n: f64) -> f64 {
    unsafe { rcosh(n) }
}

pub fn exp(n: f64) -> f64 {
    unsafe { rexp(n) }
}

pub fn expm1(n: f64) -> f64 {
    unsafe { rexpm1(n) }
}

pub fn hypot(x: f64, y: f64) -> f64 {
    unsafe { rhypot(x, y) }
}

pub fn log(n: f64) -> f64 {
    unsafe { rlog(n) }
}

pub fn log10(n: f64) -> f64 {
    unsafe { rlog10(n) }
}

pub fn log1p(n: f64) -> f64 {
    unsafe { rlog1p(n) }
}

pub fn pow(m: f64, n: f64) -> f64 {
    unsafe { rpow(m, n) }
}

pub fn sin(n: f64) -> f64 {
    unsafe { rsin(n) }
}

pub fn sinh(n: f64) -> f64 {
    unsafe { rsinh(n) }
}

pub fn sqrt(n: f64) -> f64 {
    unsafe { rsqrt(n) }
}

pub fn tan(n: f64) -> f64 {
    unsafe { rtan(n) }
}

pub fn tanh(n: f64) -> f64 {
    unsafe { rtanh(n) }
}
