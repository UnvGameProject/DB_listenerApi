                           //mysql query stuff here
#[allow(unused_imports)]
#[allow(unused_variables)]
#[allow(unused_mut)]
use mysql::*;
use mysql::prelude::*;
use dbevent_api::Article;

// pub fn run(path: String) ->Result<PooledConn>{
//     test(path);
//     poolpass
// }
                             //connection to db & test insert
pub fn run(path: String) -> Result<PooledConn> {
    let db_url = path;
    let opts = Opts::from_url(&db_url)?;
    let pool = Pool::new(opts).unwrap();
    let mut conn = pool.get_conn().unwrap();
    println!("\n");
    conn.exec_drop(
                r"INSERT INTO testdb.sample_event_table (title, e_status, priority, description) 
                VALUES (:title, :e_status, :priority, :description)",
                params! {
                    "title" => "Rust insert",
                    "e_status" => "complete",
                    "priority" => "1",
                    "description" => "this is direct from rust",
                },
                ).unwrap();
    
    Ok(conn)
}


//Note insert will not work if there is a null field >mysql safety thing
pub fn add_from_newsapi(poolpass: PooledConn , articles: &Vec<Article>) -> Result<PooledConn> {
    println!("testing insert from online 3rd party api\n");
    let mut conn = poolpass;
    conn.exec_batch(
        r"INSERT INTO testdb.article_test (article_title, article_url)
        values (:article_title, :article_url)",
        articles.iter().map(|p|params! {
            "article_title" => &p.title(),
            "article_url" => &p.url(),
        })
    )?;
    for i in articles {
        println!("{}", i.title());
        println!("{}", i.url());
        println!("done");
    }
    Ok(conn)
}







          /*cheatsheet for working with mysql syntax below*/



    //                                         //create table
    //     conn.query_drop(
    //         r"CREATE TABLE tablename (
    //          column_one(your_id) datatype(int) AUTO_INCREMENT PRIMARY KEY,
    //          column_two(title) datatype(text) not null,
    //          column_three datatype(datetime) not null )"
    //         )?;
    //                         //insert to db based on response, or iterating though
    //     conn.batch(
    //         r"INSERT INTO table (column, column)
    //         VALUES (:Article_title, :Article_url)",
    //         db_post.iter().map(|A| params! {
    //             "title" => A.title,
    //             "Article_url" => A.Article_url })
    //    
    //         )?;
    //                                       //select from db
    //     let selected_articles = conn
    //     .query_map(
    //         "SELECT item_id, Article_title, Article_url",
    //         |(item_id, Article_title,Article_url)| {
    //             Article{title: Article_title, url: Article_url}
    //         }
    //     )?;
    // }
