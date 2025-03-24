import type { Discussion } from "$lib/types";
import { fetchIntoResult } from "$lib/utils.js";
import { error, redirect } from "@sveltejs/kit";

export const load = async ({ fetch, url, params, parent }) => {
    const response = await fetchIntoResult<Array<Discussion>>(() => fetch("/api/discussions"));
    if (response.value) {
        const discussions = response.value;
        const parentData = await parent();
        const targetDiscussionUserId = url.searchParams.get("user_id");
        const currentUserId = parentData.user?.email;
        if (targetDiscussionUserId && currentUserId) {
            const maybeDiscussionId = discussions
                .find((discussion) =>
                    discussion.user_ids.includes(targetDiscussionUserId) &&
                    discussion.user_ids.includes(currentUserId) &&
                    discussion.user_ids.length === 2
                )?.id;
            if (maybeDiscussionId) {
                redirect(303, `/messages/${maybeDiscussionId}`);
            }
        }
        return {
            discussions: discussions as Array<Discussion>,
            discussion_id: params.id
        }
    }
    if (response.error?.clientError) {
        error(response.error.clientError.status)
    }
};