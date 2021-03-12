use crate::errors::AppError;
use crate::schema::{users};
use diesel::prelude::*;

type Result<T> = std::result::Result<T, AppError>;

#[derive(Queryable, Identifiable, Serialize, Debug, PartialEq)]
pub struct User {
    pub id: i32,
    pub username: String,
}

pub enum UserKey<'a> {
    Username(&'a str),
    Id(i32),
}

pub fn create_user(connection: &SqliteConnection, username: &str) -> Result<User> {
    connection.transaction(|| {
        diesel::insert_into(users::table)
            .values((users::username.eq(username), ))
            .execute(connection)?;

        users::table
            .order(users::id.desc())
            .select((users::id, users::username))
            .first(connection)
            .map_err(Into::into)
    })
}

pub fn find_user<'a>(connection: &SqliteConnection, key: UserKey<'a>) -> Result<User> {
    match key {
        UserKey::Username(name) => users::table
            .filter(users::username.eq(name))
            .select((users::id, users::username))
            .first::<User>(connection)
            .map_err(AppError::from),
        UserKey::Id(id) => users::table
            .find(id)
            .select((users::id, users::username))
            .first::<User>(connection)
            .map_err(Into::into)
    }
}
