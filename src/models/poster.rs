use diesel::{
    Identifiable, Insertable, Queryable
};
use ipnetwork::IpNetwork;
use crate::schema::posters;

#[derive(Identifiable, Queryable)]
pub struct Poster {
    pub id: i64,
    pub ip: IpNetwork,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime
}

#[derive(Insertable)]
#[diesel(table_name = posters)]
pub struct InsertPoster {
    pub ip: IpNetwork
}