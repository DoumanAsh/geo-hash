use geo_hash::{Coordinate, CoordinateError, Codec, GeoHash};

#[test]
fn validate_good_coordinate_ctor() {
    let data = [
        (90.0, 0.0),
        (-90.0, 0.0),
        (0.0, 180.0),
        (0.0, -180.0),
    ];

    for (lat, lon) in data {
        let coord = Coordinate::new(lat, lon);
        assert_eq!(coord.latitude(), lat);
        assert_eq!(coord.longitude(), lon);
    }
}

#[test]
fn validate_invalid_latitude_check() {
    let data = [
        (90.1, 0.0),
        (-90.1, 0.0),
    ];

    for (lat, lon) in data {
        let error = Coordinate::try_new(lat, lon).expect_err("should fail with invalid latitude");
        assert_eq!(error, CoordinateError::InvalidLatitude(lat));
    }
}

#[test]
fn validate_invalid_longitude_check() {
    let data = [
        (0.0, 180.1),
        (0.0, -180.1),
    ];

    for (lat, lon) in data {
        let error = Coordinate::try_new(lat, lon).expect_err("should fail with invalid longitude");
        assert_eq!(error, CoordinateError::InvalidLongitude(lat));
    }
}

#[test]
fn validate_geohash_encoding() {
    type CODEC = Codec::<12>;

    let data = [
        ("sb54v4xk18jg",0.497818518f64,38.198505253f64),
        ("00upjeyjb54g",-84.529178182,-174.125057287),
        ("kkfwu0udnhxm",-17.090238388,14.947853282),
        ("gp2cx4ywjhyj",86.06108453,-43.628546008),
        ("h0g4tmrp0cut",-85.311745894,4.459114168),
        ("v471duxnbttv",57.945830289,49.349241965),
        ("h78n33z47k3j",-69.203844118,11.314685805),
        ("gvtw7yer4bhh",77.073040753,-3.346243298),
        ("0fqwy0pgxxwj",-76.156584583,-136.834730089),
        ("dj53wuppzfrx",28.411988257,-85.123100792),
        ("rmbn08wvubcz",-11.597823607,146.281448853),
        ("qtjucvmdjyhe",-16.010823784,120.67064801),
        ("9wkcmxuswb2j",35.419323354,-105.572143468),
        ("9745rntct4xh",17.482266365,-120.621762327),
        ("fdhmnve3sz53",57.159413941,-61.222135062),
        ("1w24t6m5t8ew",-54.391332719,-112.262179799),
        ("bxgshxvdqnzj",89.33987042,-152.372551026),
        ("yhuw9x0p1mt1",72.901011648,96.39410362),
        ("800000000000",0.0,-180.0),
    ];
    for (expected, lat, lon) in data {
        let result = CODEC::encode(Coordinate::new(lat, lon));
        let expected = GeoHash::from_str(expected).expect("to create geo hash");
        assert_eq!(result, expected, "Coordinate({lat}, {lon}) has geohash='{result}' but expected='{expected}'");
    }
}

fn f64_round(x: f64, decimals: u32) -> f64 {
    let factor = 10.0_f64.powi(decimals as i32);
    (x * factor).round() / factor
}

#[track_caller]
fn compare_coordinates(left: Coordinate, expected: Coordinate) {
    assert_eq!(f64_round(left.latitude(), 4), f64_round(expected.latitude(), 4), "Expected latitude='{}' but got '{}'", expected.latitude(), left.latitude());
    assert_eq!(f64_round(left.longitude(), 4), f64_round(expected.longitude(), 4), "Expected longitude='{}' but got '{}'", expected.longitude(), left.longitude());
}

#[test]
fn validate_geohash_decoding() {
    let data = [
        ("ww8p1r4t8", Coordinate::new(37.83236503601074, 112.55836486816406), Coordinate::new(37.83240795135498, 112.558407783508)),
        ("9g3q", Coordinate::new(19.3359375, -99.4921875), Coordinate::new(19.51171875, -99.140625)),
        ("800000000000", Coordinate::new(0.0, -180.0), Coordinate::new(1.6763806343078613e-7, -179.99999966472387)),
    ];

    for (hash, expected_left, expected_right) in data {
        let hash = GeoHash::from_str(hash).expect("to create geo hash");
        let (left, right) = hash.decode().expect("to decode");

        compare_coordinates(left, expected_left);
        compare_coordinates(right, expected_right);
    }
}
