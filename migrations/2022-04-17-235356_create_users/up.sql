CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  email VARCHAR(255),
  username VARCHAR(255) NOT NULL,
  created_at TIMESTAMP,
  updated_at TIMESTAMP,
  deleted_at TIMESTAMP
)
