/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */

/// TypeInvitation : Resource type

/// Resource type
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TypeInvitation {
    #[serde(rename = "invitation")]
    Invitation,

}

impl ToString for TypeInvitation {
    fn to_string(&self) -> String {
        match self {
            Self::Invitation => String::from("invitation"),
        }
    }
}

impl Default for TypeInvitation {
    fn default() -> TypeInvitation {
        Self::Invitation
    }
}




