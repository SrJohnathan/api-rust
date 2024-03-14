use dotenvy::dotenv;
use rocket::{Error, Ignite, Rocket, Route};
use rocket_okapi::{openapi_get_routes, openapi_routes};
use rocket_okapi::rapidoc::{GeneralConfig, HideShowConfig, make_rapidoc, RapiDocConfig, Theme, UiConfig};
use rocket_okapi::settings::UrlObject;
use crate::server::data::connection;

pub mod schema;
mod data;
pub mod model;
pub mod repository;
mod controller;


pub  fn rapi_doc () ->  impl Into <Vec<Route>> + Sized {
    
    make_rapidoc(&RapiDocConfig {
        
        general : GeneralConfig { spec_urls : vec![UrlObject::new("General","../api/openapi.json")], ..Default::default() },
        
        hide_show : HideShowConfig {
            allow_spec_file_load:false,
            allow_spec_url_load:false,
            ..Default::default()
            
        },
        
        ui : UiConfig {
            theme : Theme::Dark ,
            ..Default::default()
        },
        
            ..Default::default()
    })
    
}

pub  async  fn create() -> Result<Rocket<Ignite>, Error> {
    let db = connection().await.unwrap();
    
    rocket::build()
        .manage(db)
        .mount("/",rapi_doc())
        .mount("/api",
               openapi_get_routes![
                   controller::isert_admin,
                   controller::get_all,
                   controller::delete,
                   controller::update_admin
               ])
        .launch()
        .await
    
    
}
