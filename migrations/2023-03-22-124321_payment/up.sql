-- Your SQL goes here
CREATE TABLE payment (
    payment_id SERIAL PRIMARY KEY,
    payment_method VARCHAR(50) NOT NULL,
    card_number VARCHAR(16) NOT NULL,
    expiry_date DATE NOT NULL
);