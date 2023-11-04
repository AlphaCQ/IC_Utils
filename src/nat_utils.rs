use candid::Nat;
use num_bigint::BigUint;

pub fn fstr_to_nat(v: String, decimal:f64) ->Nat{
    let l:f64= v.parse::<f64>().unwrap()*decimal;
    Nat::from(l as u64)
}
pub fn nat_from(v: u64) -> Nat {
    Nat::from(v)
}

pub fn new_zero() -> Nat {
    nat_from(0)
}

pub fn nat_to_u64(value: Nat) -> u64 {
    let v: u64 = BigUint::from(value).try_into().unwrap();
    v
}

pub fn nat_8() -> Nat {
    Nat::from(100_000_000u64)
}

pub fn nat_12() -> Nat {
    Nat::from(1_000_000_000_000u64)
}

pub fn nat_15() -> Nat {
    Nat::from(1_000_000_000_000_000u64)
}

pub fn nat_18() -> Nat {
    Nat::from(1_000_000_000_000_000_000u64)
}

pub fn nat_36() -> Nat {
    Nat::from(1_000_000_000_000_000_000_000_000_000_000_000_000u128)
}
