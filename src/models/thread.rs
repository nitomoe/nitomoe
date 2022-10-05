use diesel::deserialize::{FromSql, self};
use diesel::pg::{Pg, PgValue};
use diesel::serialize::{ToSql, Output, self, IsNull};
use diesel::{
    Identifiable, Queryable, Insertable, AsExpression, FromSqlRow, Associations
};
use std::io::Write;
use crate::models::board::Board;
use crate::schema::threads;
use crate::schema::sql_types::Threadstatus;

#[derive(Debug, AsExpression, FromSqlRow, PartialEq)]
#[diesel(sql_type = Threadstatus)]
pub enum ThreadStatus {
    Open,
    Locked
}

impl ToSql<Threadstatus, Pg> for ThreadStatus {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        match *self {
            Self::Open => out.write_all(b"Open")?,
            Self::Locked => out.write_all(b"Locked")?,
        }
        Ok(IsNull::No)
    }
}

impl FromSql<Threadstatus, Pg> for ThreadStatus {
    fn from_sql(bytes: PgValue) -> deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"Open" => Ok(Self::Open),
            b"Locked" => Ok(Self::Locked),
            x => Err(format!("Unrecognized ThreadStatus enum variant: {:?}", x).into())
        }
    }
}

#[derive(Debug, Identifiable, Queryable, Associations, PartialEq)]
#[diesel(belongs_to(Board))]
pub struct Thread {
    pub id: i64,
    pub subject: Option<String>,
    pub board_id: i32,
    pub status: ThreadStatus,
    pub sticky: bool,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime
}

#[derive(Insertable)]
#[diesel(table_name = threads)]
pub struct InsertThread {
    pub subject: Option<String>,
    pub board_id: i32,
}