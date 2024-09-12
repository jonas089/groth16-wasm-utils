use casper_types::U256;
use serde::Deserialize;
use std::{env, fs, path::PathBuf};

#[derive(Deserialize, Debug)]
pub struct VerifyingKey {
    pub vk_alpha_1: [U256; 3],
    pub vk_beta_2: [[U256; 2]; 3],
    pub vk_gamma_2: [[U256; 2]; 3],
    pub vk_delta_2: [[U256; 2]; 3],
    pub vk_alphabeta_12: [[[U256; 2]; 3]; 2],
    #[serde(rename = "IC")]
    pub ic: Vec<[U256; 3]>,
}

#[derive(Deserialize, Debug)]
pub struct Proof {
    pub pi_a: [U256; 3],
    pub pi_b: [[U256; 2]; 3],
    pub pi_c: [U256; 3],
}
pub fn read_vk() -> VerifyingKey {
    let path_to_vk: PathBuf = PathBuf::from(
        env::var("PATH_TO_VERIFYING_KEY")
            .unwrap_or("/Users/chef/Desktop/groth16-utils-casper/groth16-verifier-casper/resources/verification_key.json".to_string()),
    );
    let vk_file_content = fs::read_to_string(path_to_vk).expect("Failed to read vk");
    serde_json::from_str(&vk_file_content).expect("Failed to parse vk")
}

pub fn read_proof() -> Proof {
    let path_to_proof: PathBuf = PathBuf::from(
        env::var("PATH_TO_PROOF").unwrap_or(
            "/Users/chef/Desktop/groth16-utils-casper/groth16-verifier-casper/resources/proof.json"
                .to_string(),
        ),
    );
    let proof_file_content = fs::read_to_string(path_to_proof).expect("Failed to read proof");
    serde_json::from_str(&proof_file_content).expect("Failed to parse proof")
}
