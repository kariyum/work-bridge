// when this API is invalidated for some reason 
// the component is not re-rendered if the load function runs only on the server

import { processProjectJson, type ProjectGET, type ProjectJSON } from "$lib/types/project";

export async function load({ fetch }) {
    const response = fetch("/api/projects", { method: "GET" });
    const parseProjectJSON = (jsonData: Array<ProjectJSON>) => jsonData.map((json) => processProjectJson(json));
    const result = response
        .then((response) => {
            if (response.ok) {
                return response.json()
            } else {
                return Promise.resolve([]);
            }
        })
        .then((jsonData: Array<ProjectJSON>) => Promise.resolve(parseProjectJSON(jsonData)))
        .then((projects) => Promise.resolve({
            projects: projects as ProjectGET[],
            error: undefined
        }))
        .catch((exception) => {
            console.error(exception);
            return Promise.resolve({
                projects: [] as ProjectGET[],
                error: exception
            });
        })
    return {
        result: result
    };
}