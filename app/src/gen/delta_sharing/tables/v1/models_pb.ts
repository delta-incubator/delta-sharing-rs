// @generated by protoc-gen-es v2.2.3 with parameter "target=ts,json_types=true"
// @generated from file delta_sharing/tables/v1/models.proto (package delta_sharing.tables.v1, syntax proto3)
/* eslint-disable */

import type { GenEnum, GenFile, GenMessage } from "@bufbuild/protobuf/codegenv1";
import { enumDesc, fileDesc, messageDesc } from "@bufbuild/protobuf/codegenv1";
import type { StructJson } from "@bufbuild/protobuf/wkt";
import { file_google_protobuf_struct } from "@bufbuild/protobuf/wkt";
import type { JsonObject, Message } from "@bufbuild/protobuf";

/**
 * Describes the file delta_sharing/tables/v1/models.proto.
 */
export const file_delta_sharing_tables_v1_models: GenFile = /*@__PURE__*/
  fileDesc("CiRkZWx0YV9zaGFyaW5nL3RhYmxlcy92MS9tb2RlbHMucHJvdG8SF2RlbHRhX3NoYXJpbmcudGFibGVzLnYxIlkKDFRhYmxlU3VtbWFyeRIRCglmdWxsX25hbWUYASABKAkSNgoKdGFibGVfdHlwZRgCIAEoDjIiLmRlbHRhX3NoYXJpbmcudGFibGVzLnYxLlRhYmxlVHlwZSLOAwoKQ29sdW1uSW5mbxIMCgRuYW1lGAEgASgJEhEKCXR5cGVfdGV4dBgCIAEoCRIRCgl0eXBlX2pzb24YAyABKAkSOgoJdHlwZV9uYW1lGAQgASgOMicuZGVsdGFfc2hhcmluZy50YWJsZXMudjEuQ29sdW1uVHlwZU5hbWUSGwoOdHlwZV9wcmVjaXNpb24YBSABKAVIAIgBARIXCgp0eXBlX3NjYWxlGAYgASgFSAGIAQESHwoSdHlwZV9pbnRlcnZhbF90eXBlGAcgASgJSAKIAQESFQoIcG9zaXRpb24YCCABKAVIA4gBARIUCgdjb21tZW50GAkgASgJSASIAQESFQoIbnVsbGFibGUYCiABKAhIBYgBARIcCg9wYXJ0aXRpb25faW5kZXgYCyABKAVIBogBARIWCgljb2x1bW5faWQYDCABKAlIB4gBAUIRCg9fdHlwZV9wcmVjaXNpb25CDQoLX3R5cGVfc2NhbGVCFQoTX3R5cGVfaW50ZXJ2YWxfdHlwZUILCglfcG9zaXRpb25CCgoIX2NvbW1lbnRCCwoJX251bGxhYmxlQhIKEF9wYXJ0aXRpb25faW5kZXhCDAoKX2NvbHVtbl9pZCKCBgoJVGFibGVJbmZvEgwKBG5hbWUYASABKAkSEwoLc2NoZW1hX25hbWUYAiABKAkSFAoMY2F0YWxvZ19uYW1lGAMgASgJEjYKCnRhYmxlX3R5cGUYBCABKA4yIi5kZWx0YV9zaGFyaW5nLnRhYmxlcy52MS5UYWJsZVR5cGUSRQoSZGF0YV9zb3VyY2VfZm9ybWF0GAUgASgOMikuZGVsdGFfc2hhcmluZy50YWJsZXMudjEuRGF0YVNvdXJjZUZvcm1hdBI0Cgdjb2x1bW5zGAYgAygLMiMuZGVsdGFfc2hhcmluZy50YWJsZXMudjEuQ29sdW1uSW5mbxIdChBzdG9yYWdlX2xvY2F0aW9uGAcgASgJSACIAQESEgoFb3duZXIYCyABKAlIAYgBARIUCgdjb21tZW50GAwgASgJSAKIAQESMAoKcHJvcGVydGllcxgNIAEoCzIXLmdvb2dsZS5wcm90b2J1Zi5TdHJ1Y3RIA4gBARIkChdzdG9yYWdlX2NyZWRlbnRpYWxfbmFtZRgOIAEoCUgEiAEBEhYKCWZ1bGxfbmFtZRgRIAEoCUgFiAEBEhcKCmNyZWF0ZWRfYXQYEiABKANIBogBARIXCgpjcmVhdGVkX2J5GBMgASgJSAeIAQESFwoKdXBkYXRlZF9hdBgUIAEoA0gIiAEBEhcKCnVwZGF0ZWRfYnkYFSABKAlICYgBARIXCgpkZWxldGVkX2F0GBYgASgDSAqIAQESFQoIdGFibGVfaWQYFyABKAlIC4gBAUITChFfc3RvcmFnZV9sb2NhdGlvbkIICgZfb3duZXJCCgoIX2NvbW1lbnRCDQoLX3Byb3BlcnRpZXNCGgoYX3N0b3JhZ2VfY3JlZGVudGlhbF9uYW1lQgwKCl9mdWxsX25hbWVCDQoLX2NyZWF0ZWRfYXRCDQoLX2NyZWF0ZWRfYnlCDQoLX3VwZGF0ZWRfYXRCDQoLX3VwZGF0ZWRfYnlCDQoLX2RlbGV0ZWRfYXRCCwoJX3RhYmxlX2lkKkIKCVRhYmxlVHlwZRIaChZUQUJMRV9UWVBFX1VOU1BFQ0lGSUVEEAASCwoHTUFOQUdFRBABEgwKCEVYVEVSTkFMEAIqugEKEERhdGFTb3VyY2VGb3JtYXQSIgoeREFUQV9TT1VSQ0VfRk9STUFUX1VOU1BFQ0lGSUVEEAASCQoFREVMVEEQARILCgdJQ0VCRVJHEAISCAoESFVESRADEgsKB1BBUlFVRVQQBBIHCgNDU1YQBRIICgRKU09OEAYSBwoDT1JDEAcSCAoEQVZSTxAIEggKBFRFWFQQCRIRCg1VTklUWV9DQVRBTE9HEAoSEAoMREVMVEFTSEFSSU5HEAsqxQIKDkNvbHVtblR5cGVOYW1lEiAKHENPTFVNTl9UWVBFX05BTUVfVU5TUEVDSUZJRUQQABILCgdCT09MRUFOEAESCAoEQllURRACEgkKBVNIT1JUEAMSBwoDSU5UEAQSCAoETE9ORxAFEgkKBUZMT0FUEAYSCgoGRE9VQkxFEAcSCAoEREFURRAIEg0KCVRJTUVTVEFNUBAJEgoKBlNUUklORxAKEgoKBkJJTkFSWRALEgsKB0RFQ0lNQUwQDBIMCghJTlRFUlZBTBANEgkKBUFSUkFZEA4SCgoGU1RSVUNUEA8SBwoDTUFQEBASCAoEQ0hBUhAREggKBE5VTEwQEhIVChFVU0VSX0RFRklORURfVFlQRRATEhEKDVRJTUVTVEFNUF9OVFoQFBILCgdWQVJJQU5UEBUSDgoKVEFCTEVfVFlQRRAWQvUBChtjb20uZGVsdGFfc2hhcmluZy50YWJsZXMudjFCC01vZGVsc1Byb3RvUAFaT2dpdGh1Yi5jb20vZGVsdGEtaW5jdWJhdG9yL2RlbHRhLXNoYXJpbmctcnMvZ28vZGVsdGFfc2hhcmluZy90YWJsZXMvdjE7dGFibGVzdjGiAgNEVFiqAhZEZWx0YVNoYXJpbmcuVGFibGVzLlYxygIWRGVsdGFTaGFyaW5nXFRhYmxlc1xWMeICIkRlbHRhU2hhcmluZ1xUYWJsZXNcVjFcR1BCTWV0YWRhdGHqAhhEZWx0YVNoYXJpbmc6OlRhYmxlczo6VjFiBnByb3RvMw", [file_google_protobuf_struct]);

