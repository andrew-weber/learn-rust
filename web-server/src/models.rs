use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::schema::users;

#[derive(Queryable, Deserialize, Serialize, Debug)]
pub struct User {
    pub id: i32,
    pub name: String,
}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct InsertUser<'a> {
    pub name: &'a str,
}
