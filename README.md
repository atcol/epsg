# epsg

[<img alt="Master branch build" src="https://github.com/atcol/epsg/actions/workflows/master.yml/badge.svg?branch=master">](https://github.com/atcol/epsg/actions)
[<img alt="License - MIT" src="https://img.shields.io/badge/license-MIT-green">](https://github.com/atcol/epsg/blob/master/LICENSE)
[<img alt="EPSG on docs.rs" src="https://img.shields.io/badge/docs.rs-epsg-66c2a5?&logoColor=white&logo=data:image/svg+xml;base64,PHN2ZyByb2xlPSJpbWciIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyIgdmlld0JveD0iMCAwIDUxMiA1MTIiPjxwYXRoIGZpbGw9IiNmNWY1ZjUiIGQ9Ik00ODguNiAyNTAuMkwzOTIgMjE0VjEwNS41YzAtMTUtOS4zLTI4LjQtMjMuNC0zMy43bC0xMDAtMzcuNWMtOC4xLTMuMS0xNy4xLTMuMS0yNS4zIDBsLTEwMCAzNy41Yy0xNC4xIDUuMy0yMy40IDE4LjctMjMuNCAzMy43VjIxNGwtOTYuNiAzNi4yQzkuMyAyNTUuNSAwIDI2OC45IDAgMjgzLjlWMzk0YzAgMTMuNiA3LjcgMjYuMSAxOS45IDMyLjJsMTAwIDUwYzEwLjEgNS4xIDIyLjEgNS4xIDMyLjIgMGwxMDMuOS01MiAxMDMuOSA1MmMxMC4xIDUuMSAyMi4xIDUuMSAzMi4yIDBsMTAwLTUwYzEyLjItNi4xIDE5LjktMTguNiAxOS45LTMyLjJWMjgzLjljMC0xNS05LjMtMjguNC0yMy40LTMzLjd6TTM1OCAyMTQuOGwtODUgMzEuOXYtNjguMmw4NS0zN3Y3My4zek0xNTQgMTA0LjFsMTAyLTM4LjIgMTAyIDM4LjJ2LjZsLTEwMiA0MS40LTEwMi00MS40di0uNnptODQgMjkxLjFsLTg1IDQyLjV2LTc5LjFsODUtMzguOHY3NS40em0wLTExMmwtMTAyIDQxLjQtMTAyLTQxLjR2LS42bDEwMi0zOC4yIDEwMiAzOC4ydi42em0yNDAgMTEybC04NSA0Mi41di03OS4xbDg1LTM4Ljh2NzUuNHptMC0xMTJsLTEwMiA0MS40LTEwMi00MS40di0uNmwxMDItMzguMiAxMDIgMzguMnYuNnoiPjwvcGF0aD48L3N2Zz4K">](https://docs.rs/epsg/latest/epsg/)

[EPSG](https://epsg.org/) reference data and common types for working with Coordinate Reference Systems.

The Rust structures in this library are generated from the [EPSG Dataset](https://iogp.georepository.com/terms-of-use.html) version 10.033.

See the [documentation](https://docs.rs/epsg/latest/epsg/) for more information.

## Generating the Structures

1. Download the dataset and extract the archive
2. Clone this repository
3. Change the path in the volume in `docker-compose.yml` to point to the directory holding the EPSG dataset
4. Start the postgres container:
```
  docker-compose up
```
5. Run: `export PG_STR="host=localhost user=postgres password=postgres dbname=epsg" cargo build --release`.

## Contributions

Contributions are welcome. All submissions will be distributed under the MIT license if accepted.

## Terms

See the terms of use for the EPSG Dataset.
