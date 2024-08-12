

# Supergraph with Subgraph GraphQL Implementation in Rust

This project demonstrates the implementation of a supergraph architecture using GraphQL in Rust. It allows for the composition of multiple GraphQL subgraphs into a single supergraph, enabling a federated approach to querying across various microservices or data sources.

## Table of Contents

- [Overview](#overview)
- [Architecture](#architecture)
- [Installation](#installation)
- [Usage](#usage)
- [Examples](#examples)
- [Development](#development)
- [Contributing](#contributing)
- [License](#license)
- [Contact](#contact)

## Overview

This project implements a supergraph pattern using GraphQL. A supergraph combines multiple subgraphs, each representing a separate domain or microservice, into a single, unified GraphQL API. This is particularly useful for large-scale applications where different teams or services need to interact seamlessly.

### Key Features

- **GraphQL Subgraph Federation**: Easily manage and query across multiple subgraphs.
- **Rust-based Implementation**: Leverages the power of Rust for performance and safety.
- **Flexible Schema Composition**: Dynamically compose and merge schemas from various subgraphs.
- **Extensible Design**: Add or remove subgraphs without disrupting the supergraph.

## Architecture

The project follows a federated GraphQL architecture:

- **Supergraph**: The central API that clients interact with. It is responsible for delegating queries to the appropriate subgraphs.
- **Subgraphs**: Individual GraphQL services that represent distinct domains or data sources. Each subgraph exposes part of the overall schema.

### Example Architecture

```
          +-----------------+
          |  Client Query    |
          +-----------------+
                  |
                  v
          +-----------------+
          |   Supergraph     |
          +-----------------+
            /        |        \
           v         v         v
   +--------+   +--------+   +--------+
   |Subgraph|   |Subgraph|   |Subgraph|
   |   A    |   |   B    |   |   C    |
   +--------+   +--------+   +--------+
```

## Installation

### Prerequisites

Ensure you have Rust and Cargo installed on your machine. You can install them using the following command:

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Cloning the Repository

Clone this repository to your local machine:

```sh
git clone https://github.com/yourusername/supergraph_graphql_rust.git
cd supergraph_graphql_rust
```

### Building the Project

Compile the project with Cargo:

```sh
cargo build
```

### Running the Project

Run the project using Cargo:

```sh
cargo run
```

This will start the supergraph server, which will expose the unified GraphQL API.

## Usage

### Querying the Supergraph

Once the supergraph server is running, you can send GraphQL queries to it. The supergraph will delegate the appropriate parts of the query to the corresponding subgraphs.

Example query:

```graphql
{
  hello
  world
}


```

### Mutating the Supergraph

Example mutation:

```graphql
mutation {
  add(a: 5, b: 3)
}

```

### Configuring Subgraphs

Subgraphs are defined in the project and can be added or removed based on your needs. Each subgraph exposes part of the GraphQL schema, and the supergraph merges these schemas into a single, federated schema.

### Example Code Snippet

Here's an example of how you might define a simple subgraph in Rust:

```rust
#[derive(GraphQLObject)]
struct ServiceAData {
    id: ID,
    name: String,
}

pub struct QueryRoot;

#[graphql_object]
impl QueryRoot {
    fn service_a_data() -> ServiceAData {
        ServiceAData {
            id: "1".into(),
            name: "Service A Data".into(),
        }
    }
}
```

The supergraph would then aggregate this schema with others to form the complete API.

## Examples

The project includes several examples to help you get started with different use cases:

- **Basic Querying**: Demonstrates querying across multiple subgraphs.
- **Schema Composition**: Shows how to dynamically compose and extend schemas.
- **Error Handling**: Example of error handling across federated services.

To run an example:

```sh
cargo run --example basic_query
```

## Development

### Setting Up for Development

If you'd like to contribute or modify the project, follow these steps:

1. Fork the repository on GitHub.
2. Clone your fork:

    ```sh
    git clone https://github.com/yourusername/supergraph_graphql_rust.git
    ```

3. Create a new branch for your feature or bugfix:

    ```sh
    git checkout -b feature/your-feature
    ```

4. Make your changes and commit them:

    ```sh
    git commit -m "Add feature XYZ"
    ```

5. Push your branch to GitHub:

    ```sh
    git push origin feature/your-feature
    ```

6. Open a pull request on the original repository.

### Running Tests

This project includes unit tests and integration tests. Run them with Cargo:

```sh
cargo test
```

## Contributing

Contributions are welcome! Whether it's fixing bugs, adding new features, or improving documentation, your help is appreciated. Please follow the guidelines below:

1. Fork the project.
2. Create a branch for your feature or bugfix.
3. Write clear, concise commit messages.
4. Ensure all tests pass.
5. Submit a pull request.

Please refer to the [CONTRIBUTING.md](CONTRIBUTING.md) file for more details.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Contact

If you have any questions, suggestions, or feedback, feel free to reach out:

- **Email**: [christophermoverton@gmail.com](mailto:christophermoverton@gmail.com)

