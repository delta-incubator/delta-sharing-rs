import { list_catalogs, get_catalog, create_catalog } from "./tauri";
import { CreateCatalogRequestJson } from "../gen/delta_sharing/catalogs/v1/svc_pb";

export async function listCatalogs(maxResults: number | null = null) {
    return await list_catalogs(maxResults);
}

export async function getCatalog(name: string) {
    return await get_catalog(name);
}

export async function createCatalog(request: CreateCatalogRequestJson) {
    return create_catalog(request);
}

export default {
    listCatalogs,
    getCatalog,
    createCatalog,
};
