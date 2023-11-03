mod strict_math;

pub use strict_math::acos;
pub use strict_math::asin;
pub use strict_math::atan;
pub use strict_math::atan2;
pub use strict_math::cbrt;
pub use strict_math::cosh;
pub use strict_math::expm1;
pub use strict_math::hypot;
pub use strict_math::log;
pub use strict_math::log1p;
pub use strict_math::sinh;
pub use strict_math::sqrt;
pub use strict_math::tan;
pub use strict_math::tanh;

mod test {
    use super::*;

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
            sinh(0.055605003447049994_f64),
            0.055605003447049994_f64.sinh()
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
        assert_eq!(cbrt(-0.055605003447049994_f64), -0.3816845880251514);
    }
}
