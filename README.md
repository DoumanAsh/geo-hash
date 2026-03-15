# geo-hash

[![Rust](https://github.com/DoumanAsh/geo-hash/actions/workflows/rust.yml/badge.svg)](https://github.com/DoumanAsh/geo-hash/actions/workflows/rust.yml)
[![Crates.io](https://img.shields.io/crates/v/geo-hash.svg)](https://crates.io/crates/geo-hash)
[![Documentation](https://docs.rs/geo-hash/badge.svg)](https://docs.rs/crate/geo-hash/)

[GeoHash](https://en.wikipedia.org/wiki/Geohash) implementation

Geohash converts point into hash that can uniquely identify cell on the map of the world
Hash length will determines size of the cell with following table illustrating approximate sizes

| Hash Len | Width    | Height  |
|----------|:-------: |--------:|
| 1        | <-5km    | 5km     |
| 2        | <=1.250km| 625km   |
| 3        | <=156km  | 156km   |
| 4        | <=39.1km | 19.5km  |
| 5        | <=4.89km | 4.89km  |
| 6        | <=1.22km | 0.61km  |
| 7        | <=153m   | 153m    |
| 8        | <=38.2m  | 19.1m   |
| 9        | <=4.77m  | 4.77m   |
| 10       | <=1.19m  | 0.596m  |
| 11       | <=149mm  | 149mm   |
| 12       | <=37.2mm | 18.6mm  |

Note, width becomes smaller depending on how far coordinate is from equator

The important property of the resulting hash is that the closer coordinates are, the bigger common prefix is between two hashes.

## Features

`serde` - Implements serde interface on `GeoHash`
