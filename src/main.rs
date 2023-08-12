use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;
use std::error::Error;

pub mod model;
pub mod schema;

use crate::model::*;
use crate::schema::*;

fn establish_connection() -> MysqlConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    MysqlConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to the {database_url}"))
}

fn new_book(conn: &mut MysqlConnection, title: &str) -> Result<usize, diesel::result::Error> {
    let number_of_rows_affected: Result<usize, diesel::result::Error> =
        diesel::insert_into(books::table)
            .values(books::title.eq(title))
            .execute(conn);
    number_of_rows_affected
}

fn new_page(
    conn: &mut MysqlConnection,
    page_number: i32,
    content: &str,
    book_id: u32,
) -> Result<usize, diesel::result::Error> {
    let number_of_rows_affected = diesel::insert_into(pages::table)
        .values((
            pages::page_number.eq(page_number),
            pages::book_id.eq(book_id),
            pages::content.eq(content),
        ))
        .execute(conn)?;
    Ok(number_of_rows_affected)
}

fn joins(conn: &mut MysqlConnection) -> Result<(), Box<dyn Error>> {
    let page_with_book = pages::table
        .inner_join(books::table)
        .filter(books::title.eq("Momo"))
        .select((Page::as_select(), Book::as_select()))
        .load::<(Page, Book)>(conn)?;

    println!("Page-Book pairs: {page_with_book:?}");

    let book_without_pages = books::table
        .left_join(pages::table)
        .select((Book::as_select(), Option::<Page>::as_select()))
        .load::<(Book, Option<Page>)>(conn)?;

    println!("Book-Page pairs (including empty books): {book_without_pages:?}");
    Ok(())
}

fn main() {
    let conn: &mut MysqlConnection = &mut establish_connection();
}
