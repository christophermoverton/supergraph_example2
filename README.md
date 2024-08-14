
# MongoDB Supergraph GraphQL API Example in Rust

This project is an example of a GraphQL API implemented in Rust, with MongoDB as the backend database. It demonstrates how to set up a GraphQL server using `async-graphql` and `actix-web`, and how to connect to a MongoDB database using the `mongodb` crate.

## Project Structure

```
.
├── Cargo.toml           # Project dependencies and metadata
├── src
│   ├── db.rs            # MongoDB database connection setup
│   ├── main.rs          # Entry point for the Actix web server
│   └── schema
│       ├── mod.rs       # Main schema setup and subgraph integration
│       ├── subgraph1.rs # Subgraph 1: User-related queries
│       └── subgraph2.rs # Subgraph 2: Product-related queries
└── README.md            # This README file
```



This project is a Rust-based GraphQL API that integrates with a MongoDB database. It demonstrates how to perform CRUD operations on `User` and `Product` entities using GraphQL mutations and queries. The project is built using the following technologies:

- **Rust**
- **async-graphql**: A GraphQL server implementation in Rust.
- **mongodb**: An official MongoDB driver for Rust.
- **tokio**: An asynchronous runtime for Rust.

## Features

- **Create, Read, and Delete Users**: GraphQL mutations and queries to manage user entities.
- **Create, Read, and Delete Products**: GraphQL mutations and queries to manage product entities.
- **MongoDB Integration**: All data is stored and retrieved from a MongoDB database.

## Prerequisites

- **Rust**: Ensure you have Rust installed. If not, you can install it from [here](https://www.rust-lang.org/).
- **MongoDB**: A running MongoDB instance. You can download and install MongoDB from [here](https://www.mongodb.com/try/download/community).
- **Cargo**: Rust's package manager.

## Getting Started

### 1. Clone the Repository

```sh
git clone https://github.com/your-username/mongodb_graphql_example.git
cd mongodb_graphql_example
```

### 2. Set Up MongoDB

Ensure your MongoDB database is running locally:

- **Default URI**: `mongodb://localhost:27017`
- **Database Name**: `graphql_db`

You can adjust the database URI in the `db.rs` file if necessary.

### 3. Build the Project

Use Cargo to build the project:

```sh
cargo build
```

### 4. Run the Project

Start the GraphQL server:

```sh
cargo run
```

The server will start at `http://localhost:8000/graphql`.

### 5. Test the API

You can use tools like [Postman](https://www.postman.com/), [Insomnia](https://insomnia.rest/), or the built-in GraphQL Playground to interact with the API.

### Example GraphQL Queries and Mutations

#### Create a User

```graphql
mutation {
  createUser(input: {id: "1", name: "John Doe", email: "john.doe@example.com"}) {
    id
    name
    email
  }
}
```

#### Query a User

```graphql
query {
  user(id: "1") {
    id
    name
    email
  }
}
```

#### Delete a User

```graphql
mutation {
  deleteUser(id: "1")
}
```

#### Create a Product

```graphql
mutation {
  createProduct(input: {id: "1", name: "Laptop", price: 999.99}) {
    id
    name
    price
  }
}
```

#### Query a Product

```graphql
query {
  product(id: "1") {
    id
    name
    price
  }
}
```

#### Delete a Product

```graphql
mutation {
  deleteProduct(id: "1")
}
```

### 6. Verify Data in MongoDB

You can verify the data stored in MongoDB by connecting to your MongoDB instance using the MongoDB Shell or a GUI like MongoDB Compass:

- **Using MongoDB Shell:**
  ```sh
  mongo
  use graphql_db
  db.users.find({})
  db.products.find({})
  ```

- **Using MongoDB Compass:**
  - Connect to `mongodb://localhost:27017`.
  - Browse the `graphql_db` database and view the `users` and `products` collections.

## Project Structure

- `src/main.rs`: The main entry point of the application.
- `src/schema/subgraph1.rs`: Defines GraphQL types and resolvers for `User` operations.
- `src/schema/subgraph2.rs`: Defines GraphQL types and resolvers for `Product` operations.
- `src/db.rs`: Handles the connection to the MongoDB database.

## Dependencies

This project uses the following Rust crates:

- `async-graphql`: For handling GraphQL requests.
- `mongodb`: For interacting with the MongoDB database.
- `tokio`: For asynchronous runtime.

## Troubleshooting

- **MongoDB Connection Issues**: Ensure your MongoDB instance is running and accessible at the specified URI.
- **GraphQL Errors**: Use the error messages provided by the GraphQL API to troubleshoot issues with queries or mutations.
- **Rust Compilation Errors**: Ensure all dependencies are correctly specified in `Cargo.toml`.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for more details.

## Acknowledgments

- [MongoDB](https://www.mongodb.com/) for the database.
- [async-graphql](https://github.com/async-graphql/async-graphql) for the GraphQL server library.
- [mongodb](https://github.com/mongodb/mongo-rust-driver) for the MongoDB Rust driver.

## Contributing

Contributions are welcome! Feel free to open issues or submit pull requests.



## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
