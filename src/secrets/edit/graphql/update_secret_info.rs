#![allow(clippy::all, warnings)]
pub struct UpdateSecretInfo;
pub mod update_secret_info {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "UpdateSecretInfo";
    pub const QUERY : & str = "mutation UpdateSecretInfo($id: uuid!, $set: userSecret_set_input!) {\n  update_userSecret_by_pk(pk_columns: {id: $id}, _set: $set) {\n    id\n  }\n}\n" ;
    use ::uuid::Uuid;
    use chrono::offset::Utc;
    use chrono::DateTime;
    use serde::{Deserialize, Serialize};
    use strum_macros::{EnumIter, ToString};
    #[allow(dead_code)]
    type Boolean = bool;
    #[allow(dead_code)]
    type Float = f64;
    #[allow(dead_code)]
    type Int = i64;
    #[allow(dead_code)]
    type ID = String;
    type timestamptz = DateTime<Utc>;
    type uuid = Uuid;
    #[derive(EnumIter, ToString, Clone)]
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
    pub struct userSecret_set_input {
        #[serde(rename = "createdAt", skip_serializing_if = "Option::is_none")]
        pub created_at: Option<timestamptz>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub id: Option<uuid>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub slug: Option<String>,
        #[serde(rename = "tokenId", skip_serializing_if = "Option::is_none")]
        pub token_id: Option<uuid>,
        #[serde(rename = "updatedAt", skip_serializing_if = "Option::is_none")]
        pub updated_at: Option<timestamptz>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub vendor: Option<vendorEnum_enum>,
    }
    #[derive(Serialize)]
    pub struct Variables {
        pub id: uuid,
        pub set: userSecret_set_input,
    }
    impl Variables {}
    #[derive(Deserialize)]
    pub struct ResponseData {
        #[serde(rename = "update_userSecret_by_pk")]
        pub update_user_secret_by_pk: Option<UpdateSecretInfoUpdateUserSecretByPk>,
    }
    #[derive(Deserialize)]
    pub struct UpdateSecretInfoUpdateUserSecretByPk {
        pub id: uuid,
    }
}
impl graphql_client::GraphQLQuery for UpdateSecretInfo {
    type Variables = update_secret_info::Variables;
    type ResponseData = update_secret_info::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: update_secret_info::QUERY,
            operation_name: update_secret_info::OPERATION_NAME,
        }
    }
}
