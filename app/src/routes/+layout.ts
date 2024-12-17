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
            console.log("Redirecting from layout.ts");
            // todo, update redirect logic once we remove updating writable in this file...
            return redirect(303, decodeURIComponent(url.searchParams.get("redirect") ?? "/"));
            // return {
            //     user: user,
            //     status: response.status,
            //     redirect: decodeURIComponent(url.searchParams.get("redirect") ?? "/")
            // }
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