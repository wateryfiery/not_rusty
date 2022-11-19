/*
 * Ask yourself:
 *  who owns the data?
 *  passing by reference or by value?
 *  is it mutable?
 *
 * Ownership and borrowing only apply to data on the heap
 *
 * Ownership:
 *  Only 1 owner of data at a time
 *  
 *
 *
 *
 *
 */
fn main() {
    /*
     * Ownership:
     *  Only 1 owner of data at a time
     *
     * Example1:
     * let original: String = String::from("original value");
     * println!("\nOriginal: \t\"{}\"", original);

     * let next: String = original;  // next took the info. variable "original" no longer exists
     * println!("\nnext: \t\"{}\"", next);  
     * println!("\nOriginal: \t\"{}\"", original);  // comment this out.
     *
     * ___
     *
     * Borrowing:
     *  allows another variable to take ownership of data without deallocating the original
     *  variable
     *  put "ampersand (&) next to the variable you borring from
     *  when its borrowed the variable points to the data
     *  You can't change the variable data while its being borrowed. You can change it when the
     *  borrower is done borrowing it
     *  it's done borrowing when it exits its scope
     *
     *  if the borrower wants to change its data you'll have to (example3):
     *      1) make original variable mutable
     *      2) change borrower from & to &mut
     *      3) dereference the borrrower by using "*"
     *
     *  example1 (fine):
     * let mut original: String = String::from("original value");
     * println!("\nOriginal: \t\"{}\"", original);
     * original = String::from("new value");

     * let next: &String = &original;
     * println!("\nnext: \t\"{}\"", next);
     * original = String::from("new new value");
     * println!("\nOriginal: \t\"{}\"", original);
     *
     * example2 (error):
     * let mut original: String = String::from("original value");
     * println!("\nOriginal: \t\"{}\"", original);

     * let next: &String = &original;  // borrowed here
     * original = String::from("new value");  // value changed after borrowed 
     * println!("\nnext: \t\"{}\"", next); // error! the thing borrowed is not there, it changed
     * println!("\nOriginal: \t\"{}\"", original);
     *
     * example3:
     *
     * let mut original: String = String::from("original value");
     * println!("\nOriginal: \t\"{}\"", original);
     * original = String::from("new value");

       {
          let next: &mut String = &mut original;
          println!("\nnext before change: \t\"{}\"", next);
          *next = String::from("next value");
          println!("\nnext after change: \t\"{}\"", next);
          println!("\nOriginal inner scope: \t\"{}\"", original);
       }
     * println!("\nOriginal outer scope: \t\"{}\"", original);
     *
     * ___
     *
     * lifetimes:
     *  when a variable exits scope it is finished. 
     *  do more research...
     *
     * example1 (see error):
     * let outer_scope;
       {
           let inner_scope: i32 = 5;
           outer_scope = &inner_scope;
       }
     * println!("{}", outer_scope);
     *
     */

}
