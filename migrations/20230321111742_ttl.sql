-- Add migration script here
CREATE TABLE IF NOT EXISTS ttl (
    id UUID PRIMARY KEY,
    seconds INT,
    account_id UUID NOT NULL REFERENCES account(id),
    created_at TIMESTAMP WITH TIME ZONE NOT NULL default CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL default CURRENT_TIMESTAMP
);
