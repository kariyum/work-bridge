import { redirect, type Handle } from '@sveltejs/kit';

export const handle: Handle = async ({ event, resolve }) => {
	const pathInclusionEqualityCheck = ["/"]
	const pathStartsWithCheck = ["/register", "/login", "/api"]
	const isPathPublic = pathInclusionEqualityCheck.includes(event.url.pathname) ||
	pathStartsWithCheck.some((path) => event.url.pathname.startsWith(path))
	const isCookieDefined = event.cookies.get("Authorization") !== undefined;
	const redirectionUrl = `/login?redirect=${encodeURIComponent(event.url.pathname)}`;
	if (!isPathPublic && !isCookieDefined) {
		redirect(302, redirectionUrl);
	}
	const response = await resolve(event);
	console.log("In hooks", event.url.pathname, isPathPublic, isCookieDefined);
	if (response.status === 401 && !isPathPublic) {
		redirect(302, redirectionUrl);
	}
	return response;
};