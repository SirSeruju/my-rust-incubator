-- This file should undo anything in `up.sql`


DROP TRIGGER at_least_one_role on users;
DROP TRIGGER at_least_one_role on roles;
DROP TRIGGER at_least_one_role on users_roles;

DROP FUNCTION check_user_has_role;

DROP TABLE users_roles;
DROP TABLE users;
DROP TABLE roles;
