CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

create table users (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4 (),
    name varchar(255) not null,
    email varchar(255) not null unique,
    password varchar(255) not null,
    created_at timestamp default current_timestamp
);

CREATE INDEX idx_users_email ON users (email);