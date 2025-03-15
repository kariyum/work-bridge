import type { MessagesJsonResponse } from "./types";

export class WebSocketService {
    private static instance: WebSocketService;
    private socket: WebSocket;
    private subscribers: ((data: MessagesJsonResponse) => void)[] = [];
    
    private constructor(url: string) {
        this.socket = new WebSocket(url);
        this.socket.onmessage = (event) => {
			const wsMessage = JSON.parse(event.data);
			const message: MessagesJsonResponse = {
				id: Math.random(),
				from_user_id: wsMessage.sender_id,
				content: wsMessage.content,
                discussion_id: wsMessage.discussion_id,
				created_at: new Date().toISOString()
			};
            this.subscribers.forEach(handler => handler(message))
		};
        console.log("Socket?", this.socket.readyState);
        this.socket.onopen = (event: Event) => {
            console.log("Socket is open", event);
        };

        this.socket.onerror = (event: Event) => {
            console.error("Socket encountered an error");
        }
    }

    public static getInstance(): WebSocketService {
        console.log(WebSocketService.instance);
        if (!WebSocketService.instance) {
            WebSocketService.instance = new WebSocketService("/api/push_events");
        }
        return WebSocketService.instance.ensureOpen();
    }

    public subscribe(handler: (data: MessagesJsonResponse) => void) {
        this.subscribers.push(handler);
        console.log("component subscribed", this.subscribers.length);
        return () => {
            console.log("Socket unsubscribed.");
            this.subscribers = this.subscribers.filter((h) => h != handler);
        };
    }

    public send(data: string): void {
        if (this.socket.readyState == WebSocket.OPEN) {
            this.socket.send(data);
        } else {
            console.error("Websocket is closed. Unable to send a message.");
            // maybe add better error handling, propagation??
            this.ensureOpen().socket.send(data);
        }
    }

    public close(): void {
        if (this.socket) {
            this.socket.close();
        }
    }

    private ensureOpen(): WebSocketService {
        if (this.socket.readyState === this.socket.CLOSED) {
            WebSocketService.instance = new WebSocketService("/api/push_events");
        }
        return this;
    }
}