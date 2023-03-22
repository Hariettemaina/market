-- Your SQL goes here
CREATE TABLE product (
    product_id SERIAL PRIMARY KEY,
    product_name VARCHAR(100) NOT NULL,
    price DECIMAL(10,2) NOT NULL,
    description TEXT NOT NULL,
    category TEXT NOT NULL,
    reviews_rating TEXT NOT NULL,
    image VARCHAR(255)
);