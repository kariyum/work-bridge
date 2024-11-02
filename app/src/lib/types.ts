interface Project {
    id: number;
    user_id: string;
    title: string;
    content: string;
    budget: number;
    currency_code: string;
    deadline: Date;
    created_at: Date;
}

interface User {
    email: String,
    role: String
}

type Message = {
    sender: string;
    content: string;
};