/**
 * @generated from message delta_sharing.tables.v1.TableSummary
 */
export type TableSummary = Message<"delta_sharing.tables.v1.TableSummary"> & {
  /**
   * The full name of the table.
   *
   * @generated from field: string full_name = 1;
   */
  fullName: string;

  /**
   * @generated from field: delta_sharing.tables.v1.TableType table_type = 2;
   */
  tableType: TableType;
};

/**
 * @generated from message delta_sharing.tables.v1.TableSummary
 */
export type TableSummaryJson = {
  /**
   * The full name of the table.
   *
   * @generated from field: string full_name = 1;
   */
  fullName?: string;

  /**
   * @generated from field: delta_sharing.tables.v1.TableType table_type = 2;
   */
  tableType?: TableTypeJson;
};

/**
 * Describes the message delta_sharing.tables.v1.TableSummary.
 * Use `create(TableSummarySchema)` to create a new message.
 */
export const TableSummarySchema: GenMessage<TableSummary, TableSummaryJson> = /*@__PURE__*/
  messageDesc(file_delta_sharing_tables_v1_models, 0);

/**
 * @generated from message delta_sharing.tables.v1.ColumnInfo
 */
export type ColumnInfo = Message<"delta_sharing.tables.v1.ColumnInfo"> & {
  /**
   * Name of the column
   *
   * @generated from field: string name = 1;
   */
  name: string;

  /**
   * Full data type specification as SQL/catalogString text.
   *
   * @generated from field: string type_text = 2;
   */
  typeText: string;

  /**
   * Full data type specification, JSON-serialized.
   *
   * @generated from field: string type_json = 3;
   */
  typeJson: string;

  /**
   * Data type name.
   *
   * @generated from field: delta_sharing.tables.v1.ColumnTypeName type_name = 4;
   */
  typeName: ColumnTypeName;

  /**
   * Digits of precision; required for DecimalTypes.
   *
   * @generated from field: optional int32 type_precision = 5;
   */
  typePrecision?: number;

  /**
   * Digits to right of decimal; Required for DecimalTypes.
   *
   * @generated from field: optional int32 type_scale = 6;
   */
  typeScale?: number;

  /**
   * Format of IntervalType.
   *
   * @generated from field: optional string type_interval_type = 7;
   */
  typeIntervalType?: string;

  /**
   * Ordinal position of column (starting at position 0).
   *
   * @generated from field: optional int32 position = 8;
   */
  position?: number;

  /**
   * User-provided free-form text description.
   *
   * @generated from field: optional string comment = 9;
   */
  comment?: string;

  /**
   * Whether field may be Null.
   *
   * @generated from field: optional bool nullable = 10;
   */
  nullable?: boolean;

  /**
   * Partition index for column.
   *
   * @generated from field: optional int32 partition_index = 11;
   */
  partitionIndex?: number;

  /**
   * a unique id for the column
   *
   * @generated from field: optional string column_id = 12;
   */
  columnId?: string;
};

