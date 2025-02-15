create table if not exists table_metadata (
    id UUID primary key default uuidv7(),
    -- table names are case-insensitive as per the delta-sharing spec
    name Text collate case_insensitive not null,
    location Text not null,
    metadata JSONB,
    created_at timestamptz not null default now(),
    updated_at timestamptz
);
select trigger_updated_at('table_metadata');
