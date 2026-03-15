//!Geohash library
//!
//!In simple terms geohash converts point into hash that can uniquely identify cell on the map of the world
//!Hash length will determines size of the cell with following table illustrating approximate sizes
//!
//! | Hash Len | Width    | Height  |
//! |----------|:-------: |--------:|
//! | 1        | <-5km    | 5km     |
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
