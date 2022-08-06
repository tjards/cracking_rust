/*

Title: URLify 

Description: Write a method to replace all spaces in a string with '%20'.

Considerations:
- assume we want to trim the ends of the string
- all spaces become %20, even if they repeat

Complexity:


Author: tjards

*/

fn urlify(unurlified: &String) -> String {

    let urlified = unurlified.trim().replace(" ","%20");
    return urlified.to_string();

}


fn main(){

    let input = String::from("Mr John Smith    ");
    println!("Input is: {}",&input);
    let output = urlify(&input);
    println!("Output is: {}",&output);

}