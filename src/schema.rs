// @generated automatically by Diesel CLI.

diesel::table! {
    customer (customer_id) {
        customer_id -> Int4,
        first_name -> Varchar,
        last_name -> Varchar,
        email -> Varchar,
    }
}

diesel::table! {
    order_items (order_id, product_id) {
        order_id -> Int4,
        product_id -> Int4,
        quantity -> Int4,
    }
}

diesel::table! {
    orders (order_id) {
        order_id -> Int4,
        order_date -> Date,
        customer_id -> Nullable<Int4>,
        payment_id -> Nullable<Int4>,
        order_amount -> Nullable<Int4>,
        shipping_address -> Text,
        shipping_amount -> Nullable<Int4>,
    }
}

diesel::table! {
    payment (payment_id) {
        payment_id -> Int4,
        payment_method -> Varchar,
        card_number -> Varchar,
        expiry_date -> Date,
    }
}

diesel::table! {
    product (product_id) {
        product_id -> Int4,
        product_name -> Varchar,
        price -> Numeric,
        description -> Text,
        category -> Text,
        reviews_rating -> Text,
        image -> Nullable<Varchar>,
    }
}

diesel::joinable!(order_items -> orders (order_id));
diesel::joinable!(order_items -> product (product_id));
diesel::joinable!(orders -> customer (customer_id));
diesel::joinable!(orders -> payment (payment_id));

diesel::allow_tables_to_appear_in_same_query!(
    customer,
    order_items,
    orders,
    payment,
    product,
);
