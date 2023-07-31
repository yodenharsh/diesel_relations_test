// @generated automatically by Diesel CLI.

diesel::table! {
    books (id) {
        id -> Unsigned<Integer>,
        #[max_length = 255]
        title -> Varchar,
    }
}

diesel::table! {
    pages (id) {
        id -> Unsigned<Integer>,
        page_number -> Integer,
        content -> Text,
        book_id -> Unsigned<Integer>,
    }
}

diesel::joinable!(pages -> books (book_id));

diesel::allow_tables_to_appear_in_same_query!(
    books,
    pages,
);
