pub const TOPIC: &'static str = "test";
const PORT: &'static str = "9092";
const IP: &'static str = "10.10.0.58";

pub fn get_host() -> String {
    format!("{}:{}", IP, PORT)
}
