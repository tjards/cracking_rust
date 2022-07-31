/*

Title: Is Unique 

Description: 
Part 1: Determine if a string has all unique characters. 
Part 2: What if you cannot use additional data structures?

Considerations:
- Assumes encoded ASCII (128 chars) or Unicode (a bunch of chars)? Assume ASCII
- ASCII has 7-bit/128 chars, so we can do a quit check of string size
- Unicode would require a lot more storage 
- HashSet is good for when you never want for more than one of something

Complexity:
- Since Part 1 will never search through more than 128 characters, 
 you could say it is of O(1) complexity 
- Else, it is at most of O(n) complexity, where n is the number of characters
- Assuming Part 2 requires iteratively comparing all the chars,
then that would be of O(n^2) complexity

Author: tjards

*/

// import stuff
use std::collections::HashSet;

// Part 1 - checking using string length + HashSet
// ------------------------------------------------ 

// this function accepts the string, returns boolean
fn check_unique_char(input: &str) -> bool {
    
    // Since ASCII only has 128 char, a longer string must repeat chars
    if input.len() > 128 {
        return false;
    } else {
        // create a new set
        let mut character_set: HashSet<char> = HashSet::new();
        // compare each char to the new set
        for i in input.chars(){
            if character_set.contains(&i) {
                return false;
            }
            // then add it to the set for next round
            character_set.insert(i);
        } // end running through char set
    } // end else greater than 128
    true
} // end checker 

// Part 2 - If you couldn't use additional data structures
// -------------------------------------------------------

/*  
I donno what this really means... 
Best guess, I would just interate through each character in the 
string and compare to the others. If I sorted this first, I could
just check neighbours.
*/

fn main(){

    let input = String::from("hellorustyworld");

    // if ACSII, run the program
    match input.is_ascii(){
        true => {
            let output_1 = check_unique_char(&input);
            println!("input '{}' char uniqueness: {}",input,output_1);
            //let output_2 = check_unique_char_restricted(&input);
            //println!("input '{}' char uniqueness: {} (method 2)",input,output_2);
        }
        false => {
            println!("input '{}' contains non-ASCII chars. This program only
            supports checking ASCII" ,input);
        }

    }
}





