# **Supergraph GraphQL API with Redis in Rust**

This project demonstrates how to build a GraphQL API in Rust using `async-graphql`, Actix Web, and Redis as a data store. The API includes basic CRUD operations for managing `Users` and `Products`, stored as serialized JSON in Redis.

## **Project Setup**

### **Prerequisites**

- **Rust**: Ensure that Rust is installed on your machine. You can install it from [rust-lang.org](https://www.rust-lang.org/).
- **Redis**: You need a running Redis instance. You can either install Redis locally or use Docker to run a Redis container.

### **Running Redis**

You can run Redis locally using Docker:

```bash
docker run --name redis -p 6379:6379 -d redis
```

Alternatively, you can install and run Redis directly on your machine using:

```bash
redis-server
```

### **Environment Variables**

Create a `.env` file in the project root to store your Redis connection string:

```env
REDIS_URL=redis://127.0.0.1:6379/
```

This tells the application where to connect to Redis.

### **Project Dependencies**

In your `Cargo.toml`, include the following dependencies:

```toml
[dependencies]
async-graphql = "4.0"
async-graphql-actix-web = "4.0"
actix-web = "4.0"
redis = { version = "0.24", features = ["async-std-comp"] }
dotenv = "0.15"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
uuid = { version = "1.0", features = ["v4"] }
tokio = { version = "1", features = ["full"] }
```

### **Running the Application**

To start the application:

```bash
cargo run
```

This will start the Actix Web server, and the GraphQL Playground will be available at `http://localhost:8080/playground`.

### **Project Structure**

```
src/
  main.rs          # Application entry point, sets up Actix Web and routes
  models.rs        # Defines the User and Product models
  schema/
    mod.rs         # Defines the GraphQL schema and integrates Redis
    subgraph1.rs   # GraphQL queries and mutations for Users
    subgraph2.rs   # GraphQL queries and mutations for Products
  .env             # Stores the Redis connection string
Cargo.toml         # Project dependencies and metadata
```

## **GraphQL API Endpoints**

You can test the API using the GraphQL Playground. Below are some example queries and mutations.

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

### **Product Queries and Mutations**

#### **Mutation: Add a New Product**

```graphql
mutation {
  addProduct(name: "Smartphone", price: 699.99) {
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

## **Redis Integration**

The application uses Redis as a data store for both `User` and `Product` objects. These objects are stored as serialized JSON strings in Redis, using `serde_json` for serialization and deserialization.

- **Users**: Stored in a Redis set called `users`.
- **Products**: Stored in a Redis set called `products`.

### **Verifying Data in Redis**

You can verify the data in Redis by using the Redis CLI.

#### **Access Redis CLI**

If you're using Docker, run:

```bash
docker exec -it redis redis-cli
```

Or, if Redis is installed locally, run:

```bash
redis-cli
```

#### **Check Stored Users**

```bash
SMEMBERS users
```

#### **Check Stored Products**

```bash
SMEMBERS products
```

## **Error Handling and Logging**

- **Error Handling**: The project uses Rust's `Result` and `?` operator to handle errors throughout the code. Redis-related errors are handled and converted into GraphQL-friendly error messages.
- **Logging**: Check the application logs for any connection issues or runtime errors.

## **Testing**

After starting the server, you can test the API using the GraphQL Playground at `http://localhost:8080/playground`.

You can run the following steps to test the application:

1. **Add a User**: Use the `addUser` mutation to create a new user.
2. **Retrieve All Users**: Use the `getAllUsers` query to retrieve users stored in Redis.
3. **Add a Product**: Use the `addProduct` mutation to create a new product.
4. **Retrieve All Products**: Use the `getAllProducts` query to retrieve products stored in Redis.
5. **Verify in Redis**: Use the Redis CLI to verify that the data is stored correctly in Redis.

## **Troubleshooting**

- **Redis Connection**: Ensure that your Redis instance is running and that the `REDIS_URL` in your `.env` file points to the correct Redis server.
- **Environment Variables**: Double-check that your `.env` file is loaded correctly and contains the proper Redis connection string.
- **Dependencies**: Make sure that all dependencies in `Cargo.toml` are installed and up to date.

## **License**

This project is open source and available under the [MIT License](LICENSE).

