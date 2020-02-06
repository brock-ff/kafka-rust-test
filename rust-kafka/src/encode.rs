use serde::{Deserialize, Serialize};
use serde_json;

pub fn encode<T>(val: &T) -> Vec<u8>
where
    T: Serialize,
{
    serde_json::to_string(val).unwrap().as_bytes().to_vec()
}

pub fn decode<'a, T>(val: &'a [u8]) -> T
where
    T: Deserialize<'a>,
{
    serde_json::from_slice(val).unwrap()
}

#[allow(unused_imports)]
mod tests {
    use super::{decode, encode};
    use crate::data::Data;
    #[test]
    pub fn it_encodes_and_decodes() {
        let data = Data::new();
        println!("original\n{}", data);

        let encoded_data = encode(&data);
        println!("encoded\n{:?}", encoded_data);
        let decoded_data: Data = decode(&encoded_data);
        println!("decoded\n{}", decoded_data);

        assert_eq!(decoded_data.name, data.name);
        assert_eq!(decoded_data.id, data.id);
    }
}
