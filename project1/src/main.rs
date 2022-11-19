fn part1() {
    const EARTH_RADIUS_IN_KILOMETERS: f64 = 6371.0;

    let apple_lat_degrees: f64 = 41.4075;
    let apple_long_degrees: f64 = -81.85111;

    let orange_lat_degrees: f64 = 40.7861;
    let orange_long_degrees: f64 = -111.9822;

    // our formula needs radians not degrees
    // we convert degrees into radians
    let apple_lat_radians: f64 = apple_lat_degrees.to_radians();
    let orange_lat_radians: f64 = orange_lat_degrees.to_radians();

    let delta_lat: f64 = (apple_lat_degrees - orange_lat_degrees).to_radians();
    let delta_long: f64 = (apple_long_degrees - orange_long_degrees).to_radians();

    let inner_central_angle = f64::powi((delta_lat / 2.0).sin(), 2)
        + apple_lat_radians.cos()
        * orange_lat_radians.cos()
        * f64::powi((delta_long / 2.0).sin(), 2);
    
    let central_angle: f64 = 2.0 * inner_central_angle.sqrt().asin();

    let distance: f64 = EARTH_RADIUS_IN_KILOMETERS * central_angle;
    println!("The distance between the two points is: {:.1} kilometers", distance); // {:.1} means
                                                                                    // we only want
                                                                                    // 1 decimal
                                                                                    //   place
}
fn part2() {
    const EARTH_RADIUS_IN_KILOMETERS: f64 = 6371.0;

    let route = [
        ("apple", 41.0000, -81.0000),
        ("orange", 42.0000, -82.0000),
        ("plumb", 43.0000, -83.0000),
        ("cat", 44.0000, -84.0000),
        ("eggs", 45.0000, -85.0000),
        ("gloves", 46.0000, -86.0000),
        ("hat", 47.0000, -87.0000),
        ("yogurt", 48.0000, -88.0000),
        ("hair", 49.0000, -89.0000),
        ("jam", 50.0000, -90.0000),
        ("qack", 51.0000, -91.0000),
        ("mantis", 52.0000, -92.0000),
        ("lava", 53.0000, -93.0000),
        ("uniform", 54.0000, -94.0000),
    ];

    let mut total_distance = 0.0;
    let mut previous_waypoint: Option<(&str, f64, f64)> = None;

    for waypoint in route.iter() {
        match previous_waypoint {
            None => {
                previous_waypoint = Option::from(waypoint.clone());  // convert tuple to option
                continue;
            }
            Some(previous_waypoint) => {
                let previous_waypoint_radians = previous_waypoint.1.to_radians(); // apple
                let waypoint_radians = waypoint.1.to_radians();

                let delta_lat = (previous_waypoint.1 - waypoint.1).to_radians(); // apple
                let delta_long = (previous_waypoint.2 - waypoint.2).to_radians();  // apple

                let inner_central_angle = f64::powi((delta_lat / 2.0).sin(),2)
                    + previous_waypoint_radians.cos() * waypoint_radians.cos()
                    * f64::powi((delta_long / 2.0).sin(),2);

                let central_angle = 2.0 * inner_central_angle.sqrt().asin();
                let distance = EARTH_RADIUS_IN_KILOMETERS * central_angle;
                total_distance += distance;
                previous_waypoint = Option::from(waypoint.clone());

                println!("The distance between {} and {} is {:.1} kilometers",
                    previous_waypoint.0, waypoint.0, distance);  // apple
            }
        }
    }

}
fn main() {

    println!("done. just need to figure out that 1 error");
}
