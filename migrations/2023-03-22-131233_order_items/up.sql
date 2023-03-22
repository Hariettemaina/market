-- Your SQL goes here
CREATE TABLE order_items (
    order_id INT REFERENCES orders(order_id),
    product_id INT REFERENCES product(product_id),
    quantity INTEGER NOT NULL,
    PRIMARY KEY (order_id, product_id)
);