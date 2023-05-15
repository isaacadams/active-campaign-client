use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ContactRequest {
    pub contact: Contact,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Contact {
    pub email: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub phone: Option<String>,
    pub field_values: Option<Vec<FieldValue>>,
}

impl Default for Contact {
    fn default() -> Self {
        Self {
            email: "johndoe@example.com".to_owned(),
            first_name: Some("John".to_owned()),
            last_name: Some("Doe".to_owned()),
            phone: Some("7223224241".to_owned()),
            field_values: Some(vec![
                FieldValue {
                    field: "1".to_owned(),
                    value: "The Value for First Field".to_owned(),
                },
                FieldValue {
                    field: "6".to_owned(),
                    value: "2008-01-20".to_owned(),
                },
            ]),
        }
    }
}

impl Contact {
    pub fn new(
        email: String,
        first_name: Option<String>,
        last_name: Option<String>,
        phone: Option<String>,
        field_values: Option<Vec<FieldValue>>,
    ) -> Self {
        Self {
            email,
            first_name,
            last_name,
            phone,
            field_values,
        }
    }

    pub fn to_request(self) -> Result<reqwest::blocking::Body, serde_json::Error> {
        println!("generating request for {}", &self.email);
        let request = ContactRequest { contact: self };
        let json = serde_json::to_string(&request)?;
        Ok(json.into())
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct FieldValue {
    pub field: String,
    pub value: String,
}

impl ContactRequest {
    pub fn new(contact: Contact) -> Self {
        Self { contact }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use serde_json::Result;

    #[test]
    fn deserialize_missing_email_fails() {
        let json_str = r#"{
            "contact": {
                "firstName": "John",
                "lastName": "Doe",
                "phone": "7223224241",
                "fieldValues": [
                    {
                        "field": "1",
                        "value": "The Value for First Field"
                    },
                    {
                        "field": "6",
                        "value": "2008-01-20"
                    }
                ]
            }
        }"#;

        let result = serde_json::from_str::<Contact>(json_str);
        assert!(result.is_err());
    }

    #[test]
    fn deserialize_1() -> Result<()> {
        let json_str = r#"{
            "contact": {
                "email": "johndoe@example.com",
                "firstName": "John",
                "lastName": "Doe",
                "phone": "7223224241",
                "fieldValues": [
                    {
                        "field": "1",
                        "value": "The Value for First Field"
                    },
                    {
                        "field": "6",
                        "value": "2008-01-20"
                    }
                ]
            }
        }"#;

        let _: ContactRequest = serde_json::from_str(json_str)?;
        Ok(())
    }

    #[test]
    fn deserialize_null_should_be_none() -> Result<()> {
        let json_str = r#"{
            "contact": {
                "email": "johndoe@example.com",
                "firstName": "John",
                "lastName": null,
                "phone": "7223224241",
                "fieldValues": null
            }
        }"#;

        let request: ContactRequest = serde_json::from_str(json_str)?;
        assert_eq!(request.contact.field_values, None);
        Ok(())
    }

    #[test]
    fn deserialize_undefined_should_be_none() -> Result<()> {
        let json_str = r#"{
            "contact": {
                "email": "johndoe@example.com",
                "lastName": null,
                "phone": "7223224241"
            }
        }"#;

        let request: ContactRequest = serde_json::from_str(json_str)?;
        assert_eq!(request.contact.field_values, None);
        Ok(())
    }

    #[test]
    fn serialize_1() -> Result<()> {
        let request = ContactRequest::new(Contact::default());
        let _ = serde_json::to_string(&request).unwrap();

        Ok(())
    }

    #[test]
    fn serialize_none_works() -> Result<()> {
        let contact = Contact {
            email: "johndoe@example.com".to_owned(),
            first_name: None,
            last_name: None,
            phone: None,
            field_values: None,
        };

        let request = ContactRequest::new(contact);
        let json = serde_json::to_string(&request).unwrap();

        assert_eq!(
            json,
            r#"{
                "contact": {
                    "email": "johndoe@example.com",
                    "firstName": null,
                    "lastName": null,
                    "phone": null,
                    "fieldValues": null
                }
            }"#
            .replace("\n", "")
            .to_string()
            .replace(" ", "")
        );

        Ok(())
    }

    #[test]
    fn serialize_deserialize_1() {
        let contact = Contact {
            field_values: None,
            ..Default::default()
        };

        let serialized_contact = serde_json::to_string(&contact).unwrap();
        let deserialized_contact: Contact = serde_json::from_str(&serialized_contact).unwrap();

        assert_eq!(contact, deserialized_contact);
    }

    #[test]
    fn serialize_deserialize_2() {
        let contact = Contact {
            field_values: Some(vec![]),
            ..Default::default()
        };

        let serialized_contact = serde_json::to_string(&contact).unwrap();
        let deserialized_contact: Contact = serde_json::from_str(&serialized_contact).unwrap();

        assert_eq!(contact, deserialized_contact);
    }
}
