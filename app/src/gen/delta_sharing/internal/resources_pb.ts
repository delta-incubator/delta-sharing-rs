// The types defined in this file are nor meant to be exposed to the user.
// They are used internally by the service to manage the resources.
// THis is mainly convenience as we can leverage the code generation to create the
// necessary types to manage the resources and utilize the same macros etc.

// @generated by protoc-gen-es v2.2.3 with parameter "target=ts,json_types=true"
// @generated from file delta_sharing/internal/resources.proto (package delta_sharing.internal, syntax proto3)
/* eslint-disable */

import type { GenFile, GenMessage } from "@bufbuild/protobuf/codegenv1";
import { fileDesc, messageDesc } from "@bufbuild/protobuf/codegenv1";
import type { CatalogInfo, CatalogInfoJson } from "../catalogs/v1/models_pb";
import { file_delta_sharing_catalogs_v1_models } from "../catalogs/v1/models_pb";
import type { CredentialInfo, CredentialInfoJson } from "../credentials/v1/models_pb";
import { file_delta_sharing_credentials_v1_models } from "../credentials/v1/models_pb";
import type { ExternalLocationInfo, ExternalLocationInfoJson } from "../external_locations/v1/models_pb";
import { file_delta_sharing_external_locations_v1_models } from "../external_locations/v1/models_pb";
import type { RecipientInfo, RecipientInfoJson } from "../recipients/v1/models_pb";
import { file_delta_sharing_recipients_v1_models } from "../recipients/v1/models_pb";
import type { SchemaInfo, SchemaInfoJson } from "../schemas/v1/models_pb";
import { file_delta_sharing_schemas_v1_models } from "../schemas/v1/models_pb";
import type { ShareInfo, ShareInfoJson } from "../shares/v1/models_pb";
import { file_delta_sharing_shares_v1_models } from "../shares/v1/models_pb";
import type { SharingSchemaInfo, SharingSchemaInfoJson, SharingTable, SharingTableJson } from "../sharing/v1/models_pb";
import { file_delta_sharing_sharing_v1_models } from "../sharing/v1/models_pb";
import type { ColumnInfo, ColumnInfoJson, TableInfo, TableInfoJson } from "../tables/v1/models_pb";
import { file_delta_sharing_tables_v1_models } from "../tables/v1/models_pb";
import type { Message } from "@bufbuild/protobuf";

/**
 * Describes the file delta_sharing/internal/resources.proto.
 */
