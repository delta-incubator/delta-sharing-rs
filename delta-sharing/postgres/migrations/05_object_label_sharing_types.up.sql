-- The Rust `ObjectLabel` enum is generated from the `Resource` oneof and includes the sharing
-- resource types. Those two variants were missing from the `object_label` enum created in
-- 03_graph, so inserting a sharing table or sharing schema failed with an invalid enum value.
ALTER TYPE object_label ADD VALUE IF NOT EXISTS 'sharing_schema_info';
ALTER TYPE object_label ADD VALUE IF NOT EXISTS 'sharing_table';
