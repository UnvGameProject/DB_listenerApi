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
mod controller;
//mod scrap;
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

/*Tables in mysql need to be created prior to demonstrating this...it is simple to add drop table if 
exist and create a table however this will run into unnecessary errors based on mysql privileges for
the user profile and the tables are not meant to be modifiable by this program
CQN like rabbitmq or redis, the dll idea is terrible and will not keep-up in addition
to being easily prone to breaking due to limited memory size and memory locks
last idea is perhaps an app listening on a unix socket??!?

This was written about ad nauseum for the past 10 years and no one has found 
as suitable solution outside of better databases or DBMS that are meant to do this
Jeffs code builds but never actually ran, symbol not accessible within mysql trigger
BY DESIGN to avoid malicious code and due to revisions on later versions in response 
to all thos sql injection attacks people were doing a few years back...

               ****Tables****

-> Table 1:
CREATE TABLE testdb.article_test (article_id INT(10) UNSIGNED AUTO_INCREMENT PRIMARY KEY,
article_title VARCHAR(150) NOT NULL, article_url VARCHAR(150) NOT NULL, article_date 
TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP)

-> Table 2:
CREATE TABLE testdb.sample_event_table(eventid INT(6) UNSIGNED AUTO_INCREMENT PRIMARY KEY,
title VARCHAR(100), e_status VARCHAR(100) NOT NULL, priority INT(6) NOT NULL, description 
VARCHAR(100) NOT NULL, event_date TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP)

*/