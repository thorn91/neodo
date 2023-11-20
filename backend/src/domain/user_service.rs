use entity::user::{Entity as User, Model};
use sea_orm::{DbConn, DbErr, EntityTrait};

pub struct UserService;

impl UserService {
    pub async fn find(db: &DbConn) -> Result<Vec<Model>, DbErr> {
        let users = User::find().all(db).await?;
        Ok(users)
    }
}

// // These methods convert server errors into client errors
// impl Error {
//     pub fn client_status_and_error(&self) -> (StatusCode, ClientError) {
//         match self {
//             // Auth
//             Self::AuthFailCtxNotInRequestExt
//             | Self::AuthFailNoAuthTokenCookie
//             | Self::AuthFailCtxNotInRequestExt => (StatusCode::FORBIDDEN, ClientError::NO_AUTH),

//             // Model
//             Self::TicketDeleteFailedIdNotFound { id } => {
//                 (StatusCode::BAD_REQUEST, ClientError::INVALID_PARAMS)
//             }

//             _ => (
//                 StatusCode::INTERNAL_SERVER_ERROR,
//                 ClientError::SERVICE_ERROR,
//             ),
//         }
//     }
// }
