//Based on https://mmcloughlin.com/posts/geohash-assembly

use core::{hash, fmt, ptr};
use super::{Bbox, Coordinate, MAX_LAT, MAX_LON};

const CHUNK_BITS_SIZE: usize = 5;
const MAX_LEN: usize = 12;
const BASE32_CHARS: [char; 32] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9',
    'b', 'c', 'd', 'e', 'f', 'g', 'h', 'j', 'k', 'm',
    'n', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
];

const fn build_reverse_table(chars: [char; 32]) -> [u8; 256] {
    let mut table = [u8::MAX; 256];

    let mut idx = 0;
    while idx < chars.len() {
        table[chars[idx] as usize] = idx as u8;
        idx += 1
    }

    table
}

const REVERSE_BASE32: [u8; 256] = build_reverse_table(BASE32_CHARS);

#[derive(Copy, Clone, Debug)]
///Geohash decoding error
pub struct DecodeError {
    position: u8,
    character: u8,
}

impl fmt::Display for DecodeError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self { position, character } = self;
        fmt.write_fmt(format_args!("Encountered invalid character byte='{character}' at position={position}"))
    }
}

const fn encode(coord: Coordinate, len: usize) -> GeoHash {
    //combines two u32 into single u64 with their bits interleaved
    #[inline]
    const fn interleave(x: u32, y: u32) -> u64 {
        //spread takes a u32 and deposits its bits into the evenbit positions of a u64
        #[inline]
        const fn spread(x: u32) -> u64 {
            let mut new_x = x as u64;
            new_x = (new_x | (new_x << 16)) & 0x0000ffff0000ffff;
            new_x = (new_x | (new_x << 8)) & 0x00ff00ff00ff00ff;
            new_x = (new_x | (new_x << 4)) & 0x0f0f0f0f0f0f0f0f;
            new_x = (new_x | (new_x << 2)) & 0x3333333333333333;
            new_x = (new_x | (new_x << 1)) & 0x5555555555555555;

            new_x
        }

        spread(x) | (spread(y) << 1)
    }

    let lat32 = (coord.latitude / (MAX_LAT * 2.0)) + 1.5;
    let lon32 = (coord.longitude / (MAX_LON * 2.0)) + 1.5;

    let lat_bits = lat32.to_bits() >> 20;
    let lon_bits = lon32.to_bits() >> 20;

    let mut idx = 0;
    let mut buffer = [0u8; MAX_LEN];
    let mut interleaved_bits = interleave(lat_bits as u32, lon_bits as u32);
    while idx < len {
        //iterate over chunks of 5 bits
        let code = ((interleaved_bits >> 59) as u32) & 0x1f;
        buffer[idx] = BASE32_CHARS[code as usize] as u8;

        interleaved_bits <<= CHUNK_BITS_SIZE as u32;
        idx += 1;
    }

    GeoHash {
        buffer,
        len: len as _
    }
}

///Geohash codec
///
///Its `LEN` must be in range of `1..=12`.
///This is validated at compile time to ensure you can only use [Codec::encode] when `LEN` is correct.
pub struct Codec<const LEN: usize>;

impl<const LEN: usize> Codec<LEN> {
    const VALIDATE: () = {
        if LEN < 1 {
            panic!("Geohash len cannot less than 1")
        } else if LEN > MAX_LEN {
            panic!("Geohash len cannot greater than 12")
        }
    };

    #[inline(always)]
    ///Encodes geohash bits returning integer
    pub const fn encode(coord: Coordinate) -> GeoHash {
        let _ = Self::VALIDATE;
        encode(coord, LEN)
    }
}

#[derive(Copy, Clone)]
///GeoHash value encoded as base32 string
pub struct GeoHash {
    buffer: [u8; MAX_LEN],
    len: u8,
}

impl GeoHash {
    ///Maximum possible length of the geohash
    pub const MAX_LEN: usize = MAX_LEN;

    #[inline(always)]
    ///Creates `len` long geohash for specified coordinates `coord`.
    ///
    ///Returns `None` if `len` is not within `1..=12` range
    ///
    ///Prefer to use [Codec] to ensure `len` is valid at compile time when possible
    pub const fn encode(coord: Coordinate, len: usize) -> Option<Self> {
        if len >= 1 && len <= Self::MAX_LEN {
            Some(encode(coord, len))
        } else {
            None
        }
    }

