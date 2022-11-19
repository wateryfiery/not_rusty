#![allow (unused)]
/*
 * associated methods:
 *  do more research
 *
 *
 * traits:
 *  shared behaviour amoung structs
 *  analogous to interfaces in object-oriented languages
 *
 */
struct Waypoint {
    name: String,
    lat: f64,
    long: f64
}

struct Car {
    required_crew: u8,
    range: u16
}

struct Bus {
    required_crew: u8,
    range: u16
}

trait Ride {
    fn is_legal(&self, required_crew: u8, available_crew: u8, range: u16, distance: u16) -> bool;
}

impl Ride for Car {
    fn is_legal(&self, required_crew: u8, available_crew: u8, range: u16, distance: u16) -> bool {
        if (available_crew >= required_crew) && (range + 150 > distance) {
            true
        } else {
            false
        }
    }
}

impl Ride for Bus {
    fn is_legal(&self, required_crew: u8, available_crew: u8, range: u16, distance: u16) -> bool {
        if (available_crew >= required_crew) && (range + 350 > distance) {
            true
        } else {
            false
        }
    }
}

fn main() {
    /*
    let mut apple = Waypoint {
        name: "apple".to_string(),
        lat: 41.4000,
        long: -81.2000
    };

    let orange = Waypoint {
        ..apple  // will copy everything from apple here
    };

    let grape = Waypoint {
        name: "orange".to_string(),
        ..apple
    };
    */

    let car = Car {
        required_crew: 4,
        range: 1000
    };

    let bus = Bus {
        required_crew: 7,
        range: 2000
    };

    let car_is_legal: bool = car.is_legal(
        car.required_crew,
        3,
        car.range,
        500
    );
    let bus_is_legal: bool = bus.is_legal(
        car.required_crew,
        30,
        car.range,
        500
    );
    println!("car is legal: {}", car_is_legal);
    println!("bus is legal: {}", bus_is_legal);



}
