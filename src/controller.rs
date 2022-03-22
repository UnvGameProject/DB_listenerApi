//This module receives user input for desired db, table, or columns 
//Within the db and gives options to be updated based on
//Mysql insert, update, delete, modify commands
//May implement mysql indexes for efficiency.

//Pathprep initializes the connection to the database through url/path parsing,
//Here, Mysql navigation is done through mysql commands.
#![allow(unused_imports)]
use mysql::*;
use mysql::prelude::*;

#[derive(Debug)]
pub struct Tablez {
    pub name: String,
}
#[allow(dead_code)]
impl Tablez{
    pub fn name(&self) -> &str{
        &self.name
    }
 }

pub fn run(poolpass: PooledConn) -> Result<()> {
    let mut conn = poolpass;
    let table_stmt = format!("SHOW TABLES IN testdb");
    println!("\n{}  as JSON", table_stmt);
    #[allow(unused_mut)]
    #[allow(unused_variables)]
    let mut db_tables:Vec<String> = Vec::new();
    let mut selection = conn.start_transaction(TxOpts::default())?;
    #[allow(unused_mut)]
    let mut db_tables =  selection.query_iter(
        table_stmt).unwrap(); //might need & pointer ref
    println!("\n");
    println!("{:?} \n", db_tables);
    println!("iterate through json keys...values still encoded: \n");
    #[allow(unused_variables)]
    let mut sets = 0;
    #[allow(deprecated)]
    while let Some(result_set) = db_tables.next_set() {
        let result_set = result_set;
        sets += 1;

        println!("moving through values: {:?}", result_set.columns());
        println!("result set meta: {:?} {:?}",
        result_set.affected_rows(),
        result_set.info_str(),
        );
    };
    //println!("{:?}", &newform); 
    // selection.rollback();
    
    Ok(())
}