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
    application_submitted: boolean,
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
    application_submitted: boolean
}

export function processTaskJson(json: TaskJSON) {
    let result: TaskGET = {
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
        application_submitted: json.application_submitted
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