import { invoke } from "@tauri-apps/api/core";
import { CatalogInfoJson } from "../gen/delta_sharing/catalogs/v1/models_pb";
import { SchemaInfoJson } from "../gen/delta_sharing/schemas/v1/models_pb";
import { CreateCatalogRequestJson } from "../gen/delta_sharing/catalogs/v1/svc_pb";
import { CreateSchemaRequestJson } from "../gen/delta_sharing/schemas/v1/svc_pb";

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

export default {
    list_catalogs,
    create_catalog,
    get_catalog,
    delete_catalog,
    list_schemas,
    create_schema,
    get_schema,
    delete_schema,
};
