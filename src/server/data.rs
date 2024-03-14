use std::ops::Deref;
use bb8::{ Pool, PooledConnection};
use diesel_async::AsyncPgConnection;
use diesel_async::pooled_connection::{AsyncDieselConnectionManager, PoolError};
use rocket::{Request,State};
use rocket::request::{FromRequest, Outcome};
use rocket_okapi::gen::OpenApiGenerator;
use rocket_okapi::request::{OpenApiFromRequest, RequestHeaderInput};

pub type PgAsync = Pool<AsyncDieselConnectionManager<AsyncPgConnection>>;

pub struct PoolPost<'a>(pub PooledConnection<'a, AsyncDieselConnectionManager<AsyncPgConnection>>);


#[rocket::async_trait]
impl  <'r>FromRequest<'r> for  PoolPost<'r> {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {

        let  res = request.guard::<&State<PgAsync>>().await;

        match res {

            Outcome::Success(s) =>{

                let res =  s.get().await.unwrap();

                Outcome::Success(PoolPost(res))
            }
            Outcome::Error(s) => Outcome::Error(s),
            Outcome::Forward(s) =>Outcome::Forward(s)
        }

    }
}
impl Deref for PoolPost<'_> {
    type Target = AsyncPgConnection;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}


pub async fn connection<'a>() -> Result<PgAsync,String> {
    match std::env::var("DATABASE_URL") {
        Ok(x) => {
            let manager = AsyncDieselConnectionManager::<AsyncPgConnection>::new(x);

            match Pool::builder()
                .test_on_check_out(true)
                .build(manager).await {
                Ok(x) => {
                    Ok(x)
                }
                Err(e) => {
                    Err(e.to_string())
                }
            }
        }
        Err(e) => {
            Err(e.to_string())
        }
    }
}

impl <'r> OpenApiFromRequest<'r> for PoolPost<'r>{
    fn from_request_input(gen: &mut OpenApiGenerator, name: String, required: bool) -> rocket_okapi::Result<RequestHeaderInput> {
        Ok(RequestHeaderInput::None)
    }
}