import type { BaseNotification, FetchErrors, User } from '$lib/types.ts';
import { fetchIntoResult, getRedirectionUrl, Result, shouldRedirect } from '$lib/utils.js';
import { redirect } from '@sveltejs/kit';
import { untrack } from 'svelte';

type BaseNotificationJSON = {
    created_at: string,
    notification_type: string,
}

export async function load({ url, fetch }) {
    const whoamiResponse: Result<any, FetchErrors> = await fetchIntoResult<any>(() => fetch("/api/auth/whoami"));
    const notificationsResponse = await fetchIntoResult<BaseNotificationJSON[]>(() => fetch("/api/notifications"));
    const processNotifications = (a: BaseNotificationJSON) => {
        return {
            ...a,
            notification_type: a.notification_type,
            created_at: new Date(a.created_at)
        } as BaseNotification;
    }
    if (whoamiResponse.isOk() && notificationsResponse.isOk()) {
        const whoami = whoamiResponse.getOrThrow();
        const user = {
            email: whoami.sub,
            role: whoami.role
        } as User;
        const notifications = notificationsResponse.getOrThrow().map(processNotifications);
        return {
            user: user,
            notifications: notifications.toSorted((a, b) => b.created_at.getTime() - a.created_at.getTime())
        }
    }

    
    // if (shouldRedirect(response, untrack(() => url.pathname))) {
    //     return redirect(302, getRedirectionUrl(untrack(() => url.pathname)));
    // }
    return {
        error: whoamiResponse.error
    }
}