
export const load = async ( {fetch, params} ) => {
    // const response = await fetch("/api/messages");
    console.log("Fetching messages", params.id);
    return await Promise.resolve({
        discussion_id: params.id,
        messages: [
            {
                sender: "me",
                content: "Hello"
            },
            {
                sender: "Jane",
                content: "Hi"
            },
            {
                sender: "me",
                content: "How are you?"
            },
            {
                sender: "Jane",
                content: "I'm good, you?"
            },
            {
                sender: "me",
                content: "I'm good too"
            },
            {
                sender: "Jane",
                content: "That's great to hear"
            },
            {
                sender: "me",
                content: "I know right"
            },
            {
                sender: "Jane",
                content: "I have to go now, bye"
            },
            {
                sender: "me",
                content: "Bye"
            },
            {
                sender: "Jane",
                content: "That's great to hear"
            },
            {
                sender: "me",
                content: "I know right"
            },
            {
                sender: "Jane",
                content: "I have to go now, bye"
            },
            {
                sender: "me",
                content: "Bye"
            },
            {
                sender: "Jane",
                content: "That's great to hear"
            },
            {
                sender: "me",
                content: "I know right"
            },
            {
                sender: "Jane",
                content: "I have to go now, bye"
            },
            {
                sender: "me",
                content: "Bye"
            },
            {
                sender: "me",
                content: params.id
            },
        ]
    });
};