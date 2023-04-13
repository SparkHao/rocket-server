use crate::schema::products;
use rocket::serde::{Serialize, Deserialize};

#[derive(Queryable, Deserialize, Serialize, AsChangeset)]
#[serde(crate = "rocket::serde")]
pub struct Product {
    pub id: i32,
    pub name: String,
    pub description: String,
    #[serde(skip_deserializing)]
    pub create_at: String
}

#[derive(Insertable, Deserialize)]
#[serde(crate = "rocket::serde")]
#[table_name = "products"]
pub struct NewProduct {
    pub name: String,
    pub description: String,
}