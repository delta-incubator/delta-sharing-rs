import tauri from "./tauri";
import type { CreateCatalogRequestJson } from "../gen/delta_sharing/catalogs/v1/svc_pb";
import type { CatalogInfoJson } from "../gen/delta_sharing/catalogs/v1/models_pb";
import type { SchemaInfoJson } from "../gen/delta_sharing/schemas/v1/models_pb";
import type { CreateSchemaRequestJson } from "../gen/delta_sharing/schemas/v1/svc_pb";
import type {
    CredentialInfoJson,
    PurposeJson,
} from "../gen/delta_sharing/credentials/v1/models_pb";
import type { CreateCredentialRequestJson } from "../gen/delta_sharing/credentials/v1/svc_pb";
import type { ExternalLocationInfoJson } from "../gen/delta_sharing/external_locations/v1/models_pb";
import type { CreateExternalLocationRequestJson } from "../gen/delta_sharing/external_locations/v1/svc_pb";

export type {
    CatalogInfoJson as CatalogInfo,
    SchemaInfoJson as SchemaInfo,
    CreateCatalogRequestJson as CreateCatalogRequest,
    CreateSchemaRequestJson as CreateSchemaRequest,
    CredentialInfoJson as CredentialInfo,
    CreateCredentialRequestJson as CreateCredentialRequest,
    ExternalLocationInfoJson as ExternalLocationInfo,
    CreateExternalLocationRequestJson as CreateExternalLocationRequest,
    PurposeJson as Purpose,
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

export async function listCredentials(maxResults: number | null = null) {
    return await tauri.list_credentials(maxResults);
}

export async function getCredential(name: string) {
    return await tauri.get_credential(name);
}

export async function createCredential(request: CreateCredentialRequestJson) {
    return await tauri.create_credential(request);
}

export async function deleteCredential(name: string) {
    return await tauri.delete_credential(name);
}

export async function listExternalLocations(maxResults: number | null = null) {
    return await tauri.list_external_locations(maxResults);
}

export async function getExternalLocation(name: string) {
    return await tauri.get_external_location(name);
}

export async function createExternalLocation(
    request: CreateExternalLocationRequestJson,
) {
    return await tauri.create_external_location(request);
}

export async function deleteExternalLocation(name: string) {
    return await tauri.delete_external_location(name);
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
    credentials: {
        list: listCredentials,
        get: getCredential,
        create: createCredential,
        delete: deleteCredential,
    },
    externalLocations: {
        list: listExternalLocations,
        get: getExternalLocation,
        create: createExternalLocation,
        delete: deleteExternalLocation,
    },
};
