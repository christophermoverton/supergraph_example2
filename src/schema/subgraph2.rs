use async_graphql::{Object, Context, Result};
use sqlx::MySqlPool;
use crate::schema::scalars::GraphQLBigDecimal;  // Import the custom scalar wrapper

#[derive(Default)]
pub struct Subgraph2Query;

#[Object]
impl Subgraph2Query {
    async fn get_product_by_id(&self, ctx: &Context<'_>, product_id: i32) -> Result<Product> {
        let pool = ctx.data::<MySqlPool>().unwrap();

        let row = sqlx::query!(
            "SELECT id, name, price FROM products WHERE id = ?",
            product_id
        )
        .fetch_one(pool)
        .await
        .map_err(|e| async_graphql::Error::new(e.to_string()))?;

        let product = Product {
            id: row.id,
            name: row.name,
            price: GraphQLBigDecimal(row.price),  // Use the custom wrapper
        };

        Ok(product)
    }

    async fn get_all_products(&self, ctx: &Context<'_>) -> Result<Vec<Product>> {
        let pool = ctx.data::<MySqlPool>().unwrap();

        let rows = sqlx::query!(
            "SELECT id, name, price FROM products"
        )
        .fetch_all(pool)
        .await
        .map_err(|e| async_graphql::Error::new(e.to_string()))?;

        let products = rows
            .into_iter()
            .map(|row| Product {
                id: row.id,
                name: row.name,
                price: GraphQLBigDecimal(row.price),  // Use the custom wrapper
            })
            .collect();

        Ok(products)
    }
}

#[derive(Default)]
pub struct Subgraph2Mutation;

#[Object]
impl Subgraph2Mutation {
    async fn add_product(&self, ctx: &Context<'_>, name: String, price: GraphQLBigDecimal) -> Result<Product> {
        let pool = ctx.data::<MySqlPool>().unwrap();

        let result = sqlx::query!(
            "INSERT INTO products (name, price) VALUES (?, ?)",
            name,
            price.0  // Extract the inner BigDecimal
        )
        .execute(pool)
        .await
        .map_err(|e| async_graphql::Error::new(e.to_string()))?;

        let id = result.last_insert_id();

        Ok(Product { id: id as i32, name, price })
    }

    async fn update_product_price(&self, ctx: &Context<'_>, product_id: i32, new_price: GraphQLBigDecimal) -> Result<String> {
        let pool = ctx.data::<MySqlPool>().unwrap();

        sqlx::query!(
            "UPDATE products SET price = ? WHERE id = ?",
            new_price.0,  // Extract the inner BigDecimal
            product_id
        )
        .execute(pool)
        .await
        .map_err(|e| async_graphql::Error::new(e.to_string()))?;

        Ok(format!("Product ID {} updated successfully", product_id))
    }
}

#[derive(async_graphql::SimpleObject)]
struct Product {
    id: i32,
    name: String,
    price: GraphQLBigDecimal,  // Use the custom wrapper
}
