shares:
  - name: dat
    schemaRefs:
      - basic
      - complex
      - features

schemas:
  - name: basic
    tableRefs:
      - all_primitive_types
      - basic_append
      - basic_partitioned
  - name: complex
    tableRefs:
      - multi_partitioned
      - multi_partitioned_2
      - nested_types
      - no_replay
      - no_stats
      - partitioned_with_null
      - stats_as_struct
      - with_checkpoint
      - with_schema_change
  - name: features
    tableRefs:
      - cdf
      - check_constraints
      - column_mapping
      - deletion_vectors
      - generated_columns
      - iceberg_compat_v1
      - timestamp_ntz

tables:
  - name: all_primitive_types
    location: file://$DIRECTORY/$DAT/out/reader_tests/generated/all_primitive_types/delta
  - name: basic_append
    location: file://$DIRECTORY/$DAT/out/reader_tests/generated/basic_append/delta
  - name: basic_partitioned
    location: file://$DIRECTORY/$DAT/out/reader_tests/generated/basic_partitioned/delta
  - name: cdf
    location: file://$DIRECTORY/$DAT/out/reader_tests/generated/cdf/delta
  - name: check_constraints
    location: file://$DIRECTORY/$DAT/out/reader_tests/generated/check_constraints/delta
  - name: column_mapping
    location: file://$DIRECTORY/$DAT/out/reader_tests/generated/column_mapping/delta
  - name: deletion_vectors
    location: file://$DIRECTORY/$DAT/out/reader_tests/generated/deletion_vectors/delta
  - name: generated_columns
    location: file://$DIRECTORY/$DAT/out/reader_tests/generated/generated_columns/delta
  - name: iceberg_compat_v1
    location: file://$DIRECTORY/$DAT/out/reader_tests/generated/iceberg_compat_v1/delta
  - name: multi_partitioned
    location: file://$DIRECTORY/$DAT/out/reader_tests/generated/multi_partitioned/delta
  - name: multi_partitioned_2
    location: file://$DIRECTORY/$DAT/out/reader_tests/generated/multi_partitioned_2/delta
  - name: nested_types
    location: file://$DIRECTORY/$DAT/out/reader_tests/generated/nested_types/delta
  - name: no_replay
    location: file://$DIRECTORY/$DAT/out/reader_tests/generated/no_replay/delta
  - name: no_stats
    location: file://$DIRECTORY/$DAT/out/reader_tests/generated/no_stats/delta
  - name: partitioned_with_null
    location: file://$DIRECTORY/$DAT/out/reader_tests/generated/partitioned_with_null/delta
  - name: stats_as_struct
    location: file://$DIRECTORY/$DAT/out/reader_tests/generated/stats_as_struct/delta
  - name: timestamp_ntz
    location: file://$DIRECTORY/$DAT/out/reader_tests/generated/timestamp_ntz/delta
  - name: with_checkpoint
    location: file://$DIRECTORY/$DAT/out/reader_tests/generated/with_checkpoint/delta
  - name: with_schema_change
    location: file://$DIRECTORY/$DAT/out/reader_tests/generated/with_schema_change/delta
