import type { Discussion } from "$lib/types";
import { fetchIntoResult } from "$lib/utils.js";
import { error, redirect } from "@sveltejs/kit";

export const load = async ({ fetch, url, params, parent }) => {
    const response = await fetchIntoResult<Array<Discussion>>(() => fetch("/api/discussions"));
    if (response.value) {
        const discussions = response.value;
        return {
            discussions: discussions as Array<Discussion>,
        }
    }
    if (response.error?.clientError) {
        error(response.error.clientError.status)
    }
};