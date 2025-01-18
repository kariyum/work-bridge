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
        if (!WebSocketService.instance) {
            WebSocketService.instance = new WebSocketService("/api/chat");
        }
        return WebSocketService.instance;
    }

    public subscribe(handler: (data: MessagesJsonResponse) => void) {
        this.subscribers.push(handler);
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
        }
    }

    public close(): void {
        if (this.socket) {
            this.socket.close();
        }
    }
}