/**
 * @generated from message delta_sharing.tables.v1.ColumnInfo
 */
export type ColumnInfoJson = {
  /**
   * Name of the column
   *
   * @generated from field: string name = 1;
   */
  name?: string;

  /**
   * Full data type specification as SQL/catalogString text.
   *
   * @generated from field: string type_text = 2;
   */
  typeText?: string;

  /**
   * Full data type specification, JSON-serialized.
   *
   * @generated from field: string type_json = 3;
   */
  typeJson?: string;

  /**
   * Data type name.
   *
   * @generated from field: delta_sharing.tables.v1.ColumnTypeName type_name = 4;
   */
  typeName?: ColumnTypeNameJson;

  /**
   * Digits of precision; required for DecimalTypes.
   *
   * @generated from field: optional int32 type_precision = 5;
   */
  typePrecision?: number;

  /**
   * Digits to right of decimal; Required for DecimalTypes.
   *
   * @generated from field: optional int32 type_scale = 6;
   */
  typeScale?: number;

  /**
   * Format of IntervalType.
   *
   * @generated from field: optional string type_interval_type = 7;
   */
  typeIntervalType?: string;

  /**
   * Ordinal position of column (starting at position 0).
   *
   * @generated from field: optional int32 position = 8;
   */
  position?: number;

  /**
   * User-provided free-form text description.
   *
   * @generated from field: optional string comment = 9;
   */
  comment?: string;

  /**
   * Whether field may be Null.
   *
   * @generated from field: optional bool nullable = 10;
   */
  nullable?: boolean;

  /**
   * Partition index for column.
   *
   * @generated from field: optional int32 partition_index = 11;
   */
  partitionIndex?: number;

  /**
   * a unique id for the column
   *
   * @generated from field: optional string column_id = 12;
   */
  columnId?: string;
};

