
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

## Features

- **GraphQL API**: Exposes a GraphQL API using `async-graphql`.
- **Subgraphs**: Modular GraphQL schema split into subgraphs (`User` and `Product`).
- **MongoDB Integration**: Connects to a MongoDB database using the official `mongodb` crate.
- **Actix Web Server**: Runs the GraphQL API on an Actix web server.

## Prerequisites

- **Rust**: Ensure you have Rust installed. You can install Rust using [rustup](https://rustup.rs/).
- **MongoDB**: You need a running instance of MongoDB. Install MongoDB [here](https://docs.mongodb.com/manual/installation/).

## Installation

1. **Clone the Repository**

   ```bash
   git clone -b mongodb_example https://github.com/yourusername/your-repo-name.git
   cd your-repo-name
   ```

2. **Install Dependencies**

   Run the following command to install the project dependencies:

   ```bash
   cargo build
   ```

3. **Set Up MongoDB**

   Ensure your MongoDB instance is running. By default, this example assumes MongoDB is running locally at `mongodb://localhost:27017` with the database name `mydatabase`.

   You can change these settings in `src/db.rs`.

4. **Insert Test Data into MongoDB**

   Use the MongoDB shell or any MongoDB client to insert some test data:

   ```javascript
   use mydatabase;
   db.users.insertOne({ id: "1", name: "Alice", email: "alice@example.com" });
   db.products.insertOne({ id: "1", name: "Laptop", price: 999.99 });
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

## Troubleshooting

### Common Issues

- **MongoDB Connection Issues**: Ensure MongoDB is running and accessible at the URI specified in `db.rs`.
- **Missing Data**: If you receive errors about missing data, ensure the correct documents are inserted into MongoDB.

### Logs

Check the server logs for more information if you encounter any issues. Logs can provide detailed error messages and insights into what might be going wrong.

## Contributing

Feel free to fork the repository, submit issues, and create pull requests. Contributions are welcome!

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
