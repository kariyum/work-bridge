export const ssr = false;

export const load = async ( {fetch} ) => {
    // const response = await fetch("/api/messages");
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
        ]
    }
};