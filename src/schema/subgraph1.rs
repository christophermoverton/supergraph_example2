use async_graphql::{Context, Object, Result};
use redis::AsyncCommands;
use crate::models::User;
use serde_json;
//use uuid::Uuid;


#[derive(Default)]
pub struct UserQuery;

#[Object]
impl UserQuery {
    async fn get_all_users(&self, ctx: &Context<'_>) -> Result<Vec<User>> {
        let redis_client = ctx.data::<redis::Client>()?;
        let mut con = redis_client.get_async_connection().await?;
        let users: Vec<String> = con.smembers("users").await?;
        
        let users: Vec<User> = users
            .into_iter()
            .filter_map(|u| serde_json::from_str(&u).ok())
            .collect();

        Ok(users)
    }
}

#[derive(Default)]
pub struct UserMutation;

#[Object]
impl UserMutation {
    async fn add_user(&self, ctx: &Context<'_>, name: String, email: String) -> Result<User> {
        let redis_client = ctx.data::<redis::Client>()?;
        let mut con = redis_client.get_async_connection().await?;

        let user = User {
            id: uuid::Uuid::new_v4().to_string(),
            name,
            email,
        };

        let user_json = serde_json::to_string(&user)?;
        con.sadd("users", user_json).await?;
        
        Ok(user)
    }
}
