use serde_json::Value;

fn main() {
    let value: Value = serde_json::from_str(r#"{
        "name": "John Doe",
        "age": 43,
        "phones": [
            "+44 1234567",
            "+44 2345678"
        ]
    }"#).unwrap();
    let serialized = serde_yaml::to_string(&value).unwrap();
    println!("yaml = {}", serialized);
    let serialized = serde_xml_rs::to_string(&value).unwrap();
    println!("xml = {}", serialized);
    let serialized = toml::to_string(&value).unwrap();
    println!("toml = {}", serialized);
}
