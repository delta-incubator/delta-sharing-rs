-- Postgres cannot drop a value from an enum type. Remove any rows that use the values, then
-- rebuild the type without them so the schema returns to its 03_graph state.
delete from associations where to_label in ('sharing_schema_info', 'sharing_table');
delete from objects where label in ('sharing_schema_info', 'sharing_table');

alter type object_label rename to object_label_old;
create type object_label as enum (
    'share_info',
    'catalog_info',
    'schema_info',
    'table_info',
    'column_info',
    'credential_info',
    'external_location_info',
    'recipient_info'
);
alter table objects alter column label type object_label using label::text::object_label;
alter table associations alter column to_label type object_label using to_label::text::object_label;
drop type object_label_old;
