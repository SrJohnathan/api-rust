use diesel::{AsChangeset, Insertable, Queryable, Selectable};
use schemars::JsonSchema;
use serde_derive::{Deserialize, Serialize};

#[derive(Clone,Queryable, Selectable,Serialize,JsonSchema)]
#[diesel(table_name = super::schema::admin)]
pub struct Admin {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Insertable,Deserialize,JsonSchema,AsChangeset)]
#[diesel(table_name = super::schema::admin)]
pub struct NewAdmin {
    pub name: String,
    pub email: String,
    pub password: String,
}