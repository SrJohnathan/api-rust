
use rocket::{delete, get, post, put};
use rocket::futures::SinkExt;
use rocket::response::status;
use rocket::serde::json::Json;
use rocket_okapi::openapi;
use crate::server::data::PoolPost;
use crate::server::model::{Admin, NewAdmin};
use crate::server::repository::{create_admin, delete_admin, get_admins, update_admin_repo};


#[openapi(tag = "Admin")]
#[post("/admin", format = "application/json", data = "<task>")]
pub async fn isert_admin(mut db: PoolPost<'_>, task: Json<NewAdmin>) -> Result<status::Created<Json<Admin>>, status::BadRequest<String>> {
    match create_admin(&mut db, task.0).await {
        Ok(x) => {
            let res = status::Created::new("").body(Json(x));
            Ok(res)
        }
        Err(e) => {
            Err(status::BadRequest(e))
        }
    }
}

#[openapi (tag = "Admin")]
#[get("/all")]
pub async fn get_all(mut db:PoolPost<'_>) -> Result<status::Accepted<Option<Json<Vec<Admin>>>>,status::BadRequest<String>> {
    match get_admins(&mut db).await {
        Ok(x) => {

          //  let mut  xx  = x.to_vec();

           Ok(   status::Accepted(Some(Json(x))) )
        }
        Err(e) => {
            Err(status::BadRequest(e))
        }
    }

}

#[openapi (tag = "Admin")]
#[delete("/delete/<id>")]
pub async fn delete(mut db:PoolPost<'_>,id:i32) -> Result<status::Accepted<Option<Json<Admin>>>,status::BadRequest<String>> {
    match delete_admin(&mut db,id).await {
        Ok(x) => {
            Ok(   status::Accepted(Some(Json(x))) )
        }
        Err(e) => {
            Err(status::BadRequest(e))
        }
    }

}


#[openapi (tag = "Admin")]
#[put("/<id>",format = "application/json", data = "<task>")]
pub async fn update_admin(mut db:PoolPost<'_>,id:i32, task: Json<NewAdmin>) -> Result<status::Accepted<Option<Json<Admin>>>,status::BadRequest<String>> {
    match update_admin_repo(&mut db,id,task.0).await {
        Ok(x) => {
            Ok(   status::Accepted(Some(Json(x))) )
        }
        Err(e) => {
            Err(status::BadRequest(e))
        }
    }

}

