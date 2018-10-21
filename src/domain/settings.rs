use std::str;

use serde::de::{self, Deserialize, Deserializer, Visitor, MapAccess};
use serde::ser::Serializer;

use schema::{
    Unit,
    settings,
};

#[derive(Debug, Default, Serialize, Deserialize, Clone, Queryable, AsChangeset, Identifiable, Insertable)]
#[serde(rename_all = "camelCase")]
#[table_name = "settings"]
pub struct Settings {
    #[serde(skip_serializing)]
    pub id: Unit,
    pub username: String,
    pub password: String,
    pub info: String,
    #[serde(deserialize_with = "string_to_vec")]
    #[serde(serialize_with = "vec_to_str")]
    pub picture: Vec<u8>,
    pub upload_slots: i32,
    pub upload_rate: i32,
    pub download_slots: i32,
    pub download_rate: i32,
}


fn string_to_vec<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
where
    D: Deserializer<'de>,
{
    let string = String::deserialize(deserializer)?;

    Ok(string.into_bytes())
}

fn vec_to_str<'a, S>(bytes: &'a Vec<u8>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    if bytes.is_empty() {
        return serializer.serialize_unit();
    }

    match str::from_utf8(bytes) {
        Ok(result) => serializer.serialize_str(&result),
        Err(_) => serializer.serialize_unit()
    }
}
