-- Add migration script here
CREATE TABLE IF NOT EXISTS account (
    id UUID PRIMARY KEY,
    name VARCHAR NOT NULL UNIQUE,
    email VARCHAR,
    password VARCHAR,
    namespace VARCHAR,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL default CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL default CURRENT_TIMESTAMP
);
