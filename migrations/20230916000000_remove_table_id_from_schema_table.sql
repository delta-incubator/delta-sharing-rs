ALTER TABLE "schema"
DROP COLUMN table_id;
ALTER TABLE "table"
ADD COLUMN schema_id UUID NOT NULL REFERENCES "schema"(id);