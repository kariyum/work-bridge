import type { MessagesJsonResponse, ProposalNotification, NotificationType, NewProposalNotification } from "./types";

export class WebSocketService {
    private static instance: WebSocketService;
    private socket: WebSocket;
    private onMessage: ((data: MessagesJsonResponse) => void)[] = [];
    private onProposalNotificationHandlers: ((data: ProposalNotification) => void)[] = [];
    private onNewProposalsNotificationHandlers: ((data: NewProposalNotification) => void)[] = [];

    private constructor(url: string) {
        this.socket = new WebSocket(url);
        this.socket.onmessage = (event) => {
            const wsMessage = JSON.parse(event.data);
            const notificationType: NotificationType = wsMessage.notification_type;

            switch (notificationType) {
                case "message":
                    const message: MessagesJsonResponse = {
                        id: Math.random(),
                        from_user_id: wsMessage.sender_id,
                        content: wsMessage.content,
                        discussion_id: wsMessage.discussion_id,
                        created_at: new Date(),
                        notification_type: wsMessage.notification_type
                    };
                    this.onMessage.forEach(handler => handler(message))
                    break;

                case "proposal":
                    const proposalNotification: ProposalNotification = {
                        ...wsMessage,
                        created_at: new Date(wsMessage.created_at)
                    };
                    this.onProposalNotificationHandlers.forEach(handler => handler(proposalNotification));
                    break;

                case "new_proposal":
                    const newProposalNotification: NewProposalNotification = {
                        ...wsMessage,
                        created_at: new Date(wsMessage.created_at)
                    };
                    this.onNewProposalsNotificationHandlers.forEach(handler => handler(newProposalNotification));
                    break;
                default:
                    break;
            }

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

    public subscribeToChatMessages(handler: (data: MessagesJsonResponse) => void) {
        this.onMessage.push(handler);
        console.log("component subscribed", this.onMessage.length);
        return () => {
            console.log("Socket unsubscribed.");
            this.onMessage = this.onMessage.filter((h) => h != handler);
        };
    }

    public subscribeToProposalNotifications(handler: (data: ProposalNotification) => void) {
        this.onProposalNotificationHandlers.push(handler)
        return () => {
            this.onProposalNotificationHandlers = this.onProposalNotificationHandlers.filter((h) => h != handler);
        }
    }

    public subscribeToNewProposalNotifications(handler: (data: NewProposalNotification) => void) {
        this.onNewProposalsNotificationHandlers.push(handler)
        return () => {
            this.onNewProposalsNotificationHandlers = this.onNewProposalsNotificationHandlers.filter((h) => h != handler);
        }
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