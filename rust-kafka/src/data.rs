use rand::Rng;
use serde::{Deserialize, Serialize};
use serde_json;
use std::fmt::Formatter;

#[derive(Debug, Deserialize, Serialize)]
pub enum Key {
    BurnErc20,
    BurnErc721,
    BurnErc1155,
    MintErc20,
    MintErc721,
    MintErc1155,
    TransferErc20,
    TransferErc721,
    TransferErc1155,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Data {
    pub name: String,
    pub id: u64,
}

impl Data {
    pub fn new() -> Self {
        // get random ID number
        let mut rng = rand::thread_rng();
        let id: u64 = rng.gen_range(69, 420);
        Data {
            name: format!("item {}", id),
            id,
        }
    }
}

impl std::fmt::Display for Data {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        f.write_str(&serde_json::to_string_pretty(self).unwrap())
    }
}

impl std::fmt::Display for Key {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        f.write_str(match self {
            Key::BurnErc20 => "BurnErc20",
            Key::BurnErc721 => "BurnErc721",
            Key::BurnErc1155 => "BurnErc1155",
            Key::MintErc20 => "MintErc20",
            Key::MintErc721 => "MintErc721",
            Key::MintErc1155 => "MintErc1155",
            Key::TransferErc20 => "TransferErc20",
            Key::TransferErc721 => "TransferErc721",
            Key::TransferErc1155 => "TransferErc1155",
        })
    }
}
