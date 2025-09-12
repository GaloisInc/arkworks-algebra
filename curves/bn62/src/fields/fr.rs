use ark_ff::fields::{Fp64, Fp128, MontBackend, MontConfig};

// #[derive(MontConfig)]
// #[modulus = "6094503333997212553"]
// #[generator = "5"]
// pub struct FrConfig;
// pub type Fr = Fp64<MontBackend<FrConfig, 1>>;

#[derive(MontConfig)]
#[modulus = "3866540040962951063"]
#[generator = "3866540040962951061"]
pub struct FrConfig;
pub type Fr = Fp64<MontBackend<FrConfig, 1>>;

