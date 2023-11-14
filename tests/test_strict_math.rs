use rust_strictmath::*;

macro_rules! assert_approx_eq {
    ($a:expr, $b:expr) => {{
        let (a, b) = (&$a, &$b);
        assert!(
            (*a - *b).abs() < 1.0e-6,
            "{} is not approximately equal to {}",
            *a,
            *b
        );
    }};
}

#[test]
pub fn test() {
    assert_approx_eq!(
        acos(0.055605003447049994_f64),
        0.055605003447049994_f64.acos()
    );
    assert_approx_eq!(
        asin(0.055605003447049994_f64),
        0.055605003447049994_f64.asin()
    );
    assert_approx_eq!(
        atan(0.055605003447049994_f64),
        0.055605003447049994_f64.atan()
    );
    assert_approx_eq!(atan2(3.0_f64, -3.0_f64), 3.0_f64.atan2(-3.0_f64));
    assert_approx_eq!(
        cosh(0.055605003447049994_f64),
        0.055605003447049994_f64.cosh()
    );
    assert_approx_eq!(
        expm1(0.055605003447049994_f64),
        0.055605003447049994_f64.exp_m1()
    );
    assert_approx_eq!(hypot(2.0_f64, 3.0_f64), 2.0_f64.hypot(3.0_f64));
    assert_approx_eq!(
        log1p(0.055605003447049994_f64),
        0.055605003447049994_f64.ln_1p()
    );
    assert_approx_eq!(
        tan(0.055605003447049994_f64),
        0.055605003447049994_f64.tan()
    );
    assert_approx_eq!(
        tanh(0.055605003447049994_f64),
        0.055605003447049994_f64.tanh()
    );
    assert_approx_eq!(log(0.055605003447049994_f64), 0.055605003447049994_f64.ln());
    assert_approx_eq!(
        sqrt(0.055605003447049994_f64),
        0.055605003447049994_f64.sqrt()
    );
    assert_approx_eq!(
        cbrt(0.055605003447049994_f64),
        0.055605003447049994_f64.cbrt()
    );
    assert_approx_eq!(
        sinh(0.055605003447049994_f64),
        0.055605003447049994_f64.sinh()
    );
    assert_approx_eq!(
        cos(0.055605003447049994_f64),
        0.055605003447049994_f64.cos()
    );
    assert_approx_eq!(
        sin(0.055605003447049994_f64),
        0.055605003447049994_f64.sin()
    );
    assert_approx_eq!(
        exp(0.055605003447049994_f64),
        0.055605003447049994_f64.exp()
    );
    assert_approx_eq!(
        log10(0.055605003447049994_f64),
        0.055605003447049994_f64.log10()
    );
    assert_approx_eq!(
        pow(0.055605003447049994_f64, 0.055605003447049994_f64),
        0.055605003447049994_f64.powf(0.055605003447049994_f64)
    );
    assert_eq!(cbrt(-0.055605003447049994_f64), -0.3816845880251514);
}
