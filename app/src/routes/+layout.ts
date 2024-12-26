import { WebSocketService } from '$lib/realtime';
import { redirect } from '@sveltejs/kit';

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
            user: user,
            status: response.status,
        }
    } else {
        return {
            status: response.status,
        }
    }
}