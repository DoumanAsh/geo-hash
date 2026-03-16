# geo-hash

[![Rust](https://github.com/DoumanAsh/geo-hash/actions/workflows/rust.yml/badge.svg)](https://github.com/DoumanAsh/geo-hash/actions/workflows/rust.yml)
[![Crates.io](https://img.shields.io/crates/v/geo-hash.svg)](https://crates.io/crates/geo-hash)
[![Documentation](https://docs.rs/geo-hash/badge.svg)](https://docs.rs/crate/geo-hash/)

[GeoHash](https://en.wikipedia.org/wiki/Geohash) implementation

Geohash converts point into hash that can uniquely identify cell on the map of the world
Hash length will determines size of the cell with following table illustrating approximate sizes

| Hash Len | Width    | Height  |
|----------|:-------: |--------:|
| 1        | <=5km    | 5km     |
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

## Usage

### Encode with static hash size

You can use `Codec` to ensure encoding will never fail at compile time by supplying it with
length of the hash you desire

```rust
use geo_hash::Coordinate;
type CODEC = geo_hash::Codec::<9>;

let position = Coordinate::try_new(43.2203, 142.8635).expect("valid GPS coordinates");
let hash = CODEC::encode(position);
assert_eq!(hash, "xpttfun02");
```

## Encode with dynamic hash size

You can use `GeoHash::encode` when you cannot know size of the hash at compile time, which will fail if hash length is not within range of `1..=12`

```rust
use geo_hash::{GeoHash, Coordinate};

let position = Coordinate::try_new(43.2203, 142.8635).expect("valid GPS coordinates");
let hash = GeoHash::encode(position, 9).expect("to encode");
assert_eq!(hash, "xpttfun02");
```
## Decode hash to determine approximate position

When you only have textual hash, you can determine its bounding box or as approximation position within this bounding box

```rust
use geo_hash::{GeoHash, Coordinate};

const COORD: Coordinate =  Coordinate::new(43.22027921676636, 142.86348581314087);

//This function only checks length
let hash = GeoHash::try_from_str("xpttfun02").expect("valid geohash");
assert_eq!(hash, "xpttfun02");
//This function will check validity of hash itself
let bbox = hash.decode_bbox().expect("we should have valid hash here");
assert_eq!(bbox.min(), Coordinate::new(43.22025775909424, 142.86346435546875));
assert_eq!(bbox.max(), Coordinate::new(43.22030067443848, 142.863507270813));
let position = bbox.position();
assert_eq!(position.coordinates(), COORD);
```
