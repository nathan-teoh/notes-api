
use diesel::{prelude::*};
use serde::{Deserialize,Serialize};
use crate::schema::notes;

#[derive(Queryable, Selectable)]
#[diesel(table_name = notes)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Note{
    id: i32,
    file_path: String,
    view_count: i32,
    mark_delete: i32,
}

#[derive(Insertable)]
#[diesel(table_name=notes)]
pub struct NewNote<'a>{
    pub file_path: &'a String
}