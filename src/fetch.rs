#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate diesel;
extern crate serde;
extern crate serde_json;

mod data;
mod schema;

use diesel::prelude::*;
use diesel::sql_query;

#[derive(diesel::QueryableByName)]
struct Index
{
    #[sql_type = "diesel::sql_types::Integer"]
    id: i32,
}

fn get_or_insert_feed(db: &PgConnection, feed: &data::Feed) -> i32
{
    let res: Vec<Index> = sql_query(include_str!("insert_feed.sql"))
        .bind::<diesel::sql_types::Text, _>(&feed.url)
        .bind::<diesel::sql_types::Text, _>(&feed.title)
        .bind::<diesel::sql_types::Text, _>(&feed.description)
        .get_results(db)
        .expect("Couldn't load from db");
    res[0].id
}

fn process_feed(
    db: &PgConnection,
    feed: &data::Feed,
) -> Result<usize, diesel::result::Error>
{
    let feed_id = get_or_insert_feed(db, &feed);
    println!("{}", &feed.title);

    let bulk_insert: Vec<_> = feed
        .entries
        .iter()
        .map(|entry| {
            (
                schema::entry::columns::feed_id.eq(feed_id),
                schema::entry::columns::title.eq(entry.title.clone()),
                schema::entry::columns::link.eq(entry.link.clone()),
                schema::entry::columns::comments.eq(entry.comments.clone()),
            )
        })
        .collect();

    diesel::insert_into(schema::entry::table)
        .values(&bulk_insert)
        .on_conflict_do_nothing()
        .execute(db)
}

fn establish_connection() -> PgConnection
{
    let database_url = "postgres:///manu";
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

/**
 * This main is responsible for fetching rss updates and inserting
 * them into the database.
 */
fn main()
{
    let db = establish_connection();
    let overview = data::OverviewContext::new(&[
        "https://news.ycombinator.com/rss",
        "https://lobste.rs/rss",
    ]);

    overview
        .feeds
        .iter()
        .for_each(|feed| match process_feed(&db, feed)
        {
            Ok(n) => println!("  added {} entries", n),
            Err(err) => println!("Error with feed {}: {}", feed.title, err),
        });
}
