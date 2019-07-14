#![feature(proc_macro_hygiene, decl_macro, rustc_private)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate diesel;

extern crate rss;
extern crate serde;
extern crate serde_json;

mod data;
mod schema;

use diesel::prelude::*;
use diesel::sql_query;
use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::Template;

#[derive(diesel::QueryableByName)]
struct Res
{
    #[sql_type = "diesel::sql_types::Text"]
    json: String,
}

#[post("/delete/<id>")]
fn delete(db: Database, id: i32) -> &'static str
{
    diesel::insert_into(schema::entry_filter::table)
        .values(schema::entry_filter::columns::entry_id.eq(id))
        .on_conflict_do_nothing()
        .execute(&db.0)
        .expect("Couldn't insert");

    ""
}

#[get("/")]
fn index(db: Database) -> Template
{
    let res = sql_query(include_str!("feed_overview.sql"))
        .load::<Res>(&db.0)
        .expect("Couldn't load from db");

    let json = res[0].json.clone();

    let overview = data::OverviewContext {
        feeds: serde_json::from_str::<Vec<data::Feed>>(&json)
            .expect("Couldn't load json"),
    };

    Template::render("index", &overview)
}

#[database("manu")]
pub struct Database(PgConnection);

/**
 * This main runs the web server.
 */
fn main()
{
    rocket::ignite()
        .attach(Database::fairing())
        .attach(Template::fairing())
        .mount("/public", StaticFiles::from("static"))
        .mount("/", routes![index, delete])
        .launch();
}
