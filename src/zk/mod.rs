mod mimc;
pub mod ram;

use crate::crypto::Fr;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// A single state cell
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ZkScalar(Fr);

// Full state of a contract
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ZkState(HashMap<u32, ZkScalar>);

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ZkVerifierKey(#[serde(with = "serde_bytes")] Vec<u8>);

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ZkProof(#[serde(with = "serde_bytes")] Vec<u8>);
