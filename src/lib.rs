mod strict_math;

pub use strict_math::racos;
pub use strict_math::rasin;
pub use strict_math::ratan;
pub use strict_math::ratan2;
pub use strict_math::rcbrt;
pub use strict_math::rcosh;
pub use strict_math::rexpm1;
pub use strict_math::rhypot;
pub use strict_math::rlog1p;
pub use strict_math::rsinh;
pub use strict_math::rtan;
pub use strict_math::rtanh;

mod test {
    use super::*;

    #[test]
    pub fn test() {
        unsafe {
            assert_eq!(
                racos(0.055605003447049994_f64),
                0.055605003447049994_f64.acos()
            );
            assert_eq!(
                rasin(0.055605003447049994_f64),
                0.055605003447049994_f64.asin()
            );
            assert_eq!(
                ratan(0.055605003447049994_f64),
                0.055605003447049994_f64.atan()
            );
            assert_eq!(ratan2(3.0_f64, -3.0_f64), 3.0_f64.atan2(-3.0_f64));
            assert_eq!(
                rcbrt(0.055605003447049994_f64),
                0.055605003447049994_f64.cbrt()
            );
            assert_eq!(
                rcosh(0.055605003447049994_f64),
                0.055605003447049994_f64.cosh()
            );
            assert_eq!(
                rexpm1(0.055605003447049994_f64),
                0.055605003447049994_f64.exp_m1()
            );
            assert_eq!(rhypot(2.0_f64, 3.0_f64), 2.0_f64.hypot(3.0_f64));
            assert_eq!(
                rlog1p(0.055605003447049994_f64),
                0.055605003447049994_f64.ln_1p()
            );
            assert_eq!(
                rsinh(0.055605003447049994_f64),
                0.055605003447049994_f64.sinh()
            );
            assert_eq!(
                rtan(0.055605003447049994_f64),
                0.055605003447049994_f64.tan()
            );
            assert_eq!(
                rtanh(0.055605003447049994_f64),
                0.055605003447049994_f64.tanh()
            );
            assert_eq!(rcbrt(-0.055605003447049994_f64), -0.3816845880251514);
        }
    }
}
