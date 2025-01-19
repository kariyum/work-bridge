// when this API is invalidated for some reason 
// the component is not re-rendered if the load function runs only on the server

import { processProjectJson, type ProjectGET, type ProjectJSON } from "$lib/types/project";
import { fetchIntoResult } from "$lib/utils.js";

export async function load({ url, fetch }) {
    const response = await fetchIntoResult<ProjectJSON[]>(() => fetch("/api/projects", { method: "GET" }));
    const parseProjectJSON = (jsonData: ProjectJSON[]) => jsonData.map((json) => processProjectJson(json));
    const projects = response.map((value: ProjectJSON[]) => parseProjectJSON(value))

    return {
        error: projects.error,
        projects: projects.value as ProjectGET[],
    };
}