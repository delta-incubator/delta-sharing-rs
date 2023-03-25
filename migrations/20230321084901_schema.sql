-- Add migration script here
CREATE TABLE IF NOT EXISTS "schema" (
    id UUID PRIMARY KEY,
    name VARCHAR NOT NULL,
    share_id UUID NOT NULL REFERENCES share(id),
    table_id UUID NOT NULL REFERENCES "table"(id),
    created_by UUID NOT NULL REFERENCES account(id),
    created_at TIMESTAMP WITH TIME ZONE NOT NULL default CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL default CURRENT_TIMESTAMP,
    UNIQUE (share_id, name) INCLUDE (id)
);
