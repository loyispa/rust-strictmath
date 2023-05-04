mod strict_math;

pub use strict_math::acos;
pub use strict_math::asin;
pub use strict_math::atan;
pub use strict_math::atan2;
pub use strict_math::cbrt;
pub use strict_math::cosh;
pub use strict_math::expm1;
pub use strict_math::hypot;
pub use strict_math::log1p;
pub use strict_math::sinh;
pub use strict_math::tan;
pub use strict_math::tanh;

mod test {
    use super::*;

    #[test]
    pub fn test() {
        assert_eq!(
            acos(0.055605003447049994_f64),
            0.055605003447049994_f64.acos()
        );
        assert_eq!(
            asin(0.055605003447049994_f64),
            0.055605003447049994_f64.asin()
        );
        assert_eq!(
            atan(0.055605003447049994_f64),
            0.055605003447049994_f64.atan()
        );
        assert_eq!(atan2(3.0_f64, -3.0_f64), 3.0_f64.atan2(-3.0_f64));
        assert_eq!(
            cosh(0.055605003447049994_f64),
            0.055605003447049994_f64.cosh()
        );
        assert_eq!(
            expm1(0.055605003447049994_f64),
            0.055605003447049994_f64.exp_m1()
        );
        assert_eq!(hypot(2.0_f64, 3.0_f64), 2.0_f64.hypot(3.0_f64));
        assert_eq!(
            log1p(0.055605003447049994_f64),
            0.055605003447049994_f64.ln_1p()
        );
        assert_eq!(
            sinh(0.055605003447049994_f64),
            0.055605003447049994_f64.sinh()
        );
        assert_eq!(
            tan(0.055605003447049994_f64),
            0.055605003447049994_f64.tan()
        );
        assert_eq!(
            tanh(0.055605003447049994_f64),
            0.055605003447049994_f64.tanh()
        );
        eprintln!("cbrt:: {}", -0.055605003447049994_f64.cbrt());
        assert_eq!(cbrt(-0.055605003447049994_f64), -0.3816845880251514);
    }
}
