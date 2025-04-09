import type { Snippet } from "svelte";
import type { NetworkError, TimeoutError, ClientError, ServerError, ParsingError, UnauthorizedError, NotFound } from "./utils";

export interface User {
    email: string,
    role: "freelancer" | "recruiter"
}
export type NotificationType = "message" | "proposal" | "new_proposal";

export type ProposalStatus = "declined" | "approved" | "pending" | "cancelled";

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

export interface NewProposalNotification extends BaseNotification {
    id: number,
    content: {
        task_id: number,
        trigger_user_id: string
        proposal_id: string,
        project_id: number
    },
    user_id: string,
}

export class NewProposalNotificationImpl {
    static getHref(notif: NewProposalNotification) {
        // reroute to project and scroll to the task
        return `/project/${notif.content.project_id}/task/${notif.content.task_id}#${notif.content.proposal_id}`;
    }

    static getContent(notif: NewProposalNotification) {
        return `${notif.content.trigger_user_id} submitted his application!`;
    }
}

export class ProposalNotificationImpl {
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
    notFound?: NotFound;
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