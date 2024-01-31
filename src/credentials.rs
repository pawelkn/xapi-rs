use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Credentials {
    pub account_id: String,
    pub password: String,
    pub host: String,
    pub type_: String,
    pub safe: bool,
}

impl Credentials {
    pub fn default() -> Credentials {
        Credentials {
            account_id: String::new(),
            password: String::new(),
            host: String::from("ws.xtb.com"),
            type_: String::from("real"),
            safe: false,
        }
    }

    pub fn loads(json: &str) -> Result<Credentials, serde_json::Error> {
        serde_json::from_str(json)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_credentials_default() {
        let creds = Credentials::default();
        assert_eq!(creds.host, "ws.xtb.com");
        assert_eq!(creds.type_, "real");
        assert_eq!(creds.safe, false);
    }

    #[test]
    fn test_credentials_loads() {
        const DATA: &str = r#"{
            "accountId": "John Doe",
            "password": "123456",
            "host": "example.com",
            "type": "demo",
            "safe": true
        }"#;

        let creds = Credentials::loads(DATA);
        assert!(creds.is_ok());

        let creds = creds.unwrap();
        assert!(creds.account_id == "John Doe");
        assert!(creds.password == "123456");
        assert!(creds.host == "example.com");
        assert!(creds.type_ == "demo");
        assert!(creds.safe);
    }
}
