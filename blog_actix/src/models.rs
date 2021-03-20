use crate::errors::AppError;
use crate::schema::{users, posts};
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

// Users ///
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

// Posts ///
#[derive(Queryable, Associations, Identifiable, Serialize, Debug)]
#[belongs_to(User)]
pub struct Post {
    pub id: i32,
    pub user_id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}

pub fn create_post(connection: &SqliteConnection, user: &User, title: &str, body: &str) -> Result<Post> {
    connection.transaction(|| {
        diesel::insert_into(posts::table)
            .values((
                posts::user_id.eq(user.id),
                posts::title.eq(title),
                posts::body.eq(body)
            ))
            .execute(connection)?;

        posts::table
            .order(posts::id.desc())
            .select(posts::all_columns)
            .first(connection)
            .map_err(Into::into)
    })
}

pub fn publish_post(connection: &SqliteConnection, post_id: i32) -> Result<Post> {
    connection.transaction(|| {
        diesel::update(posts::table.filter(posts::id.eq(post_id)))
            .set(posts::published.eq(true))
            .execute(connection)?;

        posts::table
            .find(post_id)
            .select(posts::all_columns)
            .first(connection)
            .map_err(Into::into)
    })
}

pub fn all_posts(connection: &SqliteConnection) -> Result<Vec<(Post, User)>> {
    posts::table
        .order(posts::id.desc())
        .filter(posts::published.eq(true))
        .inner_join(users::table)
        .select((posts::all_columns, (users::id, users::username)))
        .load::<(Post,User)>(connection)
        .map_err(Into::into)
}

pub fn user_posts(connection: &SqliteConnection, user_id: i32) -> Result<Vec<Post>> {
    posts::table
        .filter(posts::user_id.eq(user_id))
        .order(posts::id.desc())
        .select(posts::all_columns)
        .load::<Post>(connection)
        .map_err(Into::into)
}