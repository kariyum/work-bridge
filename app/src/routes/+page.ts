// when this API is invalidated for some reason 
// the component is not re-rendered if the load function runs only on the server

export async function load({ fetch }) {
    const response = fetch("/api/projects", { method: "GET" });
    interface JSONProject {
        id: number;
        user_id: number;
        title: string;
        budget: number;
        currency_code: string;
        content: string;
        created_at: string;
        deadline: string;
    }
    const parseProjectJSON = (jsonData: Array<JSONProject>) => {
        const result: Array<ProjectObject> = jsonData.map((json: JSONProject) => {
            const project: ProjectObject = {
                id: json.id,
                user_id: json.user_id.toString(),
                title: json.title,
                budget: json.budget,
                currency_code: json.currency_code,
                content: json.content,
                created_at: new Date(json.created_at),
                deadline: new Date(json.deadline)
            }
            return project;
        });
        return result;
    };
    const result = response
        .then((response) => {
            if (response.ok) {
                return response.json() as Promise<Array<JSONProject>>
            } else {
                return Promise.resolve([] as Array<JSONProject>);
            }
        })
        .then((jsonData) => Promise.resolve(parseProjectJSON(jsonData)))
        .then((projects) => Promise.resolve({
            projects: projects,
            error: undefined
        }))
        .catch((exception) => {
            console.error(exception);
            return Promise.resolve({
                projects: [] as Array<ProjectObject>,
                error: exception
            });
        })
    return {
        result: result
    };
}