const BITSPERLONG: u32 = 32;

// #define TOP2BITS(x) ((x & (3L << (BITSPERLONG-2))) >> (BITSPERLONG-2))
macro_rules! TOP2BITS {
    ($x:expr) => {
        ($x & (3u32 << (BITSPERLONG - 2))) >> (BITSPERLONG - 2)
    };
}

#[allow(non_camel_case_types)]
#[derive(Default)]
pub(crate) struct int_sqrt {
    pub(crate) sqrt: u32,
    pub(crate) frac: u32,
}

/* usqrt:
    ENTRY x: unsigned long
    EXIT  returns floor(sqrt(x) * pow(2, BITSPERLONG/2))
    Since the square root never uses more than half the bits
    of the input, we use the other half of the bits to contain
    extra bits of precision after the binary point.
    EXAMPLE
        suppose BITSPERLONG = 32
        then    usqrt(144) = 786432 = 12 * 65536
                usqrt(32) = 370727 = 5.66 * 65536
    NOTES
        (1) change BITSPERLONG to BITSPERLONG/2 if you do not want
            the answer scaled.  Indeed, if you want n bits of
            precision after the binary point, use BITSPERLONG/2+n.
            The code assumes that BITSPERLONG is even.
        (2) This is really better off being written in assembly.
            The line marked below is really a "arithmetic shift left"
            on the double-long value with r in the upper half
            and x in the lower half.  This operation is typically
            expressible in only one or two assembly instructions.
        (3) Unrolling this loop is probably not a bad idea.
    ALGORITHM
        The calculations are the base-two analogue of the square
        root algorithm we all learned in grammar school.  Since we're
        in base 2, there is only one nontrivial trial multiplier.
        Notice that absolutely no multiplications or divisions are performed.
        This means it'll be fast on a wide range of processors.
*/

pub(crate) fn usqrt(mut x: u32, q: &mut int_sqrt) {
    let mut a: u32 = 0; /* accumulator      */
    let mut r: u32 = 0; /* remainder        */
    let mut e: u32; /* trial product    */

    for _ in 0..BITSPERLONG {
        /* NOTE 1 */
        r = (r << 2) + TOP2BITS!(x);
        x <<= 2; /* NOTE 2 */
        a <<= 1;
        e = (a << 1) + 1;
        if r >= e {
            r -= e;
            a += 1;
        }
    }

    q.sqrt = a; // memcpy(q, &a, core::mem::size_of::<u32>());
}

/*
#[cfg(test)]
mod test {
    use super::{usqrt, int_sqrt};

    #[test]
    fn isqrt_test() {
        #[allow(non_upper_case_globals)]
        const l: u32 = 0x3fed0169;
        let mut q: int_sqrt = Default::default();

        for i in 0..101 {
            usqrt(i, &mut q);
            println!("sqrt({:3}) = {:3}, remainder = {:2}\n", i, q.sqrt, q.frac);
        }
        usqrt(l, &mut q);
        println!("\nsqrt({:X}) = {:X}, remainder = {:X}", l, q.sqrt, q.frac);
    }
}
*/
