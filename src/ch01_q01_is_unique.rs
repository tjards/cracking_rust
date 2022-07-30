/*

Title: Is Unique 

Description: 

Part 1: Determine if a string has all unique characters. 
Part 2: What if you cannot use additional data structures?

Considerations:
- Encoded ASCII (128 chars) or Unicode (a bunch of chars)? Assume ASCII
- ASCII has 7-bit/128 chars, Extended ASCII would have 8-bit/256, so clarify
- Unicode would require a lot more storage 

*/

// this function accepts the string and outputs a boolean
fn check_unique_char(input: &str) -> bool {
    // Since ASCII only has 128 char, a longer string must repeat chars
    if input.len() > 128 {
        return false;
    }
    true
}

fn main(){
    let input = "thisistheinputstring";
    let output = check_unique_char(&String::from(input));
    println!("input '{}' char uniqueness: {}",input,output);
}

