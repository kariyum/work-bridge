import { WebSocketService } from '$lib/realtime';
import { redirect } from '@sveltejs/kit';

export const ssr = false;
export async function load({ url, fetch }) {
    const response = await fetch("/api/auth/whoami");
    if (response.ok) {
        const jsonResponse = await response.json();
        const user = {
            email: jsonResponse.sub,
            role: jsonResponse.role
        } as User;
        if (url.searchParams.has("redirect")) {
            return redirect(303, decodeURIComponent(url.searchParams.get("redirect") ?? "/"));
        }
        return {
            socket: new WebSocketService("/api/chat"),
            user: user,
            status: response.status,
        }
    } else {
        return {
            socket: new WebSocketService("/api/chat"),
            status: response.status,
        }
    }
}