/**
 * Describes the message delta_sharing.tables.v1.ColumnInfo.
 * Use `create(ColumnInfoSchema)` to create a new message.
 */
export const ColumnInfoSchema: GenMessage<ColumnInfo, ColumnInfoJson> = /*@__PURE__*/
  messageDesc(file_delta_sharing_tables_v1_models, 1);

/**
 * @generated from message delta_sharing.tables.v1.TableInfo
 */
export type TableInfo = Message<"delta_sharing.tables.v1.TableInfo"> & {
  /**
   * Name of table, relative to parent schema.
   *
   * @generated from field: string name = 1;
   */
  name: string;

  /**
   * Name of parent schema.
   *
   * @generated from field: string schema_name = 2;
   */
  schemaName: string;

  /**
   * Name of parent catalog.
   *
   * @generated from field: string catalog_name = 3;
   */
  catalogName: string;

  /**
   * @generated from field: delta_sharing.tables.v1.TableType table_type = 4;
   */
  tableType: TableType;

  /**
   * Data source format of the table.
   *
   * @generated from field: delta_sharing.tables.v1.DataSourceFormat data_source_format = 5;
   */
  dataSourceFormat: DataSourceFormat;

  /**
   * The array of ColumnInfo definitions of the table's columns.
   *
   * @generated from field: repeated delta_sharing.tables.v1.ColumnInfo columns = 6;
   */
  columns: ColumnInfo[];

  /**
   * Storage root URL for table (for MANAGED, EXTERNAL tables)
   *
   * @generated from field: optional string storage_location = 7;
   */
  storageLocation?: string;

  /**
   * Username of current owner of table.
   *
   * @generated from field: optional string owner = 11;
   */
  owner?: string;

  /**
   * User-provided free-form text description.
   *
   * @generated from field: optional string comment = 12;
   */
  comment?: string;

  /**
   * A map of key-value properties attached to the securable.
   *
   * @generated from field: optional google.protobuf.Struct properties = 13;
   */
  properties?: JsonObject;

  /**
   * Name of the storage credential, when a storage credential is configured for use with this table.
   *
   * @generated from field: optional string storage_credential_name = 14;
   */
  storageCredentialName?: string;

  /**
   * Full name of table, in form of catalog_name.schema_name.table_name.
   *
   * @generated from field: optional string full_name = 17;
   */
  fullName?: string;

  /**
   * Time at which this table was created, in epoch milliseconds.
   *
   * @generated from field: optional int64 created_at = 18;
   */
  createdAt?: bigint;

  /**
   * Username of table creator.
   *
   * @generated from field: optional string created_by = 19;
   */
  createdBy?: string;

  /**
   * Time at which this table was last updated, in epoch milliseconds.
   *
   * @generated from field: optional int64 updated_at = 20;
   */
  updatedAt?: bigint;

  /**
   * Username of user who last modified table.
   *
   * @generated from field: optional string updated_by = 21;
   */
  updatedBy?: string;

  /**
   * Time at which this table was deleted, in epoch milliseconds. Field is omitted if table is not deleted.
   *
   * @generated from field: optional int64 deleted_at = 22;
   */
  deletedAt?: bigint;

  /**
   * Unique identifier for the table.
   *
   * @generated from field: optional string table_id = 23;
   */
  tableId?: string;
};

/**
 * @generated from message delta_sharing.tables.v1.TableInfo
 */
