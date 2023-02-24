# Ledger stats

## Description

A tool that calculates the statistic from an acyclic, directed, unweighted graph.
The following stats are supported:

- an average number of inward references per vertex.
- an average size of root depth per vertex.
- an average number of nodes per root path.

## Building

```sh
cargo build

```

## Usage

```sh
./ledger database.txt

```

or (the `database.txt` is a default path)

```sh
cargo run
```

For help, please see:

```sh
./ledger  --help

```

## Testing

The package has two types of tests:

- unit

```sh
cargo test --lib
```

- integration:

```sh
cargo test tests
```

## Decisions

| Decision | Reason |
|----------|--------|
| Graph | The algorithm assumes that every vertex in the graph is reachable from the root vertex with ID `1`|
| Performance | The application is designed as a compromise between performance and maintainability. Additional abstractions introduce performance and processing overhead, but in return they provide better readability|
| Modularity | Components have been created to be loosely coupled. The `Graph` structure is just a facade that combines all functionalities together|
| Introduction of `VertexWithStats` | The structure that holds the `Vertex` has been introduced to keep the meta-logic separate from the graph logic. Thanks to that, `Vertex` can be used for other purposes. Obviously, the conversion between `Vertex` and `VertexWithStats` requires additional processing, but this could be easily eliminated by creating the `VertexWithStats` when reading from the file|
| Validation for timestamps is not implemented | It could be expected that vertices should appear in the database sorted by timestamp. As the instruction was not clear about this, the validation for timestamps has not been implemented |
