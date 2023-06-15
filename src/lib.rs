pub mod models;
pub mod schema;

use core::panic;
use std::env;

use diesel::sqlite::SqliteConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use schema::notes::{self, id};
use self::models::{Note,NewNote};

pub fn establish_connection() -> SqliteConnection{
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect(".env error");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {database_url}"))
}

pub fn create_note(conn: &mut SqliteConnection, f_path: String)->Note{
    let new_note = NewNote{file_path: &f_path};

    diesel::insert_into(notes::table)
        .values(&new_note)
        .returning(Note::as_returning())
        .get_result(conn)
        .expect("Error saving new note")
}

pub fn delete_note(conn: &mut SqliteConnection, id_input: i32)->Result<bool,String>{
    match diesel::delete(notes::table.filter(id.eq(id_input)))
    .execute(conn){
        Ok(u) => Ok(u > 0),
        Err(_) => Err(String::from("uhoh"))
    }
}

pub fn get_note(conn: &mut SqliteConnection, id_input: i32)->Result<Vec<Note>,String>{
    match notes::table.filter(id.eq(id_input))
                .select(Note::as_select())
                .load(conn){
                    Ok(u) => Ok(u),
                    Err(_) => Err(String::from("uhoh"))
                }
}