DROP TABLE "users";
DROP EXTENSION pgcrypto;
DROP FUNCTION bf_encrypt_password;
DROP FUNCTION authenticate_user_via_password;
DROP TRIGGER bf_encrypt_user_password_on_insert;
DROP TRIGGER bf_encrypt_user_password_on_update;
