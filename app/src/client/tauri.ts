import { invoke } from "@tauri-apps/api/core";
import { CatalogInfoJson } from "../gen/delta_sharing/catalogs/v1/models_pb";
import { CreateCatalogRequestJson } from "../gen/delta_sharing/catalogs/v1/svc_pb";
import { SchemaInfoJson } from "../gen/delta_sharing/schemas/v1/models_pb";
import { CreateSchemaRequestJson } from "../gen/delta_sharing/schemas/v1/svc_pb";
import { CredentialInfoJson } from "../gen/delta_sharing/credentials/v1/models_pb";
import { CreateCredentialRequestJson } from "../gen/delta_sharing/credentials/v1/svc_pb";
import { ExternalLocationInfoJson } from "../gen/delta_sharing/external_locations/v1/models_pb";
import { CreateExternalLocationRequestJson } from "../gen/delta_sharing/external_locations/v1/svc_pb";

export async function list_catalogs(maxResults: number | null = null) {
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

export async function list_schemas(catalog: string) {
    console.log("list_schemas", { catalog });
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

export async function list_credentials(maxResults: number | null = null) {
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

export async function list_external_locations(
    maxResults: number | null = null,
) {
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
};
