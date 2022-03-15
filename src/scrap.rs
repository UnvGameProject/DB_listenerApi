//working scrap paper for misc ideas
use crate::pathprep::*;

pub fn run(word: String){
    // let testword = dbu.dbn;
    let done = mysql_stmt(word);
    println!("{}", done)
}
#[allow(unused_variables)]
pub fn mysql_stmt(word: String) -> String {
    let dbstmt = format!("r\"INSERT INTO TABLE {}\"", word.trim());
    let returnword = dbstmt.trim();
    returnword.to_string()
}