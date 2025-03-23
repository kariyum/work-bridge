import type { Snippet } from "svelte";
import type { NetworkError, TimeoutError, ClientError, ServerError, ParsingError, UnauthorizedError } from "./utils";

export interface User {
    email: string,
    role: string
}
export type NotificationType = "message" | "proposal";

export type ProposalStatus = "rejected" | "approved" | "pending";

export interface BaseNotification {
    notification_type: NotificationType,
    created_at: Date
}

export interface MessagesJsonResponse extends BaseNotification {
    id: number,
    from_user_id: string,
    content: string,
    discussion_id: number,
};

export interface ProposalNotification extends BaseNotification {
    id: number,
    content: {
        proposal_id: string,
        proposal_status: string,
        task_id: number,
        project_id: number,
        trigger_user_id: string
    },
    user_id: string,
}

export class ProposalNotification {
    static getHref(notif: ProposalNotification) {
        // reroute to project and scroll to the task
        return `/project/${notif.content.project_id}#${notif.content.task_id}`;
    }

    static getContent(notif: ProposalNotification) {
        return `${notif.content.trigger_user_id} ${notif.content.proposal_status} your application.`;
    }
}

export type ClientMessage = {
    discussion_id: number,
    content: string,
    receivers: string[],
}

export interface Discussion {
    id: number;
    title: string;
    created_at: string;
    created_by: string;
    user_ids: Array<string>
}

export type FetchErrors = {
    networkError?: NetworkError;
    timeoutError?: TimeoutError;
    clientError?: ClientError;
    serverError?: ServerError;
    parsingError?: ParsingError;
    unauthorizedError?: UnauthorizedError;
}

export interface TaskForm {
    title: string;
    assignee_id: string;
    status: string;
    content: string;
    deadline: string;
    budget?: number;
    skills: string[];
    id?: number;
}

export interface Tab {
    snippet: Snippet;
    title: string;
    url: string,
    tab: string
}