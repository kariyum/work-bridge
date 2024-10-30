import { userStore } from '$lib/storage.js';


export async function load({ fetch }) {
    const response = await fetch("/api/whoami");
    try {
        if (response.ok) {
            const jsonResponse = await response.json();
            const user = {
                email: jsonResponse.sub,
                role: jsonResponse.role
            } as User;
            userStore.set(user);
            return {
                user: user,
                status: response.status,
            }
        } else {
            userStore.set(undefined);
            return {
                status: response.status,
            }
        }
    } catch (error) {
        userStore.set(undefined);
        return {
            error: `An error occurred while fetching user: ${error}`,
            status: response.status,
        }
    }
}