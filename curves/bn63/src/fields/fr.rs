use ark_ff::fields::{Fp64, Fp128, MontBackend, MontConfig};

// #[derive(MontConfig)]
// #[modulus = "6094503333997212553"]
// #[generator = "5"]
// pub struct FrConfig;
// pub type Fr = Fp64<MontBackend<FrConfig, 1>>;

#[derive(MontConfig)]
#[modulus = "170141183460469231731687303715884105727"]
#[generator = "43"]
pub struct FrConfig;
pub type Fr = Fp128<MontBackend<FrConfig, 2>>;

