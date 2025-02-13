/* adopted from https://github.com/lakekeeper/lakekeeper/blob/main/crates/iceberg-catalog/migrations/01_setup.sql */

-- We try to ensure every table has `created_at` and `updated_at` columns, which can help immensely with debugging
-- and auditing.
--
-- While `created_at` can just be `default now()`, setting `updated_at` on update requires a trigger which
-- is a lot of boilerplate. These two functions save us from writing that every time as instead we can just do
--
-- select trigger_updated_at('<table name>');
--
-- after a `CREATE TABLE`.
create or replace function set_updated_at() returns trigger as $$ begin NEW.updated_at = now();
return NEW;
end;
$$ language plpgsql;

comment on function set_updated_at() is 'Sets the `updated_at` column to the current timestamp';

create or replace function trigger_updated_at(tablename regclass) returns void as $$ begin execute format(
    'CREATE TRIGGER set_updated_at
    BEFORE UPDATE
    ON %s
    FOR EACH ROW
    WHEN (OLD is distinct from NEW)
    EXECUTE FUNCTION set_updated_at();',
    tablename
);
end;
$$ language plpgsql;

comment on function trigger_updated_at(regclass) is 'Creates a trigger to set the `updated_at` column to the current timestamp on update';

-- Finally, this is a text collation that sorts text case-insensitively, useful for `UNIQUE` indexes
-- over things like usernames and emails, without needing to remember to do case-conversion.
create collation case_insensitive (
    provider = icu,
    locale = 'und-u-ks-level2',
    deterministic = false
);
