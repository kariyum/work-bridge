import type { User } from '$lib/types.ts';
import { fetchWrapper } from '$lib/utils.js';

export async function load({ url, fetch }) {
    const response = await fetchWrapper<any>(fetch, "/api/auth/whoami");
    if (response.isOk()) {
        const jsonResponse = response.getOrThrow();
        const user = {
            email: jsonResponse.sub,
            role: jsonResponse.role
        } as User;
        return {
            user: user,
        }
    }
}