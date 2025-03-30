import type { TaskGET } from "$lib/types/task.js";
import { fetchIntoResult } from "$lib/utils.js";

interface ProposalJSON {
    id: number,
    user_id: string,
    task_id: number,
    status: string,
    budget: number | undefined,
    content: string | undefined,
    created_at: string,
}

export interface ProposalGET {
    id: number,
    user_id: string,
    task_id: number,
    status: string,
    budget: number | undefined,
    content: string | undefined,
    created_at: Date,
}

function processProposalJSON(json: ProposalJSON) {
    const result: ProposalGET = {
        ...json,
        created_at: new Date(json.created_at)
    }
    return result;
}

export interface TaskProposalsGET {
    task: TaskGET | undefined,
    proposals: ProposalGET[]
}

export async function load({ fetch, params, parent }) {
    const response = await fetchIntoResult<ProposalJSON[]>(() => fetch(`/api/projects/${params.id}/task/${params.task_id}/proposals`, { method: "GET" }));
    const praseProposalJSON = (jsonData: ProposalJSON[]) => jsonData.map((json) => processProposalJSON(json));
    const maybeProposals = response.map((value: ProposalJSON[]) => praseProposalJSON(value))
    const parentData = await parent();
    const maybeTaskProposals = maybeProposals.map((value: ProposalGET[]) => {
        return {
            task: parentData.project?.tasks?.find((task) => task.id.toString() === params.task_id),
            proposals: value
        } as TaskProposalsGET;
    })
    return {
        error: maybeProposals.error,
        proposals: maybeTaskProposals.value?.proposals,
        task: maybeTaskProposals.value?.task
    };
}