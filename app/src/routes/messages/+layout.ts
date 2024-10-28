export const ssr = false;

interface Discussion {
    id: number;
    title: string;
    created_at: string;
    created_by: string;
    user_ids: Array<string>
}

export const load = async ({ fetch }) => {
    const response = await fetch("/api/discussions");
    if (response.status === 401) {
        return {
            error: "Unauthorized",
            status: 401,
            discussions: [] as Array<Discussion>
        }
    }
    try {
        const discussions = await response.json();
        if (response.ok) {
            return {
                discussions: discussions as Array<Discussion>,
                status: response.status,
            }
        } else {
            return {
                error: "Something went wrong",
                discussions: [] as Array<Discussion>,
                status: response.status
            }
        }
    } catch (error) {
        return {
            error: `An error occurred while fetching discussions: ${error}`,
            status: response.status,
            discussions: [] as Array<Discussion>
        }
    }
};