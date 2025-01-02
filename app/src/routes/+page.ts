// when this API is invalidated for some reason 
// the component is not re-rendered if the load function runs only on the server

import { processProjectJson, type ProjectGET, type ProjectJSON } from "$lib/types/project";

export async function load({ parent, fetch }: { parent: Function, fetch: Function }) {
    const { status } = await parent() as {status: number};
    if (status === 401) {
        return { 
            result: { 
                projects: [] as ProjectGET[], error: undefined 
            } 
        };
    }
    try {
        const response = await fetch("/api/projects", { method: "GET" });

        if (!response.ok) {
            return { 
                result: { 
                    projects: [] as ProjectGET[], error: undefined 
                } 
            };
        }

        const jsonData: Array<ProjectJSON> = await response.json();
        const projects = jsonData.map(processProjectJson);

        return { 
            result: { 
                projects: projects as ProjectGET[], error: undefined 
            } 
        };
    } catch (exception) {
        console.error(exception);
        return { 
            result: { 
                projects: [] as ProjectGET[], error: exception 
            } 
        };
    }
}