interface ProjectObject {
    id: number;
    user_id: string;
    title: string;
    content: string;
    budget: number;
    currency_code: string;
    deadline: Date;
    created_at: Date;
}

// TODO make this a type instead
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

interface TaskObject {
    id?: number,
    title: string,
    assignee_id: string,
    status: string,
    description: string,
    // deadline: Date,
    // created_at: Date,
    // completed_at: Date | null
}

// interface TaskObject {
//     id?: number;
//     title: string;
//     content: string;
//     assignee: string;
//     skills: string;
//     deadline: string;
//     estimatedEfforts: string;
// }