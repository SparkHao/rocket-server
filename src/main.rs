#[macro_use] extern crate diesel;
use crate::diesel::RunQueryDsl;
use diesel::QueryDsl;
use diesel::ExpressionMethods;
use rocket::response::status;
use rocket::{serde::json::{Value, serde_json::json, Json}, get, post, put, delete, catch, catchers, routes, http::Status};
use rocket_sync_db_pools::database;

mod basic_auth;
use basic_auth::BasicAuthStruct;
mod models;
pub use models::*;
mod schema;
use schema::products;
mod repositoires;
use repositoires::ProductRepository;

#[database("sqlite_path")]
struct DbConn(diesel::SqliteConnection);



#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/")]
async fn get_products(conn: DbConn) -> Result<Value, status::Custom<Value>> {
    conn.run(|con| {
        // let products = products::table.limit(100)
        //     .load::<Product>(con)
        //     .expect("Error products list");
        // json!(products)
        // let products = ProductRepository::find_all(con)
        //     .expect("Error products list");
        // json!(products)

        ProductRepository::find_all(con)
            .map(|product| json!(product))
            .map_err(|e| status::Custom(Status::InternalServerError, json!(e.to_string())))
    }).await
}

#[get("/<id>")]
async fn view_product(id: i32, conn: DbConn) -> Result<Value, status::Custom<Value>> {
    conn.run(move |con| {
        // let product = products::table.find(id)
        //     .get_result::<Product>(con)
        //     .expect("Error Get");
        // let product = ProductRepository::find(con, id).expect("Error get");
        // json!(product)
        ProductRepository::find(con, id)
            .map(|product| json!(product))
            .map_err(|e| status::Custom(Status::InternalServerError, json!(e.to_string())))
    }).await
}


#[post("/", format="json", data="<new_product>")]
async fn create_product(_auth: BasicAuthStruct, conn: DbConn, new_product: Json<NewProduct>) -> Result<Value, status::Custom<Value>> {
    conn.run(|con| {
        // let result = diesel::insert_into(products::table)
        //     .values(new_product.into_inner())
        //     .execute(con)
        //     .expect("Error create product");
        // let result = ProductRepository::create(con, new_product.into_inner()).expect("insert failed");
        // json!(result)
        ProductRepository::create(con, new_product.into_inner())
            .map(|product| json!(product))
            .map_err(|e| status::Custom(Status::InternalServerError, json!(e.to_string())))
    }).await
}

#[put("/", format="json", data="<product>")]
async fn put_product(_auth: BasicAuthStruct, conn: DbConn, product: Json<Product>) -> Result<Value, status::Custom<Value>> {
    conn.run(move|con| {
        // let result = diesel::update(products::table.find(id))
        //     .set((
        //         products::name.eq(product.name.to_owned()),
        //         products::description.eq(product.description.to_owned())
        //     ))
        //     .execute(con)
        // let result = ProductRepository::save(con, product.into_inner())
        //     .expect("Error update");
        // json!(result)
        ProductRepository::save(con, product.into_inner())
            .map(|product| json!(product))
            .map_err(|e| status::Custom(Status::InternalServerError, json!(e.to_string())))
    }).await
}

#[delete("/<id>")]
async fn delete_product(id: i32, _auth: BasicAuthStruct, conn: DbConn) -> Result<Value, status::Custom<Value>> {
    conn.run(move |con| {
        // let result = diesel::delete(products::table.find(id))
        //     .execute(con)
        //     .expect("Error for delete");
        // let result = ProductRepository::delete(con, id).expect("Delete failed");
        // json!(result)
        ProductRepository::delete(con, id)
            .map(|product| json!(product))
            .map_err(|e| status::Custom(Status::InternalServerError, json!(e.to_string())))
    }).await
}

#[catch(404)]
async fn not_found_url() -> Value {
    json!("not found!")
}

#[rocket::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    rocket::build()
        .mount("/", routes![index])
        .mount("/product", routes![
            get_products, 
            view_product, 
            create_product, 
            put_product, 
            delete_product
        ])
        .register("/", catchers!(not_found_url))
        .attach(DbConn::fairing())
        .launch().await?;
    Ok(())
}