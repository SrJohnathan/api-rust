use dotenvy::dotenv;
use rocket::{Error};
use crate::server::create;

mod  server;



#[rocket::main]
async  fn main() -> Result<(),Error> {
    dotenv().ok();
    
    match   create().await {
        Ok(x) => {
            println!("start http")
        }
        Err(x) => {
            
        }
    }
    
    Ok(())
}
