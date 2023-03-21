-- Add migration script here
CREATE TABLE IF NOT EXISTS "schema" (
    id UUID PRIMARY KEY,
    name VARCHAR NOT NULL UNIQUE,
    table_id UUID REFERENCES "table"(id),
    share_id UUID REFERENCES share(id),
    created_by UUID NOT NULL REFERENCES account(id),
    created_at TIMESTAMP WITH TIME ZONE NOT NULL default CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL default CURRENT_TIMESTAMP
);
