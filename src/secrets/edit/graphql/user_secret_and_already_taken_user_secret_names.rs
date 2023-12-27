#![allow(clippy::all, warnings)]
pub struct UserSecretAndAlreadyTakenUserSecretNames;
pub mod user_secret_and_already_taken_user_secret_names {
    #![allow(dead_code)]
    use std::result::Result;
    use strum_macros::{EnumIter, ToString};
    pub const OPERATION_NAME: &str = "UserSecretAndAlreadyTakenUserSecretNames";
    pub const QUERY : & str = "query UserSecretAndAlreadyTakenUserSecretNames($id: uuid!) {\n  userSecret_by_pk(id: $id) {\n    id\n    name\n    slug\n    vendor\n    tokenId\n\n    fields {\n      id\n      name\n      value\n      slug\n    }\n\n    token {\n      id\n\n      userSecret {\n        id\n        name\n      }\n    }\n  }\n}\n" ;
    use serde::{Deserialize, Serialize};
    #[allow(dead_code)]
    type Boolean = bool;
    #[allow(dead_code)]
    type Float = f64;
    #[allow(dead_code)]
    type Int = i64;
    #[allow(dead_code)]
    type ID = String;
    type uuid = ::uuid::Uuid;
    #[derive(EnumIter, ToString)]
    pub enum vendorEnum_enum {
        agora,
        aws,
        azure,
        braintree,
        digitalOcean,
        googleCloud,
        mailchimp,
        mixpanel,
        other,
        paypal,
        pulumi,
        segment,
        sendgrid,
        stripe,
        terraform,
        twilio,
        Other(String),
    }
    impl ::serde::Serialize for vendorEnum_enum {
        fn serialize<S: serde::Serializer>(&self, ser: S) -> Result<S::Ok, S::Error> {
            ser.serialize_str(match *self {
                vendorEnum_enum::agora => "agora",
                vendorEnum_enum::aws => "aws",
                vendorEnum_enum::azure => "azure",
                vendorEnum_enum::braintree => "braintree",
                vendorEnum_enum::digitalOcean => "digitalOcean",
                vendorEnum_enum::googleCloud => "googleCloud",
                vendorEnum_enum::mailchimp => "mailchimp",
                vendorEnum_enum::mixpanel => "mixpanel",
                vendorEnum_enum::other => "other",
                vendorEnum_enum::paypal => "paypal",
                vendorEnum_enum::pulumi => "pulumi",
                vendorEnum_enum::segment => "segment",
                vendorEnum_enum::sendgrid => "sendgrid",
                vendorEnum_enum::stripe => "stripe",
                vendorEnum_enum::terraform => "terraform",
                vendorEnum_enum::twilio => "twilio",
                vendorEnum_enum::Other(ref s) => &s,
            })
        }
    }
    impl<'de> ::serde::Deserialize<'de> for vendorEnum_enum {
        fn deserialize<D: ::serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
            let s: String = ::serde::Deserialize::deserialize(deserializer)?;
            match s.as_str() {
                "agora" => Ok(vendorEnum_enum::agora),
                "aws" => Ok(vendorEnum_enum::aws),
                "azure" => Ok(vendorEnum_enum::azure),
                "braintree" => Ok(vendorEnum_enum::braintree),
                "digitalOcean" => Ok(vendorEnum_enum::digitalOcean),
                "googleCloud" => Ok(vendorEnum_enum::googleCloud),
                "mailchimp" => Ok(vendorEnum_enum::mailchimp),
                "mixpanel" => Ok(vendorEnum_enum::mixpanel),
                "other" => Ok(vendorEnum_enum::other),
                "paypal" => Ok(vendorEnum_enum::paypal),
                "pulumi" => Ok(vendorEnum_enum::pulumi),
                "segment" => Ok(vendorEnum_enum::segment),
                "sendgrid" => Ok(vendorEnum_enum::sendgrid),
                "stripe" => Ok(vendorEnum_enum::stripe),
                "terraform" => Ok(vendorEnum_enum::terraform),
                "twilio" => Ok(vendorEnum_enum::twilio),
                _ => Ok(vendorEnum_enum::Other(s)),
            }
        }
    }
    #[derive(Serialize)]
    pub struct Variables {
        pub id: uuid,
    }
    impl Variables {}
    #[derive(Deserialize)]
    pub struct ResponseData {
        #[serde(rename = "userSecret_by_pk")]
        pub user_secret_by_pk: Option<UserSecretAndAlreadyTakenUserSecretNamesUserSecretByPk>,
    }
    #[derive(Deserialize)]
    pub struct UserSecretAndAlreadyTakenUserSecretNamesUserSecretByPk {
        pub id: uuid,
        pub name: String,
        pub slug: String,
        pub vendor: vendorEnum_enum,
        #[serde(rename = "tokenId")]
        pub token_id: uuid,
        pub fields: Vec<UserSecretAndAlreadyTakenUserSecretNamesUserSecretByPkFields>,
        pub token: Option<UserSecretAndAlreadyTakenUserSecretNamesUserSecretByPkToken>,
    }
    #[derive(Deserialize)]
    pub struct UserSecretAndAlreadyTakenUserSecretNamesUserSecretByPkFields {
        pub id: uuid,
        pub name: String,
        pub value: String,
        pub slug: String,
    }
    #[derive(Deserialize)]
    pub struct UserSecretAndAlreadyTakenUserSecretNamesUserSecretByPkToken {
        pub id: uuid,
        #[serde(rename = "userSecret")]
        pub user_secret: Vec<UserSecretAndAlreadyTakenUserSecretNamesUserSecretByPkTokenUserSecret>,
    }
    #[derive(Deserialize)]
    pub struct UserSecretAndAlreadyTakenUserSecretNamesUserSecretByPkTokenUserSecret {
        pub id: uuid,
        pub name: String,
    }
}
impl graphql_client::GraphQLQuery for UserSecretAndAlreadyTakenUserSecretNames {
    type Variables = user_secret_and_already_taken_user_secret_names::Variables;
    type ResponseData = user_secret_and_already_taken_user_secret_names::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: user_secret_and_already_taken_user_secret_names::QUERY,
            operation_name: user_secret_and_already_taken_user_secret_names::OPERATION_NAME,
        }
    }
}
