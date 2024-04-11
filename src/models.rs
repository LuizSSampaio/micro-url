use serde::{Serialize, Deserialize};

use crate::schema::links;

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = links)]
pub struct NewShortLink<'a> {
    pub url_id: &'a str,
    pub long_url: &'a str,
}

#[derive(Debug, Queryable, AsChangeset)]
pub struct Link {
    pub id: i32,
    pub url_id: String,
    pub long_url: String,
}
