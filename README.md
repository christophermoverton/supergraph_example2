
# Neo4j GraphQL Example in Rust

This project is an example of a GraphQL API implemented in Rust, integrated with a Neo4j database. The API demonstrates how to set up a modular GraphQL schema with subgraphs, connect to a Neo4j database, and perform basic queries.

## Project Structure


.
├── Cargo.toml           # Project dependencies and metadata
├── src
│   ├── db.rs            # Neo4j database connection setup
│   ├── main.rs          # Entry point for the Actix web server
│   └── schema
│       ├── mod.rs       # Main schema setup and subgraph integration
│       ├── subgraph1.rs # Subgraph 1: User-related queries
│       └── subgraph2.rs # Subgraph 2: Product-related queries
└── README.md            # This README file


## Features

- **GraphQL API**: Exposes a GraphQL API using `async-graphql`.
- **Subgraphs**: Modular GraphQL schema split into subgraphs (`User` and `Product`).
- **Neo4j Integration**: Connects to a Neo4j database using `neo4rs` for data storage and retrieval.
- **Actix Web Server**: Runs the GraphQL API on an Actix web server.

## Prerequisites

- **Rust**: Ensure you have Rust installed. You can install Rust using [rustup](https://rustup.rs/).
- **Neo4j**: You need a running instance of Neo4j. Install Neo4j [here](https://neo4j.com/download/).

## Installation

1. **Clone the Repository**

   ```bash
   git clone -b neo4j_example https://github.com/yourusername/your-repo-name.git
   cd your-repo-name
   ```

2. **Install Dependencies**

   Run the following command to install the project dependencies:

   ```bash
   cargo build
   ```

3. **Set Up Neo4j**

   Ensure your Neo4j database is running. By default, this example assumes Neo4j is running locally at `bolt://localhost:7687` with the username `neo4j` and password `password`.

   You can change these settings in `src/db.rs`.

4. **Insert Test Data into Neo4j**

   Use the Neo4j browser or a Cypher script to insert some test data:

   ```cypher
   CREATE (u:User {id: '1', name: 'Alice', email: 'alice@example.com'});
   CREATE (p:Product {id: '1', name: 'Laptop', price: 999.99});
   ```

## Running the Server

To run the server, use:

```bash
cargo run
```

The server will start on `http://127.0.0.1:8080`. The GraphQL Playground will be accessible at `http://127.0.0.1:8080/playground`.

## Example Queries

### Fetch a User by ID

```graphql
{
  user(id: "1") {
    id
    name
    email
  }
}
```

### Fetch a Product by ID

```graphql
{
  product(id: "1") {
    id
    name
    price
  }
}
```

## Stopping the Server

To stop the server, find the process ID (PID) and kill it:

```bash
ps aux | grep your-binary-name
kill <PID>
```

## Contributing

Feel free to fork the repository, submit issues, and create pull requests.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
