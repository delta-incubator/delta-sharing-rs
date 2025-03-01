import { invoke } from "@tauri-apps/api/core";
import { CatalogInfoJson } from "../gen/delta_sharing/catalogs/v1/models_pb";
import { CreateCatalogRequestJson } from "../gen/delta_sharing/catalogs/v1/svc_pb";

export async function list_catalogs(maxResults: number | null = null) {
    return await invoke<CatalogInfoJson[]>("list_catalogs", { maxResults });
}

export async function create_catalog(request: CreateCatalogRequestJson) {
    return await invoke<CatalogInfoJson>("create_catalog", { request });
}

export async function get_catalog(name: string) {
    return await invoke<CatalogInfoJson>("get_catalog", { name });
}
