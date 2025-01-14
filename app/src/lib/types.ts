import type { NetworkError, TimeoutError, ClientError, ServerError, ParsingError, UnauthorizedError } from "./utils";

export interface User {
    email: string,
    role: string
}

export type MessagesJsonResponse = {
    id: number,
    from_user_id: string,
    content: string,
    created_at: string
};

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
