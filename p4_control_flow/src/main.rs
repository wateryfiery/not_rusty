enum NavAids {
    NDB,
    VOR,
    VORDME
}
/*
 * note:
 * 1..11 means this is exclusive, the last value is not included
 *  1,2,3,4,5,6,7,8,9,10
 *
 * 1..=11 mean this is inclusive, the last value is included
 *  1,2,3,4,5,6,7,8,9,10,11
 *
 * */

fn main() {
    /*
     * // If statement
     * let word = "dylan";
     * if word == "dylan" {
          println!("Hi dylan :)");
     * } else if word == "george" {
          println!("Is your first name dylan?");
     * } else {
          println!("Who are you?");
     * };
     *
     *
     * let person = "dylan";
     * let min_apples = 7;
     * let apples = 11;
     *
     * // the OR seperates the conditions [dylan] or [george + apple amount]
     * // it calculates AND first then it moves to the OR
     * if person == "dylan" || person == "george" && min_apples <= apples {
          println!("Yessss!")

      }
     * // to fix this use parenthese
     * if (person == "dylan" || person == "george") && min_apples <= apples {
          println!("Yessss!")
      }
     *
     *
     * Enumeration (Enum):
     * a list of things
     * they incriment by 1 based off previous item
     *  if previous item was 2 then next item will be 3
     *  if you send item number 4 to equal 6 then the next one will be 7
     *
     * println!("NDB:\t{}", NavAids::NDB as u8);
     * println!("VOR:\t{}", NavAids::VOR as u8);
     * println!("VORDME:\t{}", NavAids::VORDME as u8);
     *
     *
     *
     * No null datatype:
     *  they cause memory leaks
     *  they cause bugs
     *  option helps with what happens when there is a "null"
     *
     *
     * Match:
     *  Similar to switch statments in other languages
     *  we can't write an exclusive range only inclusive, not that that should be an issue
     *
     *
     *  Example1:
     * let phrase: String = String::from("dylan george");
     * let letter = phrase.chars().nth(40);

     * let value = match letter {
          Some(character) => character.to_string(),
          None => String::from("No value")
       };
     * println!("{}", value);
     *  
     *  Example2:
     *
     * let animal = "duck";
     * match animal {
          "duck" => println!("It's a duck!"),
          "dog" => println!("It's not a duck"),
          _ => println!("It's not a duck or a dog")
       }
     *
     *
     * Example3:
     * let ndb_freq:u16 = 123;

     * match ndb_freq {
          22..=500 => println!("valid"),
          _ => println!("not valid")
       }
     *
     *
     *  Example4:
     * let ndb_freq:u16 = 300;

     * match ndb_freq {
          ndb_freq if ndb_freq >= 200 && ndb_freq <= 500 => {
              println!("Valid");
          }
          _ => println!("Not valid")
       }
     *
     *
     * if let:
     *  it sees what you make it equal is the same as what it already equals
     *
     * example1:
     *
     * let animal = "dog";
     * if let animal = "dog" {
          println!("are you a dog");
      }
     *
     *
     * loop:
     *  runs a loop forever until broken
     *  "break" = will break out of the loop
     *  "continue" = will ignore the rest and go back to the top of the loop
     *
     *
     * let mut counter: i32 = 0;
     * loop {
          counter += 1;
          if counter == 5 {
              continue;  // notice that 5 is missing because of "continue"
          }
          println!("{}", counter);
          if counter == 10 {
              break;
          }
      }
     *
     * while loop:
     *
     * let mut counter: i32 = 1;

     * while counter <= 10 {
          println!("{}", counter);
          counter += 1;
      }
     *
     * 
     * for loop:
     *
     * example1:
     * for index in 1..10 {
     *     println!("{}", index);
     * }
     *
     * example2:
     *
     * let ls_name = ["dylan", "austin", "george", "theone",];

     * for name in ls_name.iter() {
          println!("{}", name);
     * }
     *
     *
     * 
     *
     */
     println!("Done");

}
