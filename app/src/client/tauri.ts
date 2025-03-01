import { invoke } from "@tauri-apps/api/core";

export async function list_catalogs(maxResults: number | null = null) {
    return await invoke("list_catalogs", { maxResults });
}
