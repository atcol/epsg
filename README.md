# epsg

[EPSG](https://epsg.org/) reference data and common types for working with Coordinate Reference Systems.

The Rust structures in this library are generated from the [EPSG Dataset](https://iogp.georepository.com/terms-of-use.html).

## Generating the Structures

1. Download the dataset and extract the archive
2. Clone this repository
3. Change the path in the volume in `docker-compose.yml` to point to this directory
4. Start the postgres container:
```
  docker-compose up
```
5. Run this tool: `cargo run`.

## Contributions

Contributions are welcome. All submissions will be distributed under the MIT license if accepted.

## Terms

See the terms of use for the EPSG Dataset.
