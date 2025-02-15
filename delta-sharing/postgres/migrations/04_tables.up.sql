create table if not exists table_metadata (
    id UUID primary key default uuidv7(),
    namespace Text [] collate case_insensitive not null,
    name Text collate case_insensitive not null,
    location Text not null,
    metadata JSONB,
    created_at timestamptz not null default now(),
    updated_at timestamptz,
    constraint unique_table_name unique (namespace, name)
);
select trigger_updated_at('table_metadata');
