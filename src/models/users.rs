use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct UserModel {
    pub id: String,
    pub name: String,
    pub email: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateUserBody {
    pub name: String,
    pub email: String,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateUserBody {
    pub name: Option<String>,
    pub email: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DeleteUserBody {
    pub id: String,
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize)]
pub struct FilteredUser {
    pub id: String,
    pub name: String,
    pub email: String,
}

#[derive(Serialize, Debug)]
pub struct UserData {
    pub user: FilteredUser,
}

#[derive(Serialize, Debug)]
pub struct UserResponse {
    pub status: String,
    pub data: UserData,
}

#[derive(Debug, Deserialize)]
pub struct RegisterUserParams {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginUserParams {
    pub authorization_code: String,
}
// pub struct LoginUserParams {
//     pub email: String,
//     pub password: String,
// }

impl UserModel {
    pub fn user_to_response(user: &UserModel) -> FilteredUser {
        FilteredUser {
            id: user.id.to_owned(),
            name: user.name.to_owned(),
            email: user.email.to_owned(),
        }
    }
}
