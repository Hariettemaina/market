// @generated automatically by Diesel CLI.

diesel::table! {
    customer (customer_id) {
        customer_id -> Int4,
        first_name -> Varchar,
        last_name -> Varchar,
        email -> Varchar,
    }
}
