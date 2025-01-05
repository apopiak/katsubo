use std::{str::FromStr, u32};

use num_bigint::BigInt;
use num_bigint::BigUint;

fn native_mult(num1: BigInt, num2: BigInt) -> BigInt {
    num1 * num2
}

fn to_u64(num: &BigInt) -> u64 {
    debug_assert!(num <= &BigInt::from(u32::MAX));
    u64::from(num.iter_u32_digits().next().unwrap())
}

fn naive_mult(num1: BigInt, num2: BigInt) -> BigInt {
    let small = BigInt::from(u32::MAX);
    // shortcut to hardware multiplication if numbers are small
    if num1 < small && num2 < small {
        return BigInt::from_biguint(
            num1.sign() * num2.sign(),
            BigUint::from(to_u64(&num1) * to_u64(&num2)),
        );
    }
    let mid = (num1.bits()).max(num2.bits()) / 2;

    let (a, b): (BigInt, BigInt) = (num1.clone() >> mid, num1 & ((BigInt::from(1) << mid) - 1));
    let (c, d): (BigInt, BigInt) = (num2.clone() >> mid, num2 & ((BigInt::from(1) << mid) - 1));

    let ac = naive_mult(a.clone(), c.clone());
    let bd = naive_mult(b.clone(), d.clone());
    let ad = naive_mult(a, d);
    let bc = naive_mult(b, c);
    let ad_plus_bc = ad + bc;

    (ac << (mid << 1)) + ((ad_plus_bc << mid) + bd)
}

fn katsubo_mult(num1: BigInt, num2: BigInt) -> BigInt {
    let small = BigInt::from(u32::MAX);
    // shortcut to hardware multiplication if numbers are small
    if num1 < small && num2 < small {
        return BigInt::from_biguint(
            num1.sign() * num2.sign(),
            BigUint::from(to_u64(&num1) * to_u64(&num2)),
        );
    }
    let mid = (num1.bits()).max(num2.bits()) / 2;

    let (a, b): (BigInt, BigInt) = (num1.clone() >> mid, num1 & ((BigInt::from(1) << mid) - 1));
    let (c, d): (BigInt, BigInt) = (num2.clone() >> mid, num2 & ((BigInt::from(1) << mid) - 1));

    let abcd = katsubo_mult(&a + &b, &c + &d);
    let ac = katsubo_mult(a, c);
    let bd = katsubo_mult(b, d);
    let abcd_minus_ac_minus_bd = abcd - &ac - &bd;

    (ac << (mid << 1)) + ((abcd_minus_ac_minus_bd << mid) + bd)
}

fn measured_run<F, R>(name: &str, f: F) -> R
where
    F: Fn() -> R,
{
    let start = std::time::Instant::now();
    let r = f();
    println!("{name}: Elapsed: {:.2?}", start.elapsed());
    r
}

fn main() {
    let num1 = BigInt::from_str("123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789").unwrap();
    let num2 = BigInt::from_str("987654321987654321987654321987654321987654321987654321987654321987654321987654321987654321987654321987654321987654321987654321987654321987654321987654321987654321987654321987654321987654321987654321987654321987654321987654321987654321987654321987654321987654321987654321987654321987654321").unwrap();

    let native = measured_run("Native ", || native_mult(num1.clone(), num2.clone()));
    let naive = measured_run("Naive  ", || naive_mult(num1.clone(), num2.clone()));
    let katsubo = measured_run("Katsubo", || katsubo_mult(num1.clone(), num2.clone()));
    assert_eq!(native, naive);
    assert_eq!(native, katsubo);
    println!("Success!");
    println!("{}\n * \n{}\n = \n{}", num1, num2, katsubo);
}
