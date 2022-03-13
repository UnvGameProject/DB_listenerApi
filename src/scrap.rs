//tester page 
#![allow(unused_variables)]
pub fn run(){
    let example_str: &str = "howdy";
    let example_string: String = String::from("partner");
    let example_string1: String = String::from("partner");
    let example_string2: String = String::from("user");

    let string_from_str: String = example_str.to_string();
    let string_from_str2: String = "Some hardcoded string".to_string();

    let string_from_hardcoded = String::from("hardcoded text");
    let string_from_str_var = String::from(example_str);

    //This begins the examples from "str"(slice) to "string"

    let str_from_string: &str = &example_string;

    // Moving to concatenating(combining) strings
    
    let combined_string_literals = ["first", "second"].concat();
    let combine_with_format_macro = format!("{}:{}@", "first", "second");

    let string_plus_str = example_string1 + example_str;

    let string_plus_str_then_format_macro: String = format!("://{}@{}", example_string2 + example_str, combined_string_literals);



    //printing first set of examples (converting string to &str)
    println!("1. {}", example_str );
    println!("2. {}", example_string);
    println!("3. {}", string_from_str );
    println!("4. {}", string_from_str2);
    println!("5. {}", string_from_hardcoded);
    println!("6. {}", string_from_str_var);
    println!("7. {}", str_from_string);
    println!("*****Moving to the concatenate examples*****" );

// Moving to printing concatenating strings examples
    println!("{}", combined_string_literals);
    println!("{}", combine_with_format_macro);
    println!("{}", string_plus_str);
    println!("{}", string_plus_str_then_format_macro);
    // println!("{}", );



}