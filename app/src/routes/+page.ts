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
    const processData = (jsonData: any) => {
        const result: Array<ProjectObject> = jsonData.map((json: any) => {
            const x: ProjectObject = {
                id: json.id,
                user_id: json.user_id,
                title: json.title,
                budget: json.budget,
                currency_code: json.currency_code,
                content: json.content,
                created_at: new Date(json.created_at),
                deadline: new Date(json.deadline)
            }
            return x;
        });
        return result;
    }
    if (response.status == 200) {
        try {
            const data = await response.json();
            return {
                status: response.status,
                projects: processData(data)
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