export const file_delta_sharing_internal_resources: GenFile = /*@__PURE__*/
  fileDesc("CiZkZWx0YV9zaGFyaW5nL2ludGVybmFsL3Jlc291cmNlcy5wcm90bxIWZGVsdGFfc2hhcmluZy5pbnRlcm5hbCK8BQoIUmVzb3VyY2USOAoKc2hhcmVfaW5mbxgBIAEoCzIiLmRlbHRhX3NoYXJpbmcuc2hhcmVzLnYxLlNoYXJlSW5mb0gAEkoKE3NoYXJpbmdfc2NoZW1hX2luZm8YAiABKAsyKy5kZWx0YV9zaGFyaW5nLnNoYXJpbmcudjEuU2hhcmluZ1NjaGVtYUluZm9IABI/Cg1zaGFyaW5nX3RhYmxlGAMgASgLMiYuZGVsdGFfc2hhcmluZy5zaGFyaW5nLnYxLlNoYXJpbmdUYWJsZUgAEkcKD2NyZWRlbnRpYWxfaW5mbxgEIAEoCzIsLmRlbHRhX3NoYXJpbmcuY3JlZGVudGlhbHMudjEuQ3JlZGVudGlhbEluZm9IABI+CgxjYXRhbG9nX2luZm8YBiABKAsyJi5kZWx0YV9zaGFyaW5nLmNhdGFsb2dzLnYxLkNhdGFsb2dJbmZvSAASOwoLc2NoZW1hX2luZm8YByABKAsyJC5kZWx0YV9zaGFyaW5nLnNjaGVtYXMudjEuU2NoZW1hSW5mb0gAEjgKCnRhYmxlX2luZm8YCCABKAsyIi5kZWx0YV9zaGFyaW5nLnRhYmxlcy52MS5UYWJsZUluZm9IABI6Cgtjb2x1bW5faW5mbxgJIAEoCzIjLmRlbHRhX3NoYXJpbmcudGFibGVzLnYxLkNvbHVtbkluZm9IABJbChZleHRlcm5hbF9sb2NhdGlvbl9pbmZvGAogASgLMjkuZGVsdGFfc2hhcmluZy5leHRlcm5hbF9sb2NhdGlvbnMudjEuRXh0ZXJuYWxMb2NhdGlvbkluZm9IABJECg5yZWNpcGllbnRfaW5mbxgLIAEoCzIqLmRlbHRhX3NoYXJpbmcucmVjaXBpZW50cy52MS5SZWNpcGllbnRJbmZvSABCCgoIcmVzb3VyY2UifwoPT2JqZWN0UmVsYXRpb25zEhIKBW93bmVyGAEgASgJSACIAQESFwoKY3JlYXRlZF9ieRgCIAEoCUgBiAEBEhcKCnVwZGF0ZWRfYnkYAyABKAlIAogBAUIICgZfb3duZXJCDQoLX2NyZWF0ZWRfYnlCDQoLX3VwZGF0ZWRfYnlC6AEKGmNvbS5kZWx0YV9zaGFyaW5nLmludGVybmFsQg5SZXNvdXJjZXNQcm90b1ABWkVnaXRodWIuY29tL2RlbHRhLWluY3ViYXRvci9kZWx0YS1zaGFyaW5nLXJzL2dvL2RlbHRhX3NoYXJpbmcvaW50ZXJuYWyiAgNESViqAhVEZWx0YVNoYXJpbmcuSW50ZXJuYWzKAhVEZWx0YVNoYXJpbmdcSW50ZXJuYWziAiFEZWx0YVNoYXJpbmdcSW50ZXJuYWxcR1BCTWV0YWRhdGHqAhZEZWx0YVNoYXJpbmc6OkludGVybmFsYgZwcm90bzM", [file_delta_sharing_catalogs_v1_models, file_delta_sharing_credentials_v1_models, file_delta_sharing_external_locations_v1_models, file_delta_sharing_recipients_v1_models, file_delta_sharing_schemas_v1_models, file_delta_sharing_shares_v1_models, file_delta_sharing_sharing_v1_models, file_delta_sharing_tables_v1_models]);

/**
 * Dummy message to hold all resources.
 *
 * @generated from message delta_sharing.internal.Resource
 */
export type Resource = Message<"delta_sharing.internal.Resource"> & {
  /**
   * @generated from oneof delta_sharing.internal.Resource.resource
   */
  resource: {
    /**
     * @generated from field: delta_sharing.shares.v1.ShareInfo share_info = 1;
     */
    value: ShareInfo;
    case: "shareInfo";
  } | {
    /**
     * @generated from field: delta_sharing.sharing.v1.SharingSchemaInfo sharing_schema_info = 2;
     */
    value: SharingSchemaInfo;
    case: "sharingSchemaInfo";
  } | {
    /**
     * @generated from field: delta_sharing.sharing.v1.SharingTable sharing_table = 3;
     */
    value: SharingTable;
    case: "sharingTable";
  } | {
    /**
     * @generated from field: delta_sharing.credentials.v1.CredentialInfo credential_info = 4;
     */
    value: CredentialInfo;
    case: "credentialInfo";
  } | {
    /**
     * @generated from field: delta_sharing.catalogs.v1.CatalogInfo catalog_info = 6;
     */
    value: CatalogInfo;
    case: "catalogInfo";
  } | {
    /**
     * @generated from field: delta_sharing.schemas.v1.SchemaInfo schema_info = 7;
     */
    value: SchemaInfo;
    case: "schemaInfo";
  } | {
    /**
     * @generated from field: delta_sharing.tables.v1.TableInfo table_info = 8;
     */
    value: TableInfo;
    case: "tableInfo";
  } | {
    /**
     * @generated from field: delta_sharing.tables.v1.ColumnInfo column_info = 9;
     */
    value: ColumnInfo;
    case: "columnInfo";
  } | {
    /**
     * @generated from field: delta_sharing.external_locations.v1.ExternalLocationInfo external_location_info = 10;
     */
    value: ExternalLocationInfo;
    case: "externalLocationInfo";
  } | {
    /**
     * @generated from field: delta_sharing.recipients.v1.RecipientInfo recipient_info = 11;
     */
    value: RecipientInfo;
    case: "recipientInfo";
  } | { case: undefined; value?: undefined };
};

