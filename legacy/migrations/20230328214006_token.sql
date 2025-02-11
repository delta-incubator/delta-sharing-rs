-- Add migration script here
CREATE TABLE IF NOT EXISTS token (
    id UUID PRIMARY KEY,
    email VARCHAR NOT NULL,
    "role" VARCHAR NOT NULL,
    "value" VARCHAR NOT NULL,
    created_by UUID NOT NULL REFERENCES account(id),
    created_at TIMESTAMP WITH TIME ZONE NOT NULL default CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL default CURRENT_TIMESTAMP
);
