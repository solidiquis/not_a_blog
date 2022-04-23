CREATE TABLE IF NOT EXISTS users (
  id SERIAL PRIMARY KEY,
  password VARCHAR(72),
  email VARCHAR(255) UNIQUE NOT NULL,
  username VARCHAR(255) UNIQUE NOT NULL,
  created_at TIMESTAMP NOT NULL,
  updated_at TIMESTAMP,
  deleted_at TIMESTAMP
);

CREATE EXTENSION IF NOT EXISTS pgcrypto;

CREATE OR REPLACE FUNCTION bf_encrypt_password()
RETURNS TRIGGER AS $$
BEGIN
	NEW.password = crypt(NEW.password, gen_salt('bf', 8));
	RETURN NEW;
END;
$$ LANGUAGE plpgsql
;

CREATE OR REPLACE TRIGGER bf_encrypt_user_password_on_insert
BEFORE INSERT
ON "users"
FOR ROW
WHEN (NEW.password IS NOT NULL)
EXECUTE FUNCTION bf_encrypt_password()
;

CREATE OR REPLACE TRIGGER bf_encrypt_user_password_on_update
BEFORE UPDATE
ON "users"
FOR ROW
WHEN (OLD.password IS DISTINCT FROM NEW.password)
EXECUTE FUNCTION bf_encrypt_password()
;

CREATE OR REPLACE FUNCTION authenticate_user_via_password(username VARCHAR(255), raw_password VARCHAR(72))
RETURNS BOOLEAN AS $$
DECLARE
	encrypted_password VARCHAR(255);
	verified BOOLEAN;
BEGIN
	SELECT password FROM "users"
	WHERE "users".username = username
	INTO encrypted_password;
	
	SELECT encrypted_password = crypt(raw_password, encrypted_password)
	INTO verified;
	
	RETURN verified;
END;
$$ LANGUAGE plpgsql;
