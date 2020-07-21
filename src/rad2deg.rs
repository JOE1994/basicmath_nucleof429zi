/*
**  RAD2DEG.RS - Functions to convert between radians and degrees
*/

use core::f64::consts::PI;

pub(crate) fn rad2deg(rad: f64) -> f64 {
    180.0 * rad / (PI)
}

pub(crate) fn deg2rad(deg: f64) -> f64 {
    PI * deg / 180.0
}

/*
#[cfg(test)]
mod test {
    use super::{rad2deg, deg2rad, PI};

    #[test]
    fn rad2deg_test() {
        #[allow(non_snake_case)]
        let mut X: f64;

        X = 0.0;
        while X <= 360.0 {
            println!("{:3.0} degrees = {:.12} radians", X, deg2rad(X));
            X += 45.0;
        }

        println!();

        X = 0.0;
        while X <= (2.* PI + 1e-6) {
            println!("{:.12} radians = {:3.0} degrees", X, rad2deg(X));
            X += PI / 6.0;
        }
    }
}
*/
