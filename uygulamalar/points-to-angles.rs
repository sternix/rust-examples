// https://gist.githubusercontent.com/timvisee/5a9b679a57b8323f97a0f7aea8a5dd3e/raw/27961005f92d6530124f65ecd35a53a88f915c18/points-to-angles.rs

extern crate nalgebra;

use nalgebra::geometry::Point3;

/// Given a list of points, calculate the rotation/angle the edges between points in radians.
///
/// In order to make reliable calculations the first two points are dropped in the result.
/// If a list of less than 3 points is given, an emtpy result is returned.
fn calc_point_angles(points: &Vec<Point3<f64>>) -> Vec<f64> {
    points
        .windows(2)
        .map(|p| p[1] - p[0])
        .collect::<Vec<_>>()
        .windows(2)
        .map(|e| e[0].angle(&e[1]))
        .collect()
}

/// A test covering lists of points that are too small to calculate angles.
#[test]
fn none() {
    let zero = vec![];
    let one = vec![Point3::origin(); 1];
    let two = vec![Point3::origin(); 2];

    assert_eq!(calc_point_angles(&zero), []);
    assert_eq!(calc_point_angles(&one), []);
    assert_eq!(calc_point_angles(&two), []);
}

/// A test covering a list of points in a straight line.
#[test]
fn straight() {
    let points = vec![
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(1.0, 1.0, 1.0),
        Point3::new(5.0, 5.0, 5.0),
    ];
    let expected = vec![0_f64];

    assert_eq!(calc_point_angles(&points), expected);
}

/// A test covering a list of points making straigt (90 degree) corners.
#[test]
fn corner() {
    let points = vec![
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(0.0, 5.0, 0.0),
        Point3::new(0.0, 5.0, 5.0),
        Point3::new(0.0, 0.0, 5.0),
        Point3::new(-5.0, 0.0, 5.0),
        Point3::new(-5.0, 0.0, 0.0),
        Point3::new(0.0, 0.0, 0.0),
    ];
    let expected = vec![90_f64.to_radians(); 5];

    assert_eq!(calc_point_angles(&points), expected);
}
