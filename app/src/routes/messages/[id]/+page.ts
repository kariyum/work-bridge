export const ssr = false;

export const load = async () => {
    // const response = await fetch("/api/discussions");
    type Message = {
		sender: string,
		content: string
	};
    return {
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
            }
        ]
    }
};