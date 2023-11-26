-- Your SQL goes here


CREATE TABLE roles (
  slug VARCHAR PRIMARY KEY,
  name VARCHAR(64) NOT NULL,
  permissions VARCHAR(64) NOT NULL
);

CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  name VARCHAR(64) NOT NULL,
  bio VARCHAR(256) NOT NULL
);

CREATE TABLE users_roles (
  role_slug VARCHAR NOT NULL REFERENCES public.roles(slug),
  user_id INTEGER NOT NULL REFERENCES public.users(id),
  PRIMARY KEY (role_slug, user_id)
);

-- TODO: Large space for optimization and error specifications
-- Guarantees that user have at least 1 role
CREATE OR REPLACE FUNCTION check_user_has_role()
RETURNS TRIGGER AS
$$
  BEGIN
    -- checks if all users have at least one role (record in users_roles)
    IF (select COUNT(DISTINCT u.id) from users u) != (select COUNT(DISTINCT ur.user_id) from users_roles ur) THEN
      RAISE EXCEPTION 'User must have at least one role';
    END IF;
    RETURN NEW;
  END;
$$ LANGUAGE PLPGSQL VOLATILE;

CREATE CONSTRAINT TRIGGER at_least_one_role
AFTER INSERT OR UPDATE OR DELETE
ON users
DEFERRABLE INITIALLY DEFERRED
FOR EACH ROW
EXECUTE procedure check_user_has_role();

CREATE CONSTRAINT TRIGGER at_least_one_role
AFTER INSERT OR UPDATE OR DELETE
ON roles
DEFERRABLE INITIALLY DEFERRED
FOR EACH ROW
EXECUTE procedure check_user_has_role();

CREATE CONSTRAINT TRIGGER at_least_one_role
AFTER INSERT OR UPDATE OR DELETE
ON users_roles
DEFERRABLE INITIALLY DEFERRED
FOR EACH ROW
EXECUTE procedure check_user_has_role();
