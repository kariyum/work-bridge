import { redirect, type Handle } from '@sveltejs/kit';

export const handle: Handle = async ({ event, resolve }) => {
  console.log("Handling", event.url);
  try {
    return resolve(event);
  } catch (error) {
    console.log("Failed to fetch", event);
    throw error;
  }
  // return resolve(event);
  if (event.request.url.includes('/api/old-endpoint')) {
    // Redirect to a new endpoint
    const newUrl = event.request.url.replace(`\/http.*/api\/http://server:80`, '');
    console.log(newUrl)
    // Forward the request to the new URL
    const response = await fetch(newUrl, event.request);
    return response;
  }

  // If no redirection is needed, proceed with the original request
  return fetch(event.request);
  const pathInclusionEqualityCheck = ["/"]
  const pathStartsWithCheck = ["/register", "/login", "/api"]
  const isPathPublic = pathInclusionEqualityCheck.includes(event.url.pathname) ||
    pathStartsWithCheck.some((path) => event.url.pathname.startsWith(path))
  const isCookieDefined = event.cookies.get("Authorization") !== undefined;
  const redirectionUrl = `/login?redirect=${encodeURIComponent(event.url.pathname)}`;
  if (!isPathPublic && !isCookieDefined) {
    console.log("Redirecting... hooks");
    redirect(302, redirectionUrl);
  }
  const response = await resolve(event);
  console.log("In hooks", event.url.pathname, isPathPublic, isCookieDefined);
  if (response.status === 401 && !isPathPublic) {
    console.log("Redirecting2... hooks");
    redirect(302, redirectionUrl);
  }
  return response;
};
