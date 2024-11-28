interface User {
    email: string,
    role: string
}

type MessagesJsonResponse = {
    id: number,
    from_user_id: string,
    content: string,
    created_at: string
};

type ClientMessage = {
    disucssion_id: number,
    content: string,
}