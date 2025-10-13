import { processTaskJson } from "./task";
import type { TaskJSON, TaskGET, TaskPOST } from "./task";

export interface ProjectJSON {
    id: number;
    user_id: number;
    title: string;
    budget: number;
    currency_code: string;
    content: string;
    created_at: string;
    deadline: string;
    tasks?: Array<TaskJSON>;
}

export interface ProjectGET {
    id?: number;
    user_id: string;
    title: string;
    content: string;
    budget: number;
    currency_code: string;
    deadline: Date;
    created_at: Date;
    tasks?: Array<TaskGET>;
}

export interface ProjectPOST {
    id?: number;
    title: string;
    content: string;
    budget: number;
    deadline: string;
    tasks?: Array<TaskPOST>;
}

// used to fill out the form since everything is a string there
export interface ProjectForm {
    title: string;
    content: string;
    budget: number;
    deadline: string;
}

export function processProjectJson(json: ProjectJSON): ProjectGET {
    const project: ProjectGET = {
        id: json.id,
        user_id: json.user_id.toString(),
        title: json.title,
        budget: json.budget,
        currency_code: json.currency_code,
        content: json.content,
        created_at: new Date(json.created_at),
        deadline: new Date(json.deadline),
        tasks: json.tasks?.map((task) => processTaskJson(task)),
    }
    return project;
}

