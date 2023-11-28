-- Your SQL goes here

CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  name VARCHAR(64) NOT NULL,
  password_hash VARCHAR NOT NULL,
  UNIQUE (name)
);

CREATE TABLE users_friends (
  user_id INTEGER NOT NULL REFERENCES public.users(id) ON DELETE CASCADE,
  friend_id INTEGER NOT NULL REFERENCES public.users(id) ON DELETE CASCADE,
  PRIMARY KEY (user_id, friend_id),
  CHECK (user_id != friend_id)
);
