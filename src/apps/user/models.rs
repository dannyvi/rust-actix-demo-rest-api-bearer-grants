use chrono::NaiveDateTime;
use utoipa::{ToSchema, openapi::schema};
use crate::{schema::users::{self, dsl::*}, database::Connection, errors::ApiError};

use diesel::prelude::*;

// #[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Queryable, Identifiable, Insertable)]

#[derive(Queryable, Serialize, Deserialize, ToSchema)]
pub struct User {
    pub id: String,
    pub username: String,
    pub mobile: String,
    #[schema(value_type = String, format = DateTime) ]
    pub created_at: NaiveDateTime,
    #[schema(value_type = String, format = DateTime) ]
    pub updated_at: NaiveDateTime,
}

// #[derive(Clone, Debug, Serialize, Deserialize, AsChangeset)]

#[derive(Insertable, AsChangeset, Serialize, Deserialize)]
#[table_name = "users"]
pub struct UserDTO {
    pub username: String,
    pub mobile: String,
}

impl User {
    pub fn find_all(conn: &Connection) -> QueryResult<Vec<User>> {
        // let conn = &pool.get().unwrap();
        users.load::<User>(conn)
    }

    // pub fn find_by_id(i: i32, conn: &Connection) -> QueryResult<User> {
    //     users.find(i).get_result::<User>(conn)
    // }

    // pub fn insert(new_users: UserDTO, conn: &Connection) -> QueryResult<usize> {
    //     diesel::insert_into(users)
    //         .values(&new_users)
    //         .execute(conn)
    // }

    // pub fn update(i: i32, updated_users: UserDTO, conn: &Connection) -> QueryResult<usize> {
    //     diesel::update(users.find(i))
    //         .set(&updated_users)
    //         .execute(conn)
    // }

    // pub fn delete(i: i32, conn: &Connection) -> QueryResult<usize> {
    //     diesel::delete(users.find(i)).execute(conn)
    // }
}
