-- Add migration script here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

create table users_pass (
    id uuid not null primary key,
    username varchar not null unique,
    password_hash varchar not null,
    created_at timestamp not null  default current_timestamp,
    updated_at timestamp not null  default current_timestamp

);