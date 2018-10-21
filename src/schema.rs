use diesel::deserialize::{self, FromSql};
use diesel::pg::Pg;
use diesel::serialize::{self, IsNull, Output, ToSql};
use diesel::*;
use std::io::Write;

table! {
    use diesel::sql_types::*;
    use super::Unit;

    settings (id) {
        id -> Unit,
        username -> Text,
        password -> Text,
        info -> Text,
        picture -> Bytea,
        upload_slots -> Int4,
        upload_rate -> Int4,
        download_slots -> Int4,
        download_rate -> Int4,
    }
}

#[postgres(type_name = "unit")]
#[sql_type = "Unit"]
#[derive(SqlType, FromSqlRow, AsExpression, Debug, PartialEq, Eq, Hash, Default, Serialize, Deserialize, Clone)]
pub struct Unit;

impl ToSql<Unit, Pg> for Unit {
    fn to_sql<W: Write>(&self, out: &mut Output<W, Pg>) -> serialize::Result {
        out.write(b"unit")?;
        Ok(IsNull::No)
    }
}

impl FromSql<Unit, Pg> for Unit {
    fn from_sql(bytes: Option<&[u8]>) -> deserialize::Result<Self> {
        Ok(Unit)
    }
}
