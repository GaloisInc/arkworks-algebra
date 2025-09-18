use ark_ff::fields::{Fp128, Fp64, MontBackend, MontConfig};
use ark_ff::{biginteger::arithmetic as fa, fields::Fp, fields::*, BigInt, BigInteger};
pub struct FrConfig;
type B = BigInt<1usize>;
type F = Fp<MontBackend<FrConfig, 1usize>, 1usize>;
impl MontConfig<1usize> for FrConfig {
    const MODULUS: B = BigInt([3866540040962951063u64]);
    const GENERATOR: F = {
        let (is_positive, limbs) = (true, [3866540040962951061u64]);
        ::ark_ff::Fp::from_sign_and_limbs(is_positive, &limbs)
    };
    const TWO_ADIC_ROOT_OF_UNITY: F = {
        let (is_positive, limbs) = (true, [3866540040962951062u64]);
        ::ark_ff::Fp::from_sign_and_limbs(is_positive, &limbs)
    };
    #[inline(always)]
    fn add_assign(a: &mut F, b: &F) {
        __add_with_carry(&mut a.0, &b.0);
        __subtract_modulus(a);
    }
    #[inline(always)]
    fn sub_assign(a: &mut F, b: &F) {
        if b.0 > a.0 {
            __add_with_carry(&mut a.0, &BigInt([3866540040962951063u64]));
        }
        __sub_with_borrow(&mut a.0, &b.0);
    }
    #[inline(always)]
    fn double_in_place(a: &mut F) {
        a.0.mul2();
        __subtract_modulus(a);
    }
    /// Sets `a = -a`.
    #[inline(always)]
    fn neg_in_place(a: &mut F) {
        if *a != F::ZERO {
            let mut tmp = BigInt([3866540040962951063u64]);
            __sub_with_borrow(&mut tmp, &a.0);
            a.0 = tmp;
        }
    }
    #[inline(always)]
    fn mul_assign(a: &mut F, b: &F) {
        {
            let mut r = [0u64; 1usize];
            let mut carry1 = 0u64;
            r[0] = fa::mac(r[0], (a.0).0[0], (b.0).0[0usize], &mut carry1);
            let k = r[0].wrapping_mul(Self::INV);
            let mut carry2 = 0u64;
            fa::mac_discard(r[0], k, 3866540040962951063u64, &mut carry2);
            r[1usize - 1] = carry1 + carry2;
            (a.0).0 = r;
        }
        __subtract_modulus(a);
    }
    #[inline(always)]
    fn square_in_place(a: &mut F) {
        {
            *a *= *a;
        }
    }
    fn sum_of_products<const M: usize>(a: &[F; M], b: &[F; M]) -> F {
        if M <= 3usize {
            let result = (0..1usize).fold(BigInt::zero(), |mut result, j| {
                let mut carry_a = 0;
                let mut carry_b = 0;
                for (a, b) in a.iter().zip(b) {
                    let a = &a.0;
                    let b = &b.0;
                    let mut carry2 = 0;
                    result.0[0] = fa::mac(result.0[0], a.0[j], b.0[0], &mut carry2);
                    carry_b = fa::adc(&mut carry_a, carry_b, carry2);
                }
                let k = result.0[0].wrapping_mul(Self::INV);
                let mut carry2 = 0;
                fa::mac_discard(result.0[0], k, 3866540040962951063u64, &mut carry2);
                result.0[1usize - 1] = fa::adc_no_carry(carry_a, carry_b, &mut carry2);
                result
            });
            let mut result = F::new_unchecked(result);
            __subtract_modulus(&mut result);
            if true {
                match (&a.iter().zip(b).map(|(a, b)| *a * b).sum::<F>(), &result) {
                    (left_val, right_val) => {
                        if !(*left_val == *right_val) {
                            unimplemented!();
                            /*
                            let kind = ::core::panicking::AssertKind::Eq;
                            ::core::panicking::assert_failed(
                                kind,
                                &*left_val,
                                &*right_val,
                                ::core::option::Option::None,
                            );
                            */
                        }
                    },
                };
            }
            result
        } else {
            a.chunks(3usize)
                .zip(b.chunks(3usize))
                .map(|(a, b)| {
                    if a.len() == 3usize {
                        Self::sum_of_products::<3usize>(
                            a.try_into().unwrap(),
                            b.try_into().unwrap(),
                        )
                    } else {
                        a.iter().zip(b).map(|(a, b)| *a * b).sum()
                    }
                })
                .sum()
        }
    }
}
#[inline(always)]
fn __subtract_modulus(a: &mut F) {
    if a.is_geq_modulus() {
        __sub_with_borrow(&mut a.0, &BigInt([3866540040962951063u64]));
    }
}
#[inline(always)]
fn __add_with_carry(a: &mut B, b: &B) {
    // use ark_ff::biginteger::arithmetic::adc_for_add_with_carry as adc;
    // let mut carry = 0;
    // adc(&mut a.0[0usize], b.0[0usize], carry);

    #[cfg(all(target_arch = "x86_64", feature = "asm"))]
    #[allow(unsafe_code)]
    unsafe {
        use core::arch::x86_64::_addcarry_u64;
        _addcarry_u64(carry, a.0[0usize], b.0[0usize], a.0[0usize]);
    }
    #[cfg(not(all(target_arch = "x86_64", feature = "asm")))]
    {
        let tmp = (a.0[0usize]) as u128 + (b.0[0usize]) as u128;
        a.0[0usize] = tmp as u64;
    }
}

#[inline(always)]
fn __sub_with_borrow(a: &mut B, b: &B) {
    // use ark_ff::biginteger::arithmetic::sbb_for_sub_with_borrow as sbb;
    // let mut borrow = 0;
    // borrow = sbb(&mut a.0[0usize], b.0[0usize], borrow);


    #[cfg(all(target_arch = "x86_64", feature = "asm"))]
    #[allow(unsafe_code)]
    unsafe {
        use core::arch::x86_64::_subborrow_u64;
        _subborrow_u64(borrow, a.0[0usize], b.0[0usize], a.0[0usize]);
    }
    #[cfg(not(all(target_arch = "x86_64", feature = "asm")))]
    {
        let tmp = (1u128 << 64) + ((a.0[0usize]) as u128) - ((b.0[0usize]) as u128);
        (a.0[0usize]) = tmp as u64;
    }
}

pub type Fr = Fp64<MontBackend<FrConfig, 1>>;
