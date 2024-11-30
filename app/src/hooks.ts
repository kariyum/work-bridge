import { redirect, type Handle } from '@sveltejs/kit';

export const handle: Handle = async ({ event, resolve }) => {
	const pathExclusionEqualityCheck = ["/"]
	const pathStartsWithCheck = ["/register", "/login"]
	const isPathPublic = pathExclusionEqualityCheck.includes(event.url.pathname) ||
		pathStartsWithCheck.some((path) => event.url.pathname.startsWith(path))
	const isCookieDefined = event.cookies.get("Authorization") !== undefined;
	if (!isPathPublic && !isCookieDefined) {
		return redirect(303, '/login');
	}
	const response = await resolve(event);
	if (response.status === 401) {
		return redirect(303, '/login');
	}
	return response;
};