extern crate diesel_demo;
extern crate diesel;

use self::diesel::prelude::*;
use self::diesel_demo::*;
use std::env::args;

fn main(){
    use diesel_demo::schema::posts::dsl::*;

    let target = args().nth(1).expect("Expected target to match against");
    let pattern = format!("%{}", target);

    let connection = establish_connection();
    let num_deleted = diesel::delete(posts.filter(title.like(pattern)))
        .execute(&connection)
        .expect("Error deleting posts");
    println!("Delete {} posts", num_deleted);
}