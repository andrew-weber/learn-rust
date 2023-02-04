use diesel::{
    prelude::*,
    r2d2::{self, ConnectionManager},
};

type DbError = Box<dyn std::error::Error + Send + Sync>;
use crate::models::{InsertUser, User};

pub fn create_connection_pool() -> r2d2::Pool<ConnectionManager<PgConnection>> {
    dotenv::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.")
}

pub fn create_user(conn: &mut PgConnection, data: InsertUser) -> Result<User, DbError> {
    use crate::schema::users::dsl::*;

    diesel::insert_into(users).values(&data).execute(conn)?;

    Ok(User {
        id: 1,
        name: data.name.to_string(),
    })
}

pub fn get_user(conn: &mut PgConnection, name: &str) -> Result<Option<User>, DbError> {
    use crate::schema::users::dsl::*;

    let user = users.filter(name.eq(name)).first::<User>(conn).optional()?;

    Ok(user)
}