export type TableInfoJson = {
  /**
   * Name of table, relative to parent schema.
   *
   * @generated from field: string name = 1;
   */
  name?: string;

  /**
   * Name of parent schema.
   *
   * @generated from field: string schema_name = 2;
   */
  schemaName?: string;

  /**
   * Name of parent catalog.
   *
   * @generated from field: string catalog_name = 3;
   */
  catalogName?: string;

  /**
   * @generated from field: delta_sharing.tables.v1.TableType table_type = 4;
   */
  tableType?: TableTypeJson;

  /**
   * Data source format of the table.
   *
   * @generated from field: delta_sharing.tables.v1.DataSourceFormat data_source_format = 5;
   */
  dataSourceFormat?: DataSourceFormatJson;

  /**
   * The array of ColumnInfo definitions of the table's columns.
   *
   * @generated from field: repeated delta_sharing.tables.v1.ColumnInfo columns = 6;
   */
  columns?: ColumnInfoJson[];

  /**
   * Storage root URL for table (for MANAGED, EXTERNAL tables)
   *
   * @generated from field: optional string storage_location = 7;
   */
  storageLocation?: string;

  /**
   * Username of current owner of table.
   *
   * @generated from field: optional string owner = 11;
   */
  owner?: string;

  /**
   * User-provided free-form text description.
   *
   * @generated from field: optional string comment = 12;
   */
  comment?: string;

  /**
   * A map of key-value properties attached to the securable.
   *
   * @generated from field: optional google.protobuf.Struct properties = 13;
   */
  properties?: StructJson;

  /**
   * Name of the storage credential, when a storage credential is configured for use with this table.
   *
   * @generated from field: optional string storage_credential_name = 14;
   */
  storageCredentialName?: string;

  /**
   * Full name of table, in form of catalog_name.schema_name.table_name.
   *
   * @generated from field: optional string full_name = 17;
   */
  fullName?: string;

  /**
   * Time at which this table was created, in epoch milliseconds.
   *
   * @generated from field: optional int64 created_at = 18;
   */
  createdAt?: string;

  /**
   * Username of table creator.
   *
   * @generated from field: optional string created_by = 19;
   */
  createdBy?: string;

  /**
   * Time at which this table was last updated, in epoch milliseconds.
   *
   * @generated from field: optional int64 updated_at = 20;
   */
  updatedAt?: string;

  /**
   * Username of user who last modified table.
   *
   * @generated from field: optional string updated_by = 21;
   */
  updatedBy?: string;

  /**
   * Time at which this table was deleted, in epoch milliseconds. Field is omitted if table is not deleted.
   *
   * @generated from field: optional int64 deleted_at = 22;
   */
  deletedAt?: string;

  /**
   * Unique identifier for the table.
   *
   * @generated from field: optional string table_id = 23;
   */
  tableId?: string;
};

/**
 * Describes the message delta_sharing.tables.v1.TableInfo.
 * Use `create(TableInfoSchema)` to create a new message.
 */
export const TableInfoSchema: GenMessage<TableInfo, TableInfoJson> = /*@__PURE__*/
  messageDesc(file_delta_sharing_tables_v1_models, 2);

/**
 * The type of the table.
 *
 * @generated from enum delta_sharing.tables.v1.TableType
 */
export enum TableType {
  /**
   * @generated from enum value: TABLE_TYPE_UNSPECIFIED = 0;
   */
  TABLE_TYPE_UNSPECIFIED = 0,

  /**
   * @generated from enum value: MANAGED = 1;
   */
  MANAGED = 1,

  /**
   * @generated from enum value: EXTERNAL = 2;
   */
  EXTERNAL = 2,
}

/**
 * The type of the table.
 *
 * @generated from enum delta_sharing.tables.v1.TableType
 */
export type TableTypeJson = "TABLE_TYPE_UNSPECIFIED" | "MANAGED" | "EXTERNAL";

/**
 * Describes the enum delta_sharing.tables.v1.TableType.
 */
export const TableTypeSchema: GenEnum<TableType, TableTypeJson> = /*@__PURE__*/
  enumDesc(file_delta_sharing_tables_v1_models, 0);

/**
 * @generated from enum delta_sharing.tables.v1.DataSourceFormat
 */
export enum DataSourceFormat {
  /**
   * @generated from enum value: DATA_SOURCE_FORMAT_UNSPECIFIED = 0;
   */
  DATA_SOURCE_FORMAT_UNSPECIFIED = 0,

  /**
   * @generated from enum value: DELTA = 1;
   */
  DELTA = 1,

  /**
   * @generated from enum value: ICEBERG = 2;
   */
  ICEBERG = 2,

  /**
   * @generated from enum value: HUDI = 3;
   */
  HUDI = 3,

  /**
   * @generated from enum value: PARQUET = 4;
   */
  PARQUET = 4,

  /**
   * @generated from enum value: CSV = 5;
   */
  CSV = 5,

  /**
   * @generated from enum value: JSON = 6;
   */
  JSON = 6,

  /**
   * @generated from enum value: ORC = 7;
   */
  ORC = 7,

  /**
   * @generated from enum value: AVRO = 8;
   */
  AVRO = 8,

  /**
   * @generated from enum value: TEXT = 9;
   */
  TEXT = 9,

