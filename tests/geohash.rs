use geo_hash::{Coordinate, CoordinateError, Codec, GeoHash, Bbox, Direction};

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
        let expected = GeoHash::from_str(expected);
        assert_eq!(result, expected, "Coordinate({lat}, {lon}) has geohash='{result}' but expected='{expected}'");
    }
}

fn f64_round(x: f64, decimals: u32) -> f64 {
    let factor = 10.0_f64.powi(decimals as i32);
    (x * factor).round() / factor
}

#[track_caller]
fn compare_coordinates(left: Coordinate, expected: Coordinate) {
    assert_eq!(f64_round(left.latitude(), 12), f64_round(expected.latitude(), 12), "Expected latitude='{}' but got '{}'", expected.latitude(), left.latitude());
    assert_eq!(f64_round(left.longitude(), 12), f64_round(expected.longitude(), 12), "Expected longitude='{}' but got '{}'", expected.longitude(), left.longitude());
}

#[test]
fn validate_geohash_decoding() {
    let data = [
        (
            "d",
            Coordinate::new(0.0, -90.0), Coordinate::new(45.0, -45.0),
            ["f","g","e","7","6","3","9","c"]
        ),
        (
            "db",
            Coordinate::new(0.0, -56.25), Coordinate::new(5.625, -45.0),
            ["dc","e1","e0","7p","6z","6x","d8","d9"]
        ),
        (
            "dbc",
            Coordinate::new(4.21875, -54.84375), Coordinate::new(5.625, -53.4375),
            ["dc1","dc4","dbf","dbd","db9","db8","dbb","dc0"]
        ),
        (
            "dbcd",
            Coordinate::new(4.5703125, -54.140625), Coordinate::new(4.74609375, -53.7890625),
            ["dbce","dbcg","dbcf","dbcc","dbc9","dbc3","dbc6","dbc7"]
        ),
        (
            "dbcde",
            Coordinate::new(4.658203125, -54.0087890625), Coordinate::new(4.7021484375, -53.96484375),
            ["dbcdg","dbcdu","dbcds","dbcdk","dbcd7","dbcd6","dbcdd","dbcdf"]
        ),
        (
            "dbcdef",
            Coordinate::new(4.669189453125, -53.975830078125), Coordinate::new(4.6746826171875, -53.96484375),
            ["dbcdeg","dbcds5","dbcds4","dbcds1","dbcdec","dbcde9","dbcded","dbcdee"]
        ),
        (
            "dbcdef1",
            Coordinate::new(4.669189453125, -53.974456787109375), Coordinate::new(4.670562744140625, -53.97308349609375),
            ["dbcdef3","dbcdef6","dbcdef4","dbcdecf","dbcdecc","dbcdecb","dbcdef0","dbcdef2"]
        ),
        (
            "dbcdef12",
            Coordinate::new(4.669189453125, -53.97411346435547), Coordinate::new(4.669361114501953, -53.97377014160156),
            ["dbcdef13","dbcdef19","dbcdef18","dbcdeccx","dbcdeccr","dbcdeccp","dbcdef10","dbcdef11"]
        ),
        (
            "dbcdef123",
            Coordinate::new(4.669232368469238, -53.97407054901123), Coordinate::new(4.669275283813477, -53.97402763366699),
            ["dbcdef129","dbcdef12d","dbcdef126","dbcdef124","dbcdef121","dbcdef120","dbcdef122","dbcdef128"]
        ),
        (
            "dbcdef1234",
            Coordinate::new(4.669243097305298, -53.97407054901123), Coordinate::new(4.669248461723328, -53.97405982017517),
            ["dbcdef1235","dbcdef1237","dbcdef1236","dbcdef1233","dbcdef1231","dbcdef122c","dbcdef122f","dbcdef122g"]
        ),
        (
            "dbcdef12345",
            Coordinate::new(4.669243097305298, -53.97406652569771), Coordinate::new(4.669244438409805, -53.9740651845932),
            ["dbcdef12347","dbcdef1234k","dbcdef1234h","dbcdef1231u","dbcdef1231g","dbcdef1231f","dbcdef12344","dbcdef12346"]
        ),
        (
            "dbcdef123456",
            Coordinate::new(4.669243432581425, -53.97406619042158), Coordinate::new(4.669243600219488, -53.974065855145454),
            ["dbcdef123457","dbcdef12345e","dbcdef12345d","dbcdef123459","dbcdef123453","dbcdef123451","dbcdef123454","dbcdef123455"]
        ),
        (
            "9g3q",
            Coordinate::new(19.3359375, -99.4921875), Coordinate::new(19.51171875, -99.140625),
            ["9g3r","9g3x","9g3w","9g3t","9g3m","9g3j","9g3n","9g3p"],
        ),
        (
            "ww8p1r4t8",
            Coordinate::new(37.83236503601074, 112.55836486816406), Coordinate::new(37.83240795135498, 112.5584077835083),
            ["ww8p1r4tb","ww8p1r4tc","ww8p1r4t9","ww8p1r4t3","ww8p1r4t2","ww8p1r4mr","ww8p1r4mx","ww8p1r4mz"],
        ),
        (
            "800000000000",
            Coordinate::new(0.0, -180.0), Coordinate::new(1.6763806343078613e-7, -179.99999966472387),
            ["800000000001","800000000003","800000000002","2pbpbpbpbpbr","2pbpbpbpbpbp","rzzzzzzzzzzz","xbpbpbpbpbpb","xbpbpbpbpbpc"],
        ),
    ];

    for (hash, expected_min, expected_max, expected_neighbors) in data {
        println!("hash='{hash}'");
        let hash = GeoHash::from_str(hash);
        let bbox = hash.decode_bbox().expect("to decode");
        let position = bbox.position();
        let Bbox { min, max } = bbox;

        compare_coordinates(min, expected_min);
        compare_coordinates(max, expected_max);
        assert_eq!(position.coord.latitude(), (max.latitude() + min.latitude()) / 2.0);
        assert_eq!(position.coord.longitude(), (max.longitude() + min.longitude()) / 2.0);

        let neighbors = position.neighbors(Direction::ALL).into_iter().map(|coord| GeoHash::encode(coord, hash.len()).expect("to encode")).collect::<Vec<_>>();
        assert_eq!(neighbors, expected_neighbors);
    }
}
