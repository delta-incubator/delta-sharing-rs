CREATE TABLE IF NOT EXISTS table_metadata (
    id UUID PRIMARY KEY,
    name VARCHAR NOT NULL,
    location VARCHAR NOT NULL,
    metadata JSONB
);
