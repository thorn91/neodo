use entity::user::{Entity as User, Model};
use sea_orm::{DbConn, DbErr, EntityTrait};

pub struct UserService;

impl UserService {
    pub async fn find(db: &DbConn) -> Result<Vec<Model>, DbErr> {
        let users = User::find().all(db).await?;
        Ok(users)
    }
}
