//main calls to run the program will go here...
#![allow(non_snake_case)]
#![allow(unused_must_use)]
#![allow(unused_imports)]
#![allow(dead_code)]
use std::{error::Error, borrow::BorrowMut};
mod theme;
mod cli_vers;
mod pathprep;
mod table_edit;
                //Defining functions below. 
                                                //    "dyn error" is ptr to obj that
                                                //     implements error trait
                                                //     call "get" function from ureq crate, passing url
                                                //     envoke "call" method, returning "result" type
                                                //     extracting success value from "result" type. use "?" operator
                                                //     In case of fail, "?" just bails out
                                                //     In case of success, chain call to "into_string"
                                                //     returns result also, "?" extracts success value
                                                //     http response is stored in "response" variable 
fn main () /*-> Result<(), Box<dyn Error>>*/{
   println!("successful build"); 
   cli_vers::run();
    
}