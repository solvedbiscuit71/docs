CREATE TABLE products (
  id SERIAL PRIMARY KEY,
  name VARCHAR,
  price INT,
  user_id INT REFERENCES users(id)
);
