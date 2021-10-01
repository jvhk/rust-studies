#[derive(Queryable)] // will generate all of the code needed to load a Post struct from a SQL query.
pub struct Post{
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}

use super::schema::posts;

#[derive(Insertable)]
#[table_name="posts"]

pub struct NewPost<'a>{
    pub title: &'a str,
    pub body: &'a str,
}