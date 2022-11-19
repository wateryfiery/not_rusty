fn main() {
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
