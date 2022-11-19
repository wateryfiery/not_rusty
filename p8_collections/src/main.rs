//#[warn(unused_imports)]
//use std::collections::VecDeque;
use std::collections::HashMap;
/*
 * sequence collections:
 *  do more research
 *  types:
 *      vector
 *      double ended queue vector
 *      linked list
 *
 * vector:
 *  make it mutable and add the type
 *  commands: push, pop, iter, remove, insert, len, clear, contains
 *      push = adds value to end
 *      iter = loops over collection
 *      insert = puts data in location 
 *      remove = removes item at specific index
 *      len = length of vector
 *      clear = empties vector
 *      contains = looks for exact match inside vector
 *
 * vector double ended queue (VecDeque):
 *  can add/remove from front or back
 *  cannot sort collection
 *
 * maps:
 *  stores entries as key value pairs
 *  types:
 *      hash
 *      btree
 *  when you assign key or value a certain type of data, it can't change and they all will be that
 *  way
 *  if keys are repeated then they just overwrite the existing key value pair
 *
 * sets:
 *  do more research
 *
 */

fn main() {
    /*
    let mut new_vector:Vec<&str> = Vec::new();
    let new_vector1 = vec![1,2,3,4];

    new_vector.push("i");
    new_vector.push("like");
    new_vector.push("apples");
    new_vector.push("alot");

    for vect in new_vector.iter() {
        println!("{}", vect);
    }
    println!("_____________");

    if let Some(third) = new_vector.get(3) {
        println!("{}", third);
    }

    println!("_____________");
    new_vector.insert(2, "new item in index 2");
    for vect in new_vector.iter() {
        println!("{}", vect);
    }

    println!("_____________");
    new_vector.remove(3);
    for vect in new_vector.iter() {
        println!("{}", vect);
    }
    */

    /*
    let mut rides:VecDeque<&str> = VecDeque::new();

    rides.push_front("front1");
    rides.push_back("back1");
    rides.push_front("front2");
    rides.push_front("front3");
    rides.push_back("back2");
    rides.push_back("back3");

    for ride in rides.iter() {
        println!("{}", ride);
    }
    */

    let mut rides = HashMap::new();

    rides.insert("ride1", ("10:00", "town"));
    rides.insert("ride2", ("30:00", "house"));
    rides.insert("ride3", ("40:00", "friend"));
    rides.insert("ride4", ("80:00", "shop"));

    let ride_number = "ride2";
    let option = rides.get(ride_number);
    let (time, destination) = option.unwrap();
    println!("{} {}", time, destination);

    let ride_number1 = "ride5";
    if !rides.contains_key(ride_number1) {
        rides.insert(ride_number1, ("20:00", "home"));
    } else {
        println!("ride ,{}, is already scheduled", ride_number1);
    }

    for ride in rides.iter() {
        println!("{:?}", ride);
    }




}