  /**
   * @generated from enum value: UNITY_CATALOG = 10;
   */
  UNITY_CATALOG = 10,

  /**
   * @generated from enum value: DELTASHARING = 11;
   */
  DELTASHARING = 11,
}

/**
 * @generated from enum delta_sharing.tables.v1.DataSourceFormat
 */
export type DataSourceFormatJson = "DATA_SOURCE_FORMAT_UNSPECIFIED" | "DELTA" | "ICEBERG" | "HUDI" | "PARQUET" | "CSV" | "JSON" | "ORC" | "AVRO" | "TEXT" | "UNITY_CATALOG" | "DELTASHARING";

/**
 * Describes the enum delta_sharing.tables.v1.DataSourceFormat.
 */
export const DataSourceFormatSchema: GenEnum<DataSourceFormat, DataSourceFormatJson> = /*@__PURE__*/
  enumDesc(file_delta_sharing_tables_v1_models, 1);

/**
 * @generated from enum delta_sharing.tables.v1.ColumnTypeName
 */
export enum ColumnTypeName {
  /**
   * @generated from enum value: COLUMN_TYPE_NAME_UNSPECIFIED = 0;
   */
  COLUMN_TYPE_NAME_UNSPECIFIED = 0,

  /**
   * @generated from enum value: BOOLEAN = 1;
   */
  BOOLEAN = 1,

  /**
   * @generated from enum value: BYTE = 2;
   */
  BYTE = 2,

  /**
   * @generated from enum value: SHORT = 3;
   */
  SHORT = 3,

  /**
   * @generated from enum value: INT = 4;
   */
  INT = 4,

  /**
   * @generated from enum value: LONG = 5;
   */
  LONG = 5,

  /**
   * @generated from enum value: FLOAT = 6;
   */
  FLOAT = 6,

  /**
   * @generated from enum value: DOUBLE = 7;
   */
  DOUBLE = 7,

  /**
   * @generated from enum value: DATE = 8;
   */
  DATE = 8,

  /**
   * @generated from enum value: TIMESTAMP = 9;
   */
  TIMESTAMP = 9,

  /**
   * @generated from enum value: STRING = 10;
   */
  STRING = 10,

  /**
   * @generated from enum value: BINARY = 11;
   */
  BINARY = 11,

  /**
   * @generated from enum value: DECIMAL = 12;
   */
  DECIMAL = 12,

  /**
   * @generated from enum value: INTERVAL = 13;
   */
  INTERVAL = 13,

  /**
   * @generated from enum value: ARRAY = 14;
   */
  ARRAY = 14,

  /**
   * @generated from enum value: STRUCT = 15;
   */
  STRUCT = 15,

  /**
   * @generated from enum value: MAP = 16;
   */
  MAP = 16,

  /**
   * @generated from enum value: CHAR = 17;
   */
  CHAR = 17,

  /**
   * @generated from enum value: NULL = 18;
   */
  NULL = 18,

  /**
   * @generated from enum value: USER_DEFINED_TYPE = 19;
   */
  USER_DEFINED_TYPE = 19,

  /**
   * @generated from enum value: TIMESTAMP_NTZ = 20;
   */
  TIMESTAMP_NTZ = 20,

  /**
   * @generated from enum value: VARIANT = 21;
   */
  VARIANT = 21,

  /**
   * @generated from enum value: TABLE_TYPE = 22;
   */
  TABLE_TYPE = 22,
}

/**
 * @generated from enum delta_sharing.tables.v1.ColumnTypeName
 */
export type ColumnTypeNameJson = "COLUMN_TYPE_NAME_UNSPECIFIED" | "BOOLEAN" | "BYTE" | "SHORT" | "INT" | "LONG" | "FLOAT" | "DOUBLE" | "DATE" | "TIMESTAMP" | "STRING" | "BINARY" | "DECIMAL" | "INTERVAL" | "ARRAY" | "STRUCT" | "MAP" | "CHAR" | "NULL" | "USER_DEFINED_TYPE" | "TIMESTAMP_NTZ" | "VARIANT" | "TABLE_TYPE";

/**
 * Describes the enum delta_sharing.tables.v1.ColumnTypeName.
 */
export const ColumnTypeNameSchema: GenEnum<ColumnTypeName, ColumnTypeNameJson> = /*@__PURE__*/
  enumDesc(file_delta_sharing_tables_v1_models, 2);

