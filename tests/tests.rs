extern crate flint;
extern crate num;
extern crate libc;

use libc::c_ulong;
use num::FromPrimitive;
use num::bigint::BigInt;
use flint::fmpz::{Fmpz, FmpzFactor};

#[test]
fn add_test() {
    let mut res = Fmpz::new();
    let mut res1 = Fmpz::new();
    let mut a = Fmpz::new();
    let mut b = Fmpz::new();
    for i in 0..100 {
        let x: BigInt = FromPrimitive::from_u64(i).unwrap();
        a.set_ui(i);
        for j in 0..100 {
            let y: BigInt = FromPrimitive::from_u64(j).unwrap();
            b.set_ui(j);
            let z = &x + &y;
            res.add_mut(&a, &b);
            res1.add_ui_mut(&a, j);
            assert!(z.to_str_radix(10) == res.get_str(10));
            assert!(z.to_str_radix(10) == res1.get_str(10));
        }
    }
}

fn fac_to_fmpz(f: &FmpzFactor) -> Fmpz {
    let mut res = Fmpz::from_si(1);
    let mut tmp = Fmpz::new();
    for &(ref a, e) in f.to_vec().iter() {
        tmp.pow_ui_mut(&a, e as c_ulong);
        res *= &tmp;
    }
    res
}

#[test]
fn factor_test() {
    let mut a = Fmpz::from_str("1844674407370955161", 10).unwrap();
    let b = Fmpz::from_si(340394);
    a -= &b;
    // println!("res1={}", res);
    // res.set_mul_ui(&a, 10);
    // println!("res2={}", res);
    // res.pow_ui(&a, 12);
    // println!("{}", res);
    let fac = a.to_factor();
    assert!(fac_to_fmpz(&fac).get_str(10) == a.get_str(10));
}