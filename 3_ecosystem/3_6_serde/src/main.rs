use chrono::{DateTime, FixedOffset};
use duration_string::DurationString;
use serde::{Deserialize, Serialize};
use serde_json;
use serde_yaml;
use toml;
use url::Url;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Request {
    #[serde(rename(serialize = "type", deserialize = "type"))]
    field_type: String,
    stream: Stream,
    gifts: Vec<Gifts>,
    debug: Debug,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Stream {
    user_id: String,
    is_private: bool,
    settings: u64,
    shard_url: Url,
    public_tariff: PublicTariff,
    private_tariff: PrivateTrariff,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct PublicTariff {
    id: u64,
    price: u64,
    duration: DurationString,
    description: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct PrivateTrariff {
    client_price: u64,
    duration: DurationString,
    description: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Gifts {
    id: u64,
    price: u64,
    description: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Debug {
    duration: DurationString,
    at: DateTime<FixedOffset>,
}

use std::fs::File;
use std::io::Read;

fn main() {
    let mut file = File::open("request.json").unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();

    let request = serde_json::from_str::<Request>(&data).unwrap();

    println!("{:?}", request);

    println!("{}", serde_yaml::to_string(&request).unwrap());
    println!();
    println!("{}", toml::to_string(&request).unwrap());
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn json_serialize_public_tariff() {
        let private_tariff = PrivateTrariff {
            client_price: 250,
            duration: "1m".to_string().try_into().unwrap(),
            description: "test private tariff".to_string(),
        };
        let private_tariff_string =
            r#"{"client_price":250,"duration":"1m","description":"test private tariff"}"#;
        assert_eq!(
            private_tariff_string,
            serde_json::to_string(&private_tariff).unwrap()
        );
    }

    #[test]
    fn json_serialize_private_tariff() {
        let public_tariff = PublicTariff {
            id: 1,
            price: 100,
            duration: "1h".to_string().try_into().unwrap(),
            description: "test public tariff".to_string(),
        };
        let public_tariff_string =
            r#"{"id":1,"price":100,"duration":"1h","description":"test public tariff"}"#;
        assert_eq!(
            public_tariff_string,
            serde_json::to_string(&public_tariff).unwrap()
        );
    }

    #[test]
    fn json_deserialize_debug() {
        let debug = Debug {
            duration: "200ms".to_string().try_into().unwrap(),
            at: DateTime::parse_from_rfc3339("2019-06-28T08:35:46Z").unwrap(),
        };
        let debug_string = r#"{"duration":"200ms","at":"2019-06-28T08:35:46Z"}"#;
        assert_eq!(serde_json::from_str::<Debug>(debug_string).unwrap(), debug);
    }
}
