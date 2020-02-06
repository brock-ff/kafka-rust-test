use serde::{Deserialize, Serialize};
use serde_json;

pub fn encode<T>(val: &T) -> Result<Vec<u8>, serde_json::Error>
where
    T: Serialize,
{
    Ok(serde_json::to_string(val)?.as_bytes().to_vec())
}

pub fn decode<'a, T>(val: &'a [u8]) -> Result<T, serde_json::Error>
where
    T: Deserialize<'a>,
{
    Ok(serde_json::from_slice(val)?)
}

#[allow(unused_imports)]
mod tests {
    use super::{decode, encode};
    use crate::data::Data;
    #[test]
    pub fn it_encodes_and_decodes() {
        let data = Data::new();
        println!("original\n{}", data);

        let encoded_data = encode(&data).unwrap();
        println!("encoded\n{:?}", encoded_data);
        let decoded_data: Data = decode(&encoded_data).unwrap();
        println!("decoded\n{}", decoded_data);

        assert_eq!(decoded_data.name, data.name);
        assert_eq!(decoded_data.id, data.id);
    }
}
