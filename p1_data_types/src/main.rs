#![allow(unused_variables)]

fn main() {
    // Primative Data types #####################################
    // intergers
    // u8 = 0..255  u = unsigned integer
    // i8 = -128..127  i = signed integer
    // isize and usize = tied to CPU architecture
    // f32 and f64 = floating numbers (all signed)
    /*
     * Characters:
     * Each character has a number assigned to it
     * check out ASCII and Unicode tables
     *  the table you use will depend on the bytes you use
     *  1 Byte = 255 charactes in ASCII table
     *  2 bytes = 65,535 charactes in the Unicode-16 table
     *  4 bytes = 4,294,967,296 characts in the Unicode-32 table
     *  single quote for character
     *  double quote for strings
     */

    // Compound Data types #####################################
    // arrays = holds multiple values of a single data type
    // tuple = hlds multipe values of different data types
    /*
     * Array
     *  let location: [f32;2]  // we have 2 things in array that are f32
     *  you have to initialize an array before using it: 
     *      let location: [f32;2] = [0.0, 0.0];
     *      let location: [f64; 1000] = [0.0; 1000];
     */
    /*
     * Tuple
     * let location: (&str, f64, f64) = ("EL", 0.0, 0.0);
     * 2 ways to get info out of array/tuple:
     *  println!("name: {}, lat: {}, long: {}", location.0, location.1, location.2);
     *  or
     *  let (name: &str, lat: f64, long:f64) = location
     *  println!("name: {}, lat: {}, long: {}", name, lat, long);
     */
    /*
     * String vs &str
     * String = vector u8 data, mutable, stored in heap
     * &str = vector u8 ddata, immutable, stored in heap or stack or embedded in compiled code
     *
     * // string slice to string
     * let name: &str = "Dylan George";
     * let name_string1: String = name.to_string();
     * let name_string2: String = "Dylan George".to_string();
     * let name_string3: String = String::from("Dylan George");
     * println!("{}\n{}\n{}\n{}", name, name_string1, name_string2, name_string3);
     *
     * // string to string slice
     * let name: String = String::from("Dylan George");
     * let name_slice1 = &name;
     * let name_slice2 = name.as_str();
     * println!("{}\n{}\n{}", name, name_slice1, name_slice2);
     *
     * // concat string slices (since they immutable)
     * let name: &str = "Dylan";
     * let surname: &str = "George";
     * let full_name = [name, " ", surname].concat();
     * println!("{}", full_name);
     *
     * let name: &str = "Dylan";
     * let surname: &str = "George";
     * let full_name = format!("{} {}", name, surname);
     * println!("{}", full_name);
     *
     * let mut full_name: String = String::new();  // gives a blank string
     * full_name.push_str("Dylan George");  // pushes a string
     * full_name.push(' ');  // pushes a single character
     * full_name.push_str("is learning");
     * println!("{}", full_name);
     *
     * let mut full_name: String = String::new();  // gives a blank string
     * full_name.push_str("Dylan George");  // pushes a string
     * full_name.push(' ');  // pushes a single character
     * full_name = full_name + "is learning";
     * println!("{}", full_name);
     */

}
