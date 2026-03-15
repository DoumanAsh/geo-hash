//Simplified f64 version from
//https://github.com/rust-lang/compiler-builtins/blob/8173070e590c04a0d36ee9f3e0397c6006596976/libm/src/math/generic/scalbn.rs#L3

const BITS: u32 = 64;
const SIGNIFICAND_BITS: u32 = 52;
const EXP_BITS: u32 = BITS - SIGNIFICAND_BITS - 1;
const EXP_SAT: u32 = (1 << EXP_BITS) - 1;
const EXP_BIAS: u32 = EXP_SAT >> 1;
const SIG_MASK: u64 = (1 << SIGNIFICAND_BITS) - 1;
const EXP_MAX: i32 = EXP_BIAS as i32;
const EXP_MIN: i32 = -(EXP_MAX - 1);


const fn f64_from_parts(exponent: u32, significand: u64) -> f64 {
    f64::from_bits(
        (0 << (BITS - 1))
        | (((exponent & EXP_SAT) as u64) << SIGNIFICAND_BITS)
        | significand & SIG_MASK
    )
}

pub const fn ldexp(mut x: f64, mut n: i32) -> f64 {
    // Bits including the implicit bit
    let sig_total_bits = SIGNIFICAND_BITS + 1;

    // Maximum and minimum values when biased
    let exp_max = EXP_MAX;
    let exp_min = EXP_MIN;

    // 2 ^ Emax, maximum positive with null significand (0x1p1023 for f64)
    let f_exp_max = f64_from_parts(EXP_BIAS << 1, 0);

    // 2 ^ Emin, minimum positive normal with null significand (0x1p-1022 for f64)
    let f_exp_min = f64_from_parts(1, 0);

    // 2 ^ sig_total_bits, moltiplier to normalize subnormals (0x1p53 for f64)
    let f_pow_subnorm = f64_from_parts(sig_total_bits + EXP_BIAS, 0);

    /*
     * The goal is to multiply `x` by a scale factor that applies `n`. However, there are cases
     * where `2^n` is not representable by `F` but the result should be, e.g. `x = 2^Emin` with
     * `n = -EMin + 2` (one out of range of 2^Emax). To get around this, reduce the magnitude of
     * the final scale operation by prescaling by the max/min power representable by `F`.
     */

    if n > exp_max {
        // Worse case positive `n`: `x`  is the minimum subnormal value, the result is `MAX`.
        // This can be reached by three scaling multiplications (two here and one final).
        debug_assert!(-exp_min + SIGNIFICAND_BITS as i32 + exp_max <= exp_max * 3);

        x *= f_exp_max;
        n -= exp_max;
        if n > exp_max {
            x *= f_exp_max;
            n -= exp_max;
            if n > exp_max {
                n = exp_max;
            }
        }
    } else if n < exp_min {
        // `mul` s.t. `!(x * mul).is_subnormal() ∀ x`
        let mul = f_exp_min * f_pow_subnorm;
        let add = -exp_min - sig_total_bits as i32;

        // Worse case negative `n`: `x`  is the maximum positive value, the result is `MIN`.
        // This must be reachable by three scaling multiplications (two here and one final).
        debug_assert!(-exp_min + SIGNIFICAND_BITS as i32 + exp_max <= add * 2 + -exp_min);

        x *= mul;
        n += add;

        if n < exp_min {
            x *= mul;
            n += add;

            if n < exp_min {
                n = exp_min;
            }
        }
    }

    let scale = f64_from_parts((EXP_BIAS as i32 + n) as u32, 0);
    x * scale
}

#[cfg(test)]
mod tests {
    use crate::{MAX_LAT, MAX_LON};
    use super::{EXP_SAT, EXP_MAX, EXP_MIN};

    #[test]
    fn should_verify_ldexp() {
        assert_eq!(EXP_SAT, 0b11111111111);
        assert_eq!(EXP_MAX, 1023);
        assert_eq!(EXP_MIN, -1022);

        let bits_len = 9 * 5;

        let lat_bits = bits_len / 2;
        let lon_bits = bits_len - lat_bits;

        let latitude = crate::math::ldexp(MAX_LAT * 2.0, -lat_bits);
        let longitude = crate::math::ldexp(MAX_LON * 2.0, -(lon_bits as i32));
        assert_eq!(4.291534423828125e-5, latitude);
        assert_eq!(4.291534423828125e-5, longitude);

        let bits_len = 4 * 5;

        let lat_bits = bits_len / 2;
        let lon_bits = bits_len - lat_bits;

        let latitude = crate::math::ldexp(MAX_LAT * 2.0, -lat_bits);
        let longitude = crate::math::ldexp(MAX_LON * 2.0, -(lon_bits as i32));
        assert_eq!(0.17578125, latitude);
        assert_eq!(0.3515625, longitude);
    }
}
