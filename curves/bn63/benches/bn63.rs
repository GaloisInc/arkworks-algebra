use ark_algebra_bench_templates::*;
use ark_bn63::{fq::Fq, fq2::Fq2, fr::Fr, Bn63, Fq12, G1Projective as G1, G2Projective as G2};

bench!(
    Name = "BN63",
    Pairing = Bn63,
    G1 = G1,
    G2 = G2,
    ScalarField = Fr,
    G1BaseField = Fq,
    G2BaseField = Fq2,
    TargetField = Fq12,
);
