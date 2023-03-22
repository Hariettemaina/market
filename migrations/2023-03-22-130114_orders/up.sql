-- Your SQL goes here
CREATE TABLE orders (
    order_id SERIAL PRIMARY KEY,
    order_date DATE NOT NULL,
    customer_id INT REFERENCES customer(customer_id),
    payment_id INT REFERENCES payment(payment_id),
    order_amount INTEGER,
    shipping_address TEXT NOT NULL,
    shipping_amount INTEGER
);