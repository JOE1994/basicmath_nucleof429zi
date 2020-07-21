use core::f64::consts::PI;
use core::intrinsics::sqrtf64;
use core::intrinsics::cosf64;
use core::intrinsics::fabsf64;
use core::intrinsics::powf64;

/*
**  CUBIC.RS - Solve a cubic polynomial
**  public domain by Ross Cottrell
*/
#[allow(non_snake_case)]
pub(crate) fn SolveCubic(a: f64, b: f64, c: f64, d: f64, solutions: &mut i32, x: &mut [f64; 3]) {
    let a1 = b / a;
    let a2 = c / a;
    let a3 = d / a;
    #[allow(non_snake_case)]
    let Q = (a1 * a1 - 3.0 * a2) / 9.0f64;
    #[allow(non_snake_case)]
    let R: f64 = (2.0 * a1 * a1 * a1 - 9.0 * a1 * a2 + 27.0 * a3) / 54.0f64;
    #[allow(non_snake_case)]
    let R2_Q3: f64 = R * R - Q * Q * Q;
    let theta: f64;

    /*
    unsafe {
        if R2_Q3 <= 0. {
            *solutions = 3;
            theta = libm::acos(R / sqrtf64(Q * Q * Q));
            x[0] = -2.0 * sqrtf64(Q) * cosf64(theta / 3.0) - a1 / 3.0;
            x[1] = -2.0 * sqrtf64(Q) * cosf64((theta + 2.0 * PI) / 3.0) - a1 / 3.0;
            x[2] = -2.0 * sqrtf64(Q) * cosf64((theta + 4.0 * PI) / 3.0) - a1 / 3.0;
        } else {
            *solutions = 1;
            x[0] = sqrtf64(R2_Q3) + libm::pow(fabsf64(R), 1. / 3.);
            x[0] += Q / x[0];
            x[0] *= if R < 0. { 1. } else { -1. };
            x[0] -= a1 / 3.0;
        }
    }
    */
    
    if R2_Q3 <= 0. {
        *solutions = 3;
        theta = libm::acos(R / libm::sqrt(Q * Q * Q));
        x[0] = -2.0 * libm::sqrt(Q) * libm::cos(theta / 3.0) - a1 / 3.0;
        x[1] = -2.0 * libm::sqrt(Q) * libm::cos((theta + 2.0 * PI) / 3.0) - a1 / 3.0;
        x[2] = -2.0 * libm::sqrt(Q) * libm::cos((theta + 4.0 * PI) / 3.0) - a1 / 3.0;
    } else {
        *solutions = 1;
        x[0] = libm::sqrt(R2_Q3) + libm::pow(libm::fabs(R), 1. / 3.);
        x[0] += Q / x[0];
        x[0] *= if R < 0. { 1. } else { -1. };
        x[0] -= a1 / 3.0;
    }
    
}

/*
#[cfg(test)]
mod test {
    use super::SolveCubic;

    #[test]
    fn cubic_test() {
        #[allow(non_upper_case_globals)]
        const a1: f64 = 1.0;
        #[allow(non_upper_case_globals)]
        const b1: f64 = -10.5;
        #[allow(non_upper_case_globals)]
        const c1: f64 = 32.0;
        #[allow(non_upper_case_globals)]
        const d1: f64 = -30.0;

        #[allow(non_upper_case_globals)]
        const a2: f64 = 1.0;
        #[allow(non_upper_case_globals)]
        const b2: f64 = -4.5;
        #[allow(non_upper_case_globals)]
        const c2: f64 = 17.0;
        #[allow(non_upper_case_globals)]
        const d2: f64 = -30.0;

        let mut x: [f64; 3] = [0.0; 3];
        let mut solutions: i32 = 0;

        SolveCubic(a1, b1, c1, d1, &mut solutions, &mut x);

        /* should get 3 solutions: 2, 6 & 2.5 */
        assert_eq!(solutions, 3);
        // assert_eq!(x[0], 2.0);
        // assert_eq!(x[1], 6.0);
        // assert_eq!(x[2], 2.5);

        SolveCubic(a2, b2, c2, d2, &mut solutions, &mut x);

        /* should get 1 solution: 2.5 */
        assert_eq!(solutions, 1);
        // assert_eq!(x[0], 2.5);

    }
}
*/
