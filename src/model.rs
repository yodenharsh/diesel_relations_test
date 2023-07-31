use crate::schema::{books, pages};
use diesel::prelude::*;

#[derive(Queryable, Identifiable, Selectable, Debug, PartialEq)]
#[diesel(table_name=books)]
pub struct Book {
    pub id: i32,
    pub title: String,
}

#[derive(Queryable, Identifiable, Selectable, Associations, Debug, PartialEq)]
#[diesel(belongs_to(Book))]
#[diesel(table_name=pages)]
pub struct Page {
    pub id: i32,
    pub page_number: i32,
    pub content: String,
    pub book_id: i32,
}
