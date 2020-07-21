#![no_main]
#![no_std]
#![feature(core_intrinsics)]
// basicmath_large.rs

use panic_halt as _;
use stm32f4xx_hal as hal;
use crate::hal::{
    prelude::*,
    stm32,
    // serial::{config::Config, Serial},
    // led::{LedColor, Leds},
};
use cortex_m::peripheral::ITM;
use cortex_m::{iprint, iprintln};

mod cubic;
mod isqrt;
mod rad2deg;

use cortex_m_rt::entry;
use {
    core::f64::consts::PI,
    core::fmt::Write,

    cubic::SolveCubic,
    isqrt::{int_sqrt, usqrt},
    // stm32f4xx_hal::stm32::USART2,
    // stm32f4xx_hal::serial::Tx,
    nb::block,
    rad2deg::{deg2rad, rad2deg},
};

#[entry]
fn main() -> ! {
    if let (Some(p), Some(cp)) = (
        stm32::Peripherals::take(),
        cortex_m::Peripherals::take()
    ) {
        let gpioa = p.GPIOA.split();
        let rcc = p.RCC.constrain();
        let clocks = rcc.cfgr.sysclk(168.mhz()).freeze();
        let mut itm = cp.ITM;
        let stim = &mut itm.stim[0];

        cortex_m::asm::bkpt(); // Begin running benchmark!

        let (mut a1, mut b1, mut c1, mut d1) = (1.0f64, -10.5f64, 32.0f64, -30.0f64);
        let mut x: [f64; 3] = [0.; 3];
        #[allow(non_snake_case)]
        let mut X: f64;

        let mut solutions: i32 = 0;
        // let _l: u64 = 0x3fed0169;

        let mut q: int_sqrt = Default::default();
        // let _n: i32 = 0;

        /* solve soem cubic functions */
        iprintln!(stim, "********* CUBIC FUNCTIONS ***********");
        
        /* should get 3 solutions: 2, 6 & 2.5   */
        SolveCubic(a1, b1, c1, d1, &mut solutions, &mut x);
        iprint!(stim, "Solutions:");
        for i in 0..(solutions as usize) {
            iprint!(stim, " {}", x[i]);
        }
        iprintln!(stim);

        a1 = 1.0;
        b1 = -4.5;
        c1 = 17.0;
        d1 = -30.0;
        /* should get 1 solution: 2.5           */
        SolveCubic(a1, b1, c1, d1, &mut solutions, &mut x);
        
        iprint!(stim, "Solutions:");
        for i in 0..(solutions as usize) {
            iprint!(stim, " {}", x[i]);
        }
        iprintln!(stim);

        a1 = 1.0;
        b1 = -3.5;
        c1 = 22.0;
        d1 = -31.0;
        SolveCubic(a1, b1, c1, d1, &mut solutions, &mut x);
        
        iprint!(stim, "Solutions:");
        for i in 0..(solutions as usize) {
            
            iprint!(stim, " {}", x[i]);
        }
        
        iprintln!(stim);

        a1 = 1.0;
        b1 = -13.7;
        c1 = 1.0;
        d1 = -35.0;
        SolveCubic(a1, b1, c1, d1, &mut solutions, &mut x);
        
        iprint!(stim, "Solutions:");
        for i in 0..(solutions as usize) {
            
            iprint!(stim, " {}", x[i]);
        }
        
        iprintln!(stim);

        a1 = 3.0;
        b1 = 12.34;
        c1 = 5.0;
        d1 = 12.0;
        SolveCubic(a1, b1, c1, d1, &mut solutions, &mut x);
        
        iprint!(stim, "Solutions:");
        for i in 0..(solutions as usize) {
            
            iprint!(stim, " {}", x[i]);
        }
        
        iprintln!(stim);

        a1 = -8.0;
        b1 = -67.89;
        c1 = 6.0;
        d1 = -23.6;
        SolveCubic(a1, b1, c1, d1, &mut solutions, &mut x);
        
        iprint!(stim, "Solutions:");
        for i in 0..(solutions as usize) {
            
            iprint!(stim, " {}", x[i]);
        }
        
        iprintln!(stim);

        a1 = 45.0;
        b1 = 8.67;
        c1 = 7.5;
        d1 = 34.0;
        SolveCubic(a1, b1, c1, d1, &mut solutions, &mut x);
        
        iprint!(stim, "Solutions:");
        for i in 0..(solutions as usize) {
            
            iprint!(stim, " {}", x[i]);
        }
        
        iprintln!(stim);

        a1 = -12.0;
        b1 = -1.7;
        c1 = 5.3;
        d1 = 16.0;
        SolveCubic(a1, b1, c1, d1, &mut solutions, &mut x);
        // tx.write_str("Solutions:").ok();
        iprint!(stim, "Solutions:");
        for i in 0..(solutions as usize) {
            
            iprint!(stim, " {}", x[i]);
        }
        // block!(tx.write(b'\n')).ok();
        iprintln!(stim);

        /* Now solve some random equations */
        a1 = 1.0;
        while a1 < 10.0 {
            b1 = 10.0;
            while b1 > 0.0 {
                c1 = 5.0;
                while c1 < 15.0 {
                    d1 = -1.0;
                    while d1 > -5.0 {
                        SolveCubic(a1, b1, c1, d1, &mut solutions, &mut x);
                        
                        iprint!(stim, "Solutions:");
                        for i in 0..(solutions as usize) {
                            
                            iprint!(stim, " {}", x[i]);
                        }
                        
                        iprintln!(stim);
                        d1 -= 0.451;
                    }
                    c1 += 0.61;
                }
                b1 -= 0.25;
            }
            a1 += 1.0;
        }

        // tx.write_str("********* INTEGER SQR ROOTS ***********\n").ok();
        /* perform some integer square roots */
        for i in (0..100000).step_by(2) {
            usqrt(i, &mut q);
            
            iprintln!(stim, "sqrt({:3}) = {:2}", i, q.sqrt);
        }
        // block!(tx.write(b'\n')).ok();
        iprintln!(stim);
        for l in 0x3fed0169..0x3fed4169 {
            usqrt(l, &mut q);

            iprintln!(stim, "sqrt({:X}) = {:X}", l, q.sqrt);
        }

        iprintln!(stim, "********* ANGLE CONVERSION ***********");
        /* convert some rads to degrees */
        /*   for (X = 0.0; X <= 360.0; X += 1.0) */
        X = 0.;
        while X <= 360. {
            iprintln!(stim, "{:3.0} degrees = {:.12} radians", X, deg2rad(X));
            X += 0.001;
        }

        iprintln!(stim);

        /*   for (X = 0.0; X <= (2 * PI + 1e-6); X += (PI / 180)) */
        X = 0.0;
        while X <= (2. * PI + 1e-6) {

            iprintln!(stim, "{:.12} radians = {:3.0} degrees", X, rad2deg(X));
            X += PI / 5760.
        }
    }

    cortex_m::asm::bkpt(); // Finished running benchmark!
    loop {
        continue;
    }
}
