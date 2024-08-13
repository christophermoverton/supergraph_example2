


# Neo4j GraphQL Example

This project is a Rust-based GraphQL API that integrates with a Neo4j database. It demonstrates how to perform CRUD operations on `User` and `Product` entities using GraphQL mutations and queries. The project is built using the following technologies:

- **Rust**
- **async-graphql**: A GraphQL server implementation in Rust.
- **neo4rs**: A Neo4j driver for Rust.
- **tokio**: An asynchronous runtime for Rust.

## Features

- **Create, Read, and Delete Users**: GraphQL mutations and queries to manage user entities.
- **Create, Read, and Delete Products**: GraphQL mutations and queries to manage product entities.
- **Neo4j Integration**: All data is stored and retrieved from a Neo4j database.

## Prerequisites

- **Rust**: Ensure you have Rust installed. If not, you can install it from [here](https://www.rust-lang.org/).
- **Neo4j**: A running Neo4j instance. You can download and install Neo4j from [here](https://neo4j.com/download/).
- **Cargo**: Rust's package manager.

## Getting Started

### 1. Clone the Repository

```sh
git clone https://github.com/your-username/neo4j_graphql_example.git
cd neo4j_graphql_example
```

### 2. Set Up Neo4j

Ensure your Neo4j database is running locally:

- **Default URI**: `127.0.0.1:7687`
- **Default Username**: `neo4j`
- **Default Password**: `neo` (or whatever password you set)

You can adjust the database credentials in the `db.rs` file if necessary.

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

### 6. Verify Data in Neo4j

You can verify the data stored in Neo4j by accessing the Neo4j Browser at `http://localhost:7474` and running queries like:

```cypher
MATCH (u:User) RETURN u;
MATCH (p:Product) RETURN p;
```

## Project Structure

- `src/main.rs`: The main entry point of the application.
- `src/schema/subgraph1.rs`: Defines GraphQL types and resolvers for `User` operations.
- `src/schema/subgraph2.rs`: Defines GraphQL types and resolvers for `Product` operations.
- `src/db.rs`: Handles the connection to the Neo4j database.

## Dependencies

This project uses the following Rust crates:

- `async-graphql`: For handling GraphQL requests.
- `neo4rs`: For interacting with the Neo4j database.
- `tokio`: For asynchronous runtime.

## Troubleshooting

- **Neo4j Connection Issues**: Ensure your Neo4j instance is running and accessible at the specified URI.
- **GraphQL Errors**: Use the error messages provided by the GraphQL API to troubleshoot issues with queries or mutations.
- **Rust Compilation Errors**: Ensure all dependencies are correctly specified in `Cargo.toml`.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for more details.

## Acknowledgments

- [Neo4j](https://neo4j.com/) for the graph database.
- [async-graphql](https://github.com/async-graphql/async-graphql) for the GraphQL server library.
- [neo4rs](https://github.com/neo4j/neo4j-rust) for the Neo4j Rust driver.

## Contributing

Contributions are welcome! Feel free to open issues or submit pull requests.

