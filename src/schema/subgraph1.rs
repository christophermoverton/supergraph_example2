use async_graphql::{Object, Context, Result};
use sqlx::MySqlPool;

#[derive(Default)]
pub struct Subgraph1Query;

#[Object]
impl Subgraph1Query {
    // Fetch a user by their ID
    async fn get_user_by_id(&self, ctx: &Context<'_>, user_id: i32) -> Result<User> {
        let pool = ctx.data::<MySqlPool>().unwrap();

        let row = sqlx::query_as!(User, "SELECT id, name, email FROM users WHERE id = ?", user_id)
            .fetch_one(pool)
            .await
            .map_err(|e| async_graphql::Error::new(e.to_string()))?;

        Ok(row)
    }

    // Fetch all users
    async fn get_all_users(&self, ctx: &Context<'_>) -> Result<Vec<User>> {
        let pool = ctx.data::<MySqlPool>().unwrap();

        let rows = sqlx::query_as!(User, "SELECT id, name, email FROM users")
            .fetch_all(pool)
            .await
            .map_err(|e| async_graphql::Error::new(e.to_string()))?;

        Ok(rows)
    }
}

#[derive(Default)]
pub struct Subgraph1Mutation;

#[Object]
impl Subgraph1Mutation {
    // Add a new user with name and email
    async fn add_user(&self, ctx: &Context<'_>, name: String, email: String) -> Result<User> {
        let pool = ctx.data::<MySqlPool>().unwrap();

        let result = sqlx::query!("INSERT INTO users (name, email) VALUES (?, ?)", name, email)
            .execute(pool)
            .await
            .map_err(|e| async_graphql::Error::new(e.to_string()))?;

        let id = result.last_insert_id();

        Ok(User { id: id as i32, name, email })
    }

    // Update a user's email by their ID
    async fn update_user_email(&self, ctx: &Context<'_>, user_id: i32, new_email: String) -> Result<User> {
        let pool = ctx.data::<MySqlPool>().unwrap();
    
        sqlx::query!("UPDATE users SET email = ? WHERE id = ?", new_email, user_id)
            .execute(pool)
            .await
            .map_err(|e| async_graphql::Error::new(e.to_string()))?;
    
        let row = sqlx::query_as!(
            User,
            "SELECT id, name, email FROM users WHERE id = ?",
            user_id
        )
        .fetch_one(pool)
        .await
        .map_err(|e| async_graphql::Error::new(e.to_string()))?;
    
        Ok(row)
    }
    
}

// Define the User GraphQL type
#[derive(async_graphql::SimpleObject)]
struct User {
    id: i32,
    name: String,
    email: String,
}
