/*

Title: Check Permutations 

Description: Given two strings, write a method to decide if one 
    a permutation of the other

Considerations:
- what does perumutation mean? Assume = anagram
- to be permutations of eachother, strings have to be the same length
- assume sub-permutations (i.e. dog and goddy) are not in scope for the question
- ignore whitespace
- assume not case-sensitive, so we convert all to lowercase
- if they are permutations, a sorted set should be equivalent

Complexity:
- the sorting is of complexity O(n*Log(n))
- checking lengths first would be quick

Author: tjards

*/

//use std::vec::Vec;

fn check_permutation(input1: &str, input2: &str) -> bool{

    // check if lengths the same
    if input1.chars().count() != input2.chars().count() {
        println!("{},{}",input1, input2);
        println!("Strings not the same length.");
        return false;
    } else{
        // load and sort the strings (as vectors)
        let mut vec1: Vec<char> = input1.chars().collect();
        vec1.sort_unstable();
        let mut vec2: Vec<char> = input2.chars().collect();
        vec2.sort_unstable();
        // if they are not the same
        if vec1 != vec2{
            println!("Strings dont have all the same chars .");
            return false;
        }
    }
    true

}

fn main(){

    // define the strings
    let mut input1 = String::from("he lLo Rustyworld");
    let mut input2 = String::from("rhusetylwlorlod");

    println!("Note: program is not case-sensitive and will automatically remove whitespace");
    println!("Inputs (raw): {},{}",input1, input2);

    // remove whitespace and lowercase everything
    input1.retain(|c| !c.is_whitespace());
    input1.make_ascii_lowercase();
    input2.retain(|c| !c.is_whitespace());
    input2.make_ascii_lowercase();
    
    println!("Inputs (cleaned): {},{}",input1, input2);

    // run check
    let output = check_permutation(&input1,&input2);
    println!("inputs '{}, {}' permutationness (excluding whitespace): {}",input1,input2,output);

}