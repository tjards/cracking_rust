/*

Title: Check Permutations 

Description: Given two strings, write a method to decide if one 
    a permutation of the other

Considerations:
- what does perumutation mean? Assume = anagram
- to be permutations of eachother, strings have to be the same length
- assume sub-permutations are not in scope for the question
- ignore whitespace
- assume not cas-sensitive
- if they are permutations, a sorted set should be equivalent
- consider randomness to reduce time in checking (?)

Complexity:
- the sorting is of complexity O(n*Log(n))
- checking lengths first would be quick

Issues remaining:
- the .trim() method only removes whitespace from ends but I want 
    remove all whitespace (even between): start from line 66 below.
    may have to write something more custom.


Author: tjards

*/

// import stuff
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
        //println!("{:?},{:?}",vec1, vec2);
    }
    true

}

fn main(){

    // define the strings
    let input1 = String::from("he llorustyworld");
    let input2 = String::from("rhusetylwlorlod");

    println!("Inputs: {},{}",input1, input2);

    // remove whitespace (not working yet, as only trims ends)
    let input1_ = input1.trim();
    let input2_ = input2.trim();
    
    println!("Inputs (trimmed): {},{}",input1_, input2_);

    // run check
    let output = check_permutation(&input1_,&input2_);
    println!("inputs '{}, {}' permutationness (excluding whitespace) {}:",input1,input2,output);
      
}