/**
 * Dummy message to hold all resources.
 *
 * @generated from message delta_sharing.internal.Resource
 */
export type ResourceJson = {
  /**
   * @generated from field: delta_sharing.shares.v1.ShareInfo share_info = 1;
   */
  shareInfo?: ShareInfoJson;

  /**
   * @generated from field: delta_sharing.sharing.v1.SharingSchemaInfo sharing_schema_info = 2;
   */
  sharingSchemaInfo?: SharingSchemaInfoJson;

  /**
   * @generated from field: delta_sharing.sharing.v1.SharingTable sharing_table = 3;
   */
  sharingTable?: SharingTableJson;

  /**
   * @generated from field: delta_sharing.credentials.v1.CredentialInfo credential_info = 4;
   */
  credentialInfo?: CredentialInfoJson;

  /**
   * @generated from field: delta_sharing.catalogs.v1.CatalogInfo catalog_info = 6;
   */
  catalogInfo?: CatalogInfoJson;

  /**
   * @generated from field: delta_sharing.schemas.v1.SchemaInfo schema_info = 7;
   */
  schemaInfo?: SchemaInfoJson;

  /**
   * @generated from field: delta_sharing.tables.v1.TableInfo table_info = 8;
   */
  tableInfo?: TableInfoJson;

  /**
   * @generated from field: delta_sharing.tables.v1.ColumnInfo column_info = 9;
   */
  columnInfo?: ColumnInfoJson;

  /**
   * @generated from field: delta_sharing.external_locations.v1.ExternalLocationInfo external_location_info = 10;
   */
  externalLocationInfo?: ExternalLocationInfoJson;

  /**
   * @generated from field: delta_sharing.recipients.v1.RecipientInfo recipient_info = 11;
   */
  recipientInfo?: RecipientInfoJson;
};

/**
 * Describes the message delta_sharing.internal.Resource.
 * Use `create(ResourceSchema)` to create a new message.
 */
export const ResourceSchema: GenMessage<Resource, ResourceJson> = /*@__PURE__*/
  messageDesc(file_delta_sharing_internal_resources, 0);

/**
 * @generated from message delta_sharing.internal.ObjectRelations
 */
export type ObjectRelations = Message<"delta_sharing.internal.ObjectRelations"> & {
  /**
   * Username of current owner of table.
   *
   * @generated from field: optional string owner = 1;
   */
  owner?: string;

  /**
   * Username of table creator.
   *
   * @generated from field: optional string created_by = 2;
   */
  createdBy?: string;

  /**
   * Username of user who last modified table.
   *
   * @generated from field: optional string updated_by = 3;
   */
  updatedBy?: string;
};

/**
 * @generated from message delta_sharing.internal.ObjectRelations
 */
export type ObjectRelationsJson = {
  /**
   * Username of current owner of table.
   *
   * @generated from field: optional string owner = 1;
   */
  owner?: string;

  /**
   * Username of table creator.
   *
   * @generated from field: optional string created_by = 2;
   */
  createdBy?: string;

  /**
   * Username of user who last modified table.
   *
   * @generated from field: optional string updated_by = 3;
   */
  updatedBy?: string;
};

/**
 * Describes the message delta_sharing.internal.ObjectRelations.
 * Use `create(ObjectRelationsSchema)` to create a new message.
 */
export const ObjectRelationsSchema: GenMessage<ObjectRelations, ObjectRelationsJson> = /*@__PURE__*/
  messageDesc(file_delta_sharing_internal_resources, 1);

