//!Geohash library
//!
//!In simple terms geohash converts point into hash that can uniquely identify cell on the map of the world
//!Hash length will determines size of the cell with following table illustrating approximate sizes
//!
//! | Hash Len | Width    | Height  |
//! |----------|:-------: |--------:|
//! | 1        | <=5km    | 5km     |
//! | 2        | <=1.250km| 625km   |
//! | 3        | <=156km  | 156km   |
//! | 4        | <=39.1km | 19.5km  |
//! | 5        | <=4.89km | 4.89km  |
//! | 6        | <=1.22km | 0.61km  |
//! | 7        | <=153m   | 153m    |
//! | 8        | <=38.2m  | 19.1m   |
//! | 9        | <=4.77m  | 4.77m   |
//! | 10       | <=1.19m  | 0.596m  |
//! | 11       | <=149mm  | 149mm   |
//! | 12       | <=37.2mm | 18.6mm  |
//!
//! Note, width becomes smaller depending on how far coordinate is from equator
//!
//! The important property of the resulting hash is that the closer coordinates are, the bigger common prefix is between two hashes.
//!
//! ## Features
//!
//! `serde` - Implements serde interface on `GeoHash`
//!
//! ## Usage
//!
//! ### Encode with static hash size
//!
//! You can use [Codec] to ensure encoding will never fail at compile time by supplying it with
//! length of the hash you desire
//!
//! ```rust
//! use geo_hash::Coordinate;
//! type CODEC = geo_hash::Codec::<9>;
//!
//! let position = Coordinate::try_new(43.2203, 142.8635).expect("valid GPS coordinates");
//! let hash = CODEC::encode(position);
//! assert_eq!(hash, "xpttfun02");
//! ```
//!
//!### Encode with dynamic hash size
//!
//! You can use [GeoHash::encode] when you cannot know size of the hash at compile time, which will fail if hash length is not within range of `1..=12`
//!
//! ```rust
//! use geo_hash::{GeoHash, Coordinate};
//!
//! let position = Coordinate::try_new(43.2203, 142.8635).expect("valid GPS coordinates");
//! let hash = GeoHash::encode(position, 9).expect("to encode");
//! assert_eq!(hash, "xpttfun02");
//! ```
//!### Decode hash to determine approximate position
//!
//! When you only have textual hash, you can determine its [bounding box](Bbox) or as approximation [position](GeoHashPosition) within this bounding box
//!
//! ```rust
//! use geo_hash::{GeoHash, Coordinate};
//!
//! const COORD: Coordinate =  Coordinate::new(43.22027921676636, 142.86348581314087);
//!
//! //This function only checks length
//! let hash = GeoHash::try_from_str("xpttfun02").expect("valid geohash");
//! assert_eq!(hash, "xpttfun02");
//! //This function will check validity of hash itself
//! let bbox = hash.decode_bbox().expect("we should have valid hash here");
//! assert_eq!(bbox.min(), Coordinate::new(43.22025775909424, 142.86346435546875));
//! assert_eq!(bbox.max(), Coordinate::new(43.22030067443848, 142.863507270813));
//! let position = bbox.position();
//! assert_eq!(position.coordinates(), COORD);
//! ```


#![no_std]
#![warn(missing_docs)]
#![allow(clippy::style)]

use core::fmt;

const MAX_LAT: f64 = 90.0;
const MIN_LAT: f64 = -90.0;
const MAX_LON: f64 = 180.0;
const MIN_LON: f64 = -180.0;

#[cfg(feature = "serde")]
mod serde;
mod codec;
mod math;
pub use codec::{Codec, GeoHash, DecodeError};

#[derive(Debug, Clone, Copy, PartialEq)]
///Possible errors when creating [Coordinate]
pub enum CoordinateError {
    ///Indicates invalid latitude value outside of allowed bounds
    InvalidLatitude(f64),
    ///Indicates invalid longitude value outside of allowed bounds
    InvalidLongitude(f64),
}

impl fmt::Display for CoordinateError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidLatitude(value) => fmt.write_fmt(format_args!("Invalid latitude='{value}'. Allowed values are {MIN_LAT}..={MAX_LAT}")),
            Self::InvalidLongitude(value) => fmt.write_fmt(format_args!("Invalid longitude='{value}'. Allowed values are {MIN_LON}..={MAX_LON}")),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
///Coordinate
pub struct Coordinate {
    latitude: f64,
    longitude: f64,
}

impl Coordinate {
    #[inline(always)]
    ///Creates new instance with panic in case of error
    pub const fn new(latitude: f64, longitude: f64) -> Self {
        match Self::try_new(latitude, longitude) {
            Ok(result) => result,
            Err(_) => panic!("Invalid coordinates"),
        }
    }

