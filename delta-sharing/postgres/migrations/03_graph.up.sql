CREATE TYPE object_label AS ENUM ('share', 'schema', 'table', 'principal');

CREATE TABLE objects (
    id uuid primary key default uuidv7(),
    label object_label not null,
    namespace Text [] collate case_insensitive not null,
    name Text collate case_insensitive not null,
    properties jsonb,
    created_at timestamptz not null default now(),
    updated_at timestamptz,
    constraint unique_object_name unique (label, namespace, name)
);
select trigger_updated_at('objects');
create index objects_label_index on objects (label, name);

CREATE TYPE association_label AS ENUM (
    'has_part', 'part_of',
    'created', 'created_by'
);

CREATE TABLE associations (
    id uuid primary key default uuidv7(),
    from_id uuid not null references objects (id),
    label association_label not null,
    to_id uuid not null references objects (id),
    properties jsonb,
    created_at timestamptz not null default now(),
    updated_at timestamptz,
    unique (from_id, label, to_id)
);
select trigger_updated_at('associations');
create index associations_label_index on associations (label);
create index associations_tuple_index on associations (from_id, label, to_id);
