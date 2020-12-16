// macros
use crate::schema::*;

// serialize
use serde::Serialize;

#[derive(Debug, Queryable, Serialize)]
pub struct Diagram {
    pub id: i32,
    pub title: String,
    pub photo: String,
    pub caption: String,
}

#[derive(Debug, Insertable, AsChangeset)]
#[table_name = "diagrams"]
pub struct NewDiagram<'a> {
    pub title: &'a str,
    pub photo: String,
    pub caption: &'a str,
}
