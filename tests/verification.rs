use casper_types::U256;
use groth16_verifier_casper::{read_proof, read_vk};

#[test]
fn test_verify_arkworks() {
    use ark_bn254::{self, Fq2, G1Affine, G2Affine};
    use num_bigint::BigUint;
    use std::str::FromStr;

    type G1 = G1Affine;
    type G2 = G2Affine;

    fn parse_biguint_to_fq2(value1: &str, value2: &str) -> Fq2 {
        let fq1 = parse_biguint_to_fq(value1);
        let fq2 = parse_biguint_to_fq(value2);
        Fq2::new(fq1, fq2)
    }
    fn points_to_g1(x: U256, y: U256) -> G1 {
        G1::new_unchecked(
            parse_biguint_to_fq(&x.to_string()),
            parse_biguint_to_fq(&y.to_string()),
        )
    }
    fn points_to_g2(x1: U256, x2: U256, y1: U256, y2: U256) -> G2 {
        G2::new_unchecked(
            parse_biguint_to_fq2(&x1.to_string(), &x2.to_string()),
            parse_biguint_to_fq2(&y1.to_string(), &y2.to_string()),
        )
    }

    let vk = read_vk();
    let proof = read_proof();
    use casper_groth16::{parse_biguint_to_fq, verify_groth16_proof};
    let pi_a = points_to_g1(proof.pi_a[0], proof.pi_a[1]);
    let pi_b = points_to_g2(
        proof.pi_b[0][0],
        proof.pi_b[0][1],
        proof.pi_b[1][0],
        proof.pi_b[1][1],
    );
    let pi_c = points_to_g1(proof.pi_c[0], proof.pi_c[1]);

    let vk_alpha1 = points_to_g1(vk.vk_alpha_1[0], vk.vk_alpha_1[1]);

    let vk_beta2 = points_to_g2(
        vk.vk_beta_2[0][0],
        vk.vk_beta_2[0][1],
        vk.vk_beta_2[1][0],
        vk.vk_beta_2[1][1],
    );

    let vk_gamma2 = points_to_g2(
        vk.vk_gamma_2[0][0],
        vk.vk_gamma_2[0][1],
        vk.vk_gamma_2[1][0],
        vk.vk_gamma_2[1][1],
    );
    let vk_delta2 = points_to_g2(
        vk.vk_delta_2[0][0],
        vk.vk_delta_2[0][1],
        vk.vk_delta_2[1][0],
        vk.vk_delta_2[1][1],
    );
    let inputs = vec![
        BigUint::from_str("33").unwrap(),
        BigUint::from_str("3").unwrap(),
        BigUint::from_str("5").unwrap(),
    ];
    let mut ics: Vec<G1> = Vec::new();
    for ic in &vk.ic {
        ics.push(points_to_g1(ic[0], ic[1]))
    }
    assert!(verify_groth16_proof(
        pi_a, pi_b, pi_c, vk_alpha1, vk_beta2, vk_gamma2, vk_delta2, ics, inputs,
    ));
}

#[test]
fn test_verify_bn() {
    // todo!("Write a test for the bn library!")
}
