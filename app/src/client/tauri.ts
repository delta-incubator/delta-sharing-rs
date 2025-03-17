import { invoke } from "@tauri-apps/api/core";
import { CatalogInfoJson } from "../gen/delta_sharing/catalogs/v1/models_pb";
import { CreateCatalogRequestJson } from "../gen/delta_sharing/catalogs/v1/svc_pb";
import { SchemaInfoJson } from "../gen/delta_sharing/schemas/v1/models_pb";
import { CreateSchemaRequestJson } from "../gen/delta_sharing/schemas/v1/svc_pb";
import { CredentialInfoJson } from "../gen/delta_sharing/credentials/v1/models_pb";
import { CreateCredentialRequestJson } from "../gen/delta_sharing/credentials/v1/svc_pb";
import { ExternalLocationInfoJson } from "../gen/delta_sharing/external_locations/v1/models_pb";
import { CreateExternalLocationRequestJson } from "../gen/delta_sharing/external_locations/v1/svc_pb";
import { RecipientInfoJson } from "../gen/delta_sharing/recipients/v1/models_pb";
import { CreateRecipientRequestJson } from "../gen/delta_sharing/recipients/v1/svc_pb";
import { ShareInfoJson } from "../gen/delta_sharing/shares/v1/models_pb";
import { CreateShareRequestJson } from "../gen/delta_sharing/shares/v1/svc_pb";
import {
    TableInfoJson,
    TableSummaryJson,
} from "../gen/delta_sharing/tables/v1/models_pb";
import { CreateTableRequestJson } from "../gen/delta_sharing/tables/v1/svc_pb";

export async function list_catalogs(maxResults?: number) {
    return await invoke<CatalogInfoJson[]>("list_catalogs", { maxResults });
}

export async function create_catalog(request: CreateCatalogRequestJson) {
    return await invoke<CatalogInfoJson>("create_catalog", { request });
}

export async function get_catalog(name: string) {
    return await invoke<CatalogInfoJson>("get_catalog", { name });
}

export async function delete_catalog(name: string) {
    return await invoke<void>("delete_catalog", { name });
}

export async function list_schemas(catalog: string, maxResults?: number) {
    console.log("list_schemas", { catalog, maxResults });
    return await invoke<SchemaInfoJson[]>("list_schemas", { catalog });
}

export async function create_schema(request: CreateSchemaRequestJson) {
    console.log("create_schema", { request });
    return await invoke<SchemaInfoJson>("create_schema", { request });
}

export async function get_schema(catalog: string, name: string) {
    return await invoke<SchemaInfoJson>("get_schema", { catalog, name });
}

export async function delete_schema(catalog: string, name: string) {
    return await invoke<void>("delete_schema", { catalog, name });
}

export async function list_credentials(maxResults?: number) {
    return await invoke<CredentialInfoJson[]>("list_credentials", {
        maxResults,
    });
}

export async function create_credential(request: CreateCredentialRequestJson) {
    return await invoke<CredentialInfoJson>("create_credential", { request });
}

export async function get_credential(name: string) {
    return await invoke<CredentialInfoJson>("get_credential", { name });
}

export async function delete_credential(name: string) {
    return await invoke<void>("delete_credential", { name });
}

export async function list_external_locations(maxResults?: number) {
    return await invoke<ExternalLocationInfoJson[]>("list_external_locations", {
        maxResults,
    });
}

export async function create_external_location(
    request: CreateExternalLocationRequestJson,
) {
    return await invoke<ExternalLocationInfoJson>("create_external_location", {
        request,
    });
}

export async function get_external_location(name: string) {
    return await invoke<ExternalLocationInfoJson>("get_external_location", {
        name,
    });
}

export async function delete_external_location(name: string) {
    return await invoke<void>("delete_external_location", { name });
}

export async function list_recipients(maxResults?: number) {
    return await invoke<RecipientInfoJson[]>("list_recipients", { maxResults });
}

export async function create_recipient(request: CreateRecipientRequestJson) {
    return await invoke<RecipientInfoJson>("create_recipient", { request });
}

export async function get_recipient(name: string) {
    return await invoke<RecipientInfoJson>("get_recipient", { name });
}

export async function delete_recipient(name: string) {
    return await invoke<void>("delete_recipient", { name });
}

export async function list_shares(maxResults?: number) {
    return await invoke<ShareInfoJson[]>("list_shares", { maxResults });
}

export async function create_share(request: CreateShareRequestJson) {
    return await invoke<ShareInfoJson>("create_share", { request });
}

export async function get_share(name: string, includeSharedData?: boolean) {
    return await invoke<ShareInfoJson>("get_share", {
        name,
        includeSharedData,
    });
}

export async function delete_share(name: string) {
    return await invoke<void>("delete_share", { name });
}

export async function list_table_summaries(
    catalog: string,
    schemaPattern?: string,
    tablePattern?: string,
    maxResults?: number,
) {
    return await invoke<TableSummaryJson[]>("list_table_summaries", {
        catalog,
        schemaPattern,
        tablePattern,
        maxResults,
    });
}

export async function list_tables(
    catalog: string,
    schema: string,
    maxResults?: number,
) {
    return await invoke<TableInfoJson[]>("list_tables", {
        catalog,
        schema,
        maxResults,
    });
}

export async function create_table(request: CreateTableRequestJson) {
    return await invoke<TableInfoJson>("create_table", { request });
}

export async function get_table(catalog: string, schema: string, name: string) {
    return await invoke<TableInfoJson>("get_table", { catalog, schema, name });
}

export async function delete_table(
    catalog: string,
    schema: string,
    name: string,
) {
    return await invoke<void>("delete_table", {
        fullName: `${catalog}.${schema}.${name}`,
    });
}

export default {
    list_catalogs,
    create_catalog,
    get_catalog,
    delete_catalog,
    list_schemas,
    create_schema,
    get_schema,
    delete_schema,
    list_credentials,
    create_credential,
    get_credential,
    delete_credential,
    list_external_locations,
    create_external_location,
    get_external_location,
    delete_external_location,
    list_recipients,
    create_recipient,
    get_recipient,
    delete_recipient,
    list_shares,
    create_share,
    get_share,
    delete_share,
    list_table_summaries,
    list_tables,
    create_table,
    get_table,
    delete_table,
};
