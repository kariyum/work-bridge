import type { FetchErrors, User } from '$lib/types.ts';
import { fetchIntoResult, getRedirectionUrl, Result, shouldRedirect } from '$lib/utils.js';
import { redirect } from '@sveltejs/kit';
import { untrack } from 'svelte';

export async function load({ url, fetch }) {
    const response: Result<any, FetchErrors> = await fetchIntoResult<any>(() => fetch("/api/auth/whoami"));
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
    
    if (shouldRedirect(response, untrack(() => url.pathname))) {
        return redirect(302, getRedirectionUrl(untrack(() => url.pathname)));
    }
    return {
        error: response.error
    }
}