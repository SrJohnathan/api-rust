use diesel::{ExpressionMethods, QueryDsl};
use diesel_async::RunQueryDsl;
use crate::server::data::{PoolPost};
use crate::server::model::{Admin, NewAdmin};
use crate::server::schema::admin;

pub async fn create_admin(db: &mut PoolPost<'_>, ad: NewAdmin) -> Result<Admin, String> {
    let res =
        diesel::insert_into(admin::table).values(ad)
            .returning(admin::all_columns)
            .get_result::<Admin>(&mut db.0).await;

    match res {
        Ok(x) => {
            Ok(x)
        }
        Err(e) => {
            Err(e.to_string())
        }
    }
}


pub async fn get_admins(db: &mut PoolPost<'_>) -> Result<Vec<Admin>, String> {
    match admin::dsl::admin.load::<Admin>(&mut db.0).await {
        Ok(x) => {
            Ok(x)
        }
        Err(e) => {
            Err(e.to_string())
        }
    }
}

pub async fn delete_admin(db: &mut PoolPost<'_>, id: i32) -> Result<Admin,String> {
    let idd = admin::table.filter(admin::dsl::id.eq(id));

    let res = diesel::delete(idd).returning(admin::all_columns)
        .get_result::<Admin>(&mut db.0).await;

    match res {
        Ok(e) => {
            Ok(e)
        }
        Err(e) => Err(e.to_string())
    }
}


pub async fn update_admin_repo(db: &mut PoolPost<'_>, id: i32,values :NewAdmin) -> Result<Admin,String> {
    let idd = admin::table.filter(admin::dsl::id.eq(id));

    let res = diesel::update(idd)
        .set(
            values
    ).returning(admin::all_columns)
        .get_result::<Admin>(&mut db.0).await;

    match res {
        Ok(e) => {
            Ok(e)
        }
        Err(e) => Err(e.to_string())
    }
}