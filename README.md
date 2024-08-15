# **Supergraph Example with MySQL**

This project demonstrates how to build a GraphQL API using Rust, `async-graphql`, and MySQL. The API includes basic CRUD operations for managing `Users` and `Products` in a MySQL database.

## **Project Setup**

### **Prerequisites**

- **Rust**: Ensure you have Rust installed. You can install it from [rust-lang.org](https://www.rust-lang.org/).
- **MySQL**: You need a running MySQL instance. You can use Docker to run MySQL locally:
  ```bash
  docker run --name mysql -e MYSQL_ROOT_PASSWORD=password -e MYSQL_DATABASE=supergraph_example -p 3306:3306 -d mysql:latest
  ```
- **Cargo Dependencies**: The project uses several crates that you can install via Cargo:
  - `async-graphql`
  - `actix-web`
  - `async-graphql-actix-web`
  - `sqlx` with MySQL support
  - `dotenv`
  - `bigdecimal`

### **Database Setup**

Before running the project, ensure you have the correct database schema. You can create the `users` and `products` tables in your MySQL instance:

```sql
CREATE TABLE users (
    id INT PRIMARY KEY AUTO_INCREMENT,
    name VARCHAR(255) NOT NULL,
    email VARCHAR(255) NOT NULL UNIQUE
);

CREATE TABLE products (
    id INT PRIMARY KEY AUTO_INCREMENT,
    name VARCHAR(255) NOT NULL,
    price DECIMAL(10, 2) NOT NULL
);
```

### **Environment Variables**

Create a `.env` file in the project root to manage your database connection string:

```env
DATABASE_URL=mysql://root:password@localhost/supergraph_example
```

Make sure to replace `root`, `password`, and `supergraph_example` with your actual MySQL username, password, and database name.

### **Cargo Dependencies**

Add the following dependencies in your `Cargo.toml`:

```toml
[dependencies]
async-graphql = "4.0"
async-graphql-actix-web = "4.0"
actix-web = "4.0"
sqlx = { version = "0.6", features = ["mysql", "runtime-tokio-rustls", "macros"] }
dotenv = "0.15"
bigdecimal = "0.3"
tokio = { version = "1", features = ["full"] }
```

### **Running the Project**

To run the project, use the following command:

```bash
cargo run
```

The GraphQL API will be available at `http://localhost:8080/graphql`, and you can access the interactive GraphQL Playground at `http://localhost:8080/playground`.

## **GraphQL API**

### **User Queries and Mutations**

#### **Mutation: Add a New User**
```graphql
mutation {
  addUser(name: "John Doe", email: "john.doe@example.com") {
    id
    name
    email
  }
}
```

#### **Query: Get All Users**
```graphql
query {
  getAllUsers {
    id
    name
    email
  }
}
```

#### **Mutation: Update User Email**
```graphql
mutation {
  updateUserEmail(userId: 1, newEmail: "john.doe@newdomain.com") {
    id
    name
    email
  }
}
```

### **Product Queries and Mutations**

#### **Mutation: Add a New Product**
```graphql
mutation {
  addProduct(name: "Smartphone", price: "699.99") {
    id
    name
    price
  }
}
```

#### **Query: Get All Products**
```graphql
query {
  getAllProducts {
    id
    name
    price
  }
}
```

#### **Mutation: Update Product Price**
```graphql
mutation {
  updateProductPrice(productId: 1, newPrice: "799.99") {
    id
    name
    price
  }
}
```

## **Project Structure**

- `src/`
  - `main.rs`: The entry point of the application, responsible for setting up the Actix Web server and routes.
  - `schema/`
    - `mod.rs`: The root of the GraphQL schema, defining the queries and mutations for both users and products.
    - `subgraph1.rs`: Contains the GraphQL queries and mutations related to `Users`.
    - `subgraph2.rs`: Contains the GraphQL queries and mutations related to `Products`.
    - `scalars.rs`: Defines custom scalar types like `GraphQLBigDecimal` for handling MySQL `DECIMAL` fields.
  - `.env`: Stores the database connection string.

### **Custom Scalar for `BigDecimal`**

To handle MySQL `DECIMAL` types in GraphQL, a custom scalar is implemented in `scalars.rs`. This allows for precise handling of currency-like fields.

```rust
use async_graphql::{Scalar, Value, InputValueError, InputValueResult, ScalarType};
use bigdecimal::{BigDecimal, FromPrimitive};
use std::str::FromStr;

pub struct GraphQLBigDecimal(pub BigDecimal);

#[Scalar(name = "BigDecimal")]
impl ScalarType for GraphQLBigDecimal {
    fn parse(value: Value) -> InputValueResult<Self> {
        match value {
            Value::String(s) => BigDecimal::from_str(&s)
                .map(Self)
                .map_err(|_| InputValueError::custom("Invalid BigDecimal value")),
            Value::Number(num) => num
                .as_f64()
                .and_then(BigDecimal::from_f64)
                .map(Self)
                .ok_or_else(|| InputValueError::custom("Invalid BigDecimal value")),
            _ => Err(InputValueError::expected_type(value)),
        }
    }

    fn to_value(&self) -> Value {
        Value::String(self.0.to_string())
    }
}
```

### **Testing with GraphQL Playground**

You can use the GraphQL Playground to test all queries and mutations provided by the API. Visit the following URL after starting your server:

```
http://localhost:8080/playground
```

## **Troubleshooting**

- **Database Connection Issues**: Ensure your `.env` file is correctly set up and that your MySQL instance is running.
- **Compilation Errors**: Check that all dependencies in `Cargo.toml` are correctly installed, and make sure your Rust toolchain is up to date.

## **License**

This project is open source and available under the [MIT License](LICENSE).

---

