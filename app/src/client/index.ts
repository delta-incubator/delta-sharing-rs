import tauri from "./tauri";
import { CreateCatalogRequestJson } from "../gen/delta_sharing/catalogs/v1/svc_pb";
import type { CatalogInfoJson } from "../gen/delta_sharing/catalogs/v1/models_pb";
import type { SchemaInfoJson } from "../gen/delta_sharing/schemas/v1/models_pb";
import { CreateSchemaRequestJson } from "../gen/delta_sharing/schemas/v1/svc_pb";

export type {
    CatalogInfoJson as CatalogInfo,
    SchemaInfoJson as SchemaInfo,
    CreateCatalogRequestJson as CreateCatalogRequest,
    CreateSchemaRequestJson as CreateSchemaRequest,
};

export async function listCatalogs(maxResults: number | null = null) {
    return await tauri.list_catalogs(maxResults);
}

export async function getCatalog(name: string) {
    return await tauri.get_catalog(name);
}

export async function createCatalog(request: CreateCatalogRequestJson) {
    return await tauri.create_catalog(request);
}

export async function deleteCatalog(name: string) {
    return await tauri.delete_catalog(name);
}

export async function listSchemas(catalog: string) {
    return await tauri.list_schemas(catalog);
}

export async function createSchema(request: CreateSchemaRequestJson) {
    return await tauri.create_schema(request);
}

export async function getSchema(catalog: string, name: string) {
    return await tauri.get_schema(catalog, name);
}

export async function deleteSchema({
    catalog,
    name,
}: {
    catalog: string;
    name: string;
}) {
    return await tauri.delete_schema(catalog, name);
}

export default {
    catalogs: {
        list: listCatalogs,
        get: getCatalog,
        create: createCatalog,
        delete: deleteCatalog,
    },
    schemas: {
        list: listSchemas,
        get: getSchema,
        create: createSchema,
        delete: deleteSchema,
    },
};
