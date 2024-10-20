export const ssr = false;
interface Discussion {
    id: number;
    title: string;
    created_at: string;
    created_by: string;
    user_ids: Array<string>
}
export const load = async () => {
    const response = await fetch("/api/discussions");
    try {
        const discussions = await response.json();
        console.log("response", response);
        if (response.ok) {
            return {
                discussions: discussions as Array<Discussion>,
            }
        } else {
            return {
                error: "You are not authorized to view this page",
                discussions: [] as Array<Discussion>
            }
        }
    } catch (error) {
        return {
            error: "An error occurred",
            discussions: [] as Array<Discussion>
        }
    }
};