    #[inline(always)]
    ///Creates geohash from string, validating its length within `1..=12` with panic in case of failure
    pub const fn from_str(text: &str) -> Self {
        match Self::try_from_str(text) {
            Some(result) => result,
            None => panic!("'text' is not valid geohash")
        }
    }

    ///Creates geohash from string, validating its length within `1..=12`
    pub const fn try_from_str(text: &str) -> Option<Self> {
        if text.len() >= 1 && text.len() <= 12 {
            let mut buffer = [0u8; MAX_LEN];
            unsafe {
                ptr::copy_nonoverlapping(text.as_ptr(), buffer.as_mut_ptr(), text.len());
            }
            Some(Self {
                buffer,
                len: text.len() as _
            })
        } else {
            None
        }
    }

    #[inline(always)]
    ///Returns geohash length
    pub const fn len(&self) -> usize {
        self.len as _
    }

    #[inline(always)]
    ///Gets text representation of the hash
    pub const fn as_str(&self) -> &str {
        unsafe {
            core::str::from_utf8_unchecked(
                core::slice::from_raw_parts(self.buffer.as_ptr(), self.len as _)
            )
        }
    }

    ///Decodes geohash into its cell's coordinates
    pub const fn decode_bbox(&self) -> Result<Bbox, DecodeError> {
        let bits_len = self.len * CHUNK_BITS_SIZE as u8;

        let mut position = 0;
        let mut hash: u64 = 0;
        let bytes = self.as_str().as_bytes();
        while position < bytes.len() {
            let character = bytes[position];
            let ch_decoded = REVERSE_BASE32[character as usize];
            if ch_decoded == u8::MAX {
                return Err(DecodeError {
                    position: position as _,
                    character,
                })
            }

            hash <<= 5;
            hash |= ch_decoded as u64;

            position += 1;
        }

        //squashes the even bit positions of a u64 into a u32
        #[inline]
        const fn squash(x: u64) -> u32 {
            let mut new_x = x & 0x5555555555555555;
            new_x = (new_x | (new_x >> 1)) & 0x3333333333333333;
            new_x = (new_x | (new_x >> 2)) & 0x0f0f0f0f0f0f0f0f;
            new_x = (new_x | (new_x >> 4)) & 0x00ff00ff00ff00ff;
            new_x = (new_x | (new_x >> 8)) & 0x0000ffff0000ffff;
            new_x = (new_x | (new_x >> 16)) & 0x00000000ffffffff;
            new_x as u32
        }

        //Reverse interleave()
        #[inline]
        const fn deinterleave(x: u64) -> (u32, u32) {
            (squash(x), squash(x >> 1))
        }

        let full_hash = hash << (64 - bits_len);
        let (lat32, lon32) = deinterleave(full_hash);

        //reverse integer conversion
        const fn decode32(x: u32, r: f64) -> f64 {
            let p = f64::from_bits(((x as u64) << 20) | (1023 << 52));
            2.0 * r * (p - 1.0) - r
        }
        let min = Coordinate {
            latitude: decode32(lat32, MAX_LAT),
            longitude: decode32(lon32, MAX_LON),
        };

        const fn calc_geohash_diff(bits: u32) -> Coordinate {
            let lat_bits = bits / 2;
            let lon_bits = bits - lat_bits;

            let latitude = crate::math::ldexp(MAX_LAT * 2.0, -(lat_bits as i32));
            let longitude = crate::math::ldexp(MAX_LON * 2.0, -(lon_bits as i32));
            Coordinate {
                latitude,
                longitude,
            }
        }

        let max_diff = calc_geohash_diff(bits_len as u32);
        let max = Coordinate {
            latitude: min.latitude + max_diff.latitude,
            longitude: min.longitude + max_diff.longitude,
        };

        Ok(Bbox {
            min,
            max
        })
    }
}

impl fmt::Debug for GeoHash {
    #[inline(always)]
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self.as_str(), fmt)
    }
}

impl fmt::Display for GeoHash {
    #[inline(always)]
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self.as_str(), fmt)
    }
}

impl hash::Hash for GeoHash {
    #[inline(always)]
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        state.write(self.as_str().as_bytes())
    }
}

impl PartialEq for GeoHash {
    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        self.as_str() == other.as_str()
    }
}

impl PartialEq<str> for GeoHash {
    #[inline(always)]
    fn eq(&self, other: &str) -> bool {
        self.as_str() == other
    }
}

impl PartialEq<&str> for GeoHash {
    #[inline(always)]
    fn eq(&self, other: &&str) -> bool {
        self.as_str() == *other
    }
}
