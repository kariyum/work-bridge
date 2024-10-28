export const ssr = false;

export async function load({ fetch, params }) {
    const response = await fetch("/api/projects", { method: "GET" });
    if (response.status == 401) {
        return {
            status: response.status,
            error: "You are not authorized to view this page",
            projects: [] as Array<Project>
        }
    }
    if (response.status == 200) {
        try {
            const data = await response.json();
            return {
                status: response.status,
                projects: data as Array<Project>
            }
        } catch (error) {
            return {
                status: response.status,
                error: `An error occurred while fetching projects: ${error}`,
                projects: [] as Array<Project>
            }
        }
    } else {
        return {
            status: response.status,
            error: "You are not authorized to view this page",
            projects: [] as Array<Project>
        }
    }
}