use diesel::{
    PgConnection, QueryResult, RunQueryDsl, QueryDsl, ExpressionMethods
};
use ipnetwork::IpNetwork;
use crate::schema::posters::dsl::*;
use crate::models::poster::{InsertPoster, Poster};

pub fn get_by_ip_address(conn: &mut PgConnection, poster_ip: IpNetwork) -> QueryResult<Poster> {
    posters.filter(ip.eq(poster_ip)).get_result(conn)
}

pub fn insert_poster(conn: &mut PgConnection, poster_ip: IpNetwork) -> QueryResult<Poster> {
    let insert_poster = InsertPoster {
        ip: poster_ip
    };

    diesel::insert_into(posters).values(insert_poster).get_result(conn)
}

pub fn get_or_insert_poster(conn: &mut PgConnection, poster_ip: IpNetwork) -> QueryResult<Poster> {
    match get_by_ip_address(conn, poster_ip) {
        Ok(poster) => Ok(poster),
        Err(diesel::result::Error::NotFound) => {
            insert_poster(conn, poster_ip)
        },
        Err(e) => Err(e)
    }
}