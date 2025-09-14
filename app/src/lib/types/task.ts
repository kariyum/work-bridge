import type { ProposalStatus } from "$lib/types";

export interface TaskJSON {
    id: number,
    project_id: number,
    title: string,
    content: string,
    assignee_id: string,
    budget: number,
    status: string,
    deadline: string,
    created_at: string,
    skills: string[],
    proposal_status: string | undefined,
    proposal_id: number | undefined
}

export interface TaskGET {
    id: number,
    project_id: number,
    title: string,
    content: string,
    assignee_id: string,
    budget: number,
    status: string,
    skills: string[],
    deadline: Date,
    created_at: Date,
    proposal_status: ProposalStatus | undefined,
    proposal_id: number | undefined
}

export function processTaskJson(json: TaskJSON) {
    const result: TaskGET = {
        id: json.id,
        project_id: json.project_id,
        title: json.title,
        content: json.content,
        assignee_id: json.assignee_id,
        budget: json.budget,
        status: json.status,
        deadline: new Date(json.deadline),
        created_at: new Date(json.created_at),
        skills: json.skills,
        proposal_status: json.proposal_status as ProposalStatus,
        proposal_id: json.proposal_id
    };
    return result;
}

export interface TaskPOST {
    title: string,
    content: string,
    assignee_id: string,
    skills: string[]
    budget: number,
    status: string,
    deadline: string,
}