    #[inline(always)]
    ///Creates new instance verifying coordinates are valid
    pub const fn try_new(latitude: f64, longitude: f64) -> Result<Self, CoordinateError> {
        if latitude.is_nan() || latitude < MIN_LAT || latitude > MAX_LAT {
            Err(CoordinateError::InvalidLatitude(latitude))
        } else if longitude.is_nan() || longitude < MIN_LON || longitude > MAX_LON {
            Err(CoordinateError::InvalidLongitude(latitude))
        } else {
            Ok(Self {
                latitude,
                longitude
            })
        }
    }

    #[inline(always)]
    ///Returns latitude
    pub const fn latitude(&self) -> f64 {
        self.latitude
    }

    #[inline(always)]
    ///Returns longitude
    pub const fn longitude(&self) -> f64 {
        self.longitude
    }
}

//Convenience impls to handle automatic deref on references
impl PartialEq<Coordinate> for &'_ Coordinate {
    #[inline(always)]
    fn eq(&self, other: &Coordinate) -> bool {
        PartialEq::eq(*self, other)
    }
}

impl PartialEq<&Coordinate> for Coordinate {
    #[inline(always)]
    fn eq(&self, other: &&Coordinate) -> bool {
        PartialEq::eq(self, *other)
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
///Geohash position with margins of error
pub struct GeoHashPosition {
    ///Position itself
    pub coord: Coordinate,
    ///Latitude error
    pub lat_err: f64,
    ///Longitude error
    pub lon_err: f64,
}

impl GeoHashPosition {
    #[inline(always)]
    ///Returns self's coordinates
    pub const fn coordinates(&self) -> Coordinate {
        self.coord
    }

    ///Retrieves neighboring coordinates in specified `direction`
    pub const fn neighbor(&self, direction: Direction) -> Coordinate {
        let (direction_lat, direction_lon) = direction.to_lat_lon();
        Coordinate {
            longitude: math::rem_euclid((self.coord.longitude + 2f64 * self.lon_err.abs() * direction_lon) + MAX_LON, MAX_LON * 2.0) - MAX_LON,
            latitude: math::rem_euclid((self.coord.latitude + 2f64 * self.lat_err.abs() * direction_lat) + MAX_LAT, MAX_LAT * 2.0) - MAX_LAT,
        }
    }

    ///Retrieves neighbors for specified `directions`
    pub const fn neighbors<const N: usize>(&self, directions: [Direction; N]) -> [Coordinate; N] {
        let mut result = [Coordinate::new(0.0, 0.0); N];

        let mut idx = 0;
        while idx < directions.len() {
            result[idx] = self.neighbor(directions[idx]);
            idx += 1;
        }

        result
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
///Geo Bounded box describing geohash cell
pub struct Bbox {
    ///Min position of the box (top left)
    pub min: Coordinate,
    ///Max position of the box (bottom right)
    pub max: Coordinate,
}

impl Bbox {
    #[inline(always)]
    ///Returns min position of the box (top left)
    pub const fn min(&self) -> &Coordinate {
        &self.min
    }

    #[inline(always)]
    ///Returns max position of the box (bottom right)
    pub const fn max(&self) -> &Coordinate {
        &self.max
    }

    #[inline(always)]
    ///Calculates [GeoHashPosition]
    pub const fn position(&self) -> GeoHashPosition {
        let min = self.min;
        let max = self.max;
        GeoHashPosition {
            coord: Coordinate {
                latitude: (min.latitude + max.latitude) / 2.0,
                longitude: (min.longitude + max.longitude) / 2.0,
            },
            lat_err: (max.latitude - min.latitude) / 2.0,
            lon_err: (max.longitude - min.longitude) / 2.0,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
///Direction to select neighbor
pub enum Direction {
    ///North
    N,
    ///North East
    NE,
    ///East
    E,
    ///South East
    SE,
    ///South
    S,
    ///South West
    SW,
    ///West
    W,
    ///North West
    NW,
}

impl Direction {
    ///All directions in single array
    pub const ALL: [Self; 8] = [Self::N, Self::NE, Self::E, Self::SE, Self::S, Self::SW, Self::W, Self::NW];

    const fn to_lat_lon(self) -> (f64, f64) {
        match self {
            Direction::SW => (-1.0, -1.0),
            Direction::S => (-1.0, 0.0),
            Direction::SE => (-1.0, 1.0),
            Direction::W => (0.0, -1.0),
            Direction::E => (0.0, 1.0),
            Direction::NW => (1.0, -1.0),
            Direction::N => (1.0, 0.0),
            Direction::NE => (1.0, 1.0),
        }
    }
}
