export interface TaskJSON {
    id: number,
    project_id: number,
    title: string,
    content: string,
    assignee_id: string,
    bdget: number,
    deadline: string,
    created_at: string,
}

export interface TaskGET {
    id: number,
    project_id: number,
    title: string,
    content: string,
    assignee_id: string,
    budget: number,
    deadline: Date,
    created_at: Date,
}

export function processTaskJson(json: TaskJSON) {
    let result: TaskGET = {
        id: json.id,
        project_id: json.project_id,
        title: json.title,
        content: json.content,
        assignee_id: json.assignee_id,
        budget: json.bdget,
        deadline: new Date(json.deadline),
        created_at: new Date(json.created_at),
    };
    return result;
}

export interface TaskPOST {
    id: number,
    project_id: number,
    title: string,
    content: string,
    assignee_id: string,
    bdget: number,
    deadline: string,
    created_at: string,
}