// when this API is invalidated for some reason 
// the component is not re-rendered if the load function runs only on the server

export async function load({ fetch, params }) {
    // if (!cookies.get("Authorization")) {
    //     return {
    //         status: 401,
    //         error: "You are not authorized to view this page",
    //         projects: [] as Array<Project>
    //     }
    // }
    const response = await fetch("/api/projects", { method: "GET" });
    if (response.status == 401) {
        return {
            status: response.status,
            error: "You are not authorized to view this page",
            projects: [] as Array<ProjectObject>
        }
    }
    if (response.status == 200) {
        try {
            const data = await response.json();
            return {
                status: response.status,
                projects: data as Array<ProjectObject>
            }
        } catch (error) {
            return {
                status: response.status,
                error: `An error occurred while fetching projects: ${error}`,
                projects: [] as Array<ProjectObject>
            }
        }
    } else {
        return {
            status: response.status,
            error: "You are not authorized to view this page",
            projects: [] as Array<ProjectObject>
        }
    }
}