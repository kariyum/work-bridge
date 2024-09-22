// src/hooks.client.js
import { goto } from '$app/navigation';

/** @type {import('@sveltejs/kit').Handle} */
export async function handle({ event, resolve }) {
  // Define your rerouting logic here
  console.log(`Running server hooks for ${event.url.pathname}`);
  // rewrite the request to the API localhost:8080
  if (event.url.pathname.startsWith('/api')) {
    console.log("Rewriting request to localhost:8080", event.request.url);
    const new_request = new Request(
      event.request.url.replace('http://localhost:5173/api', 'http://localhost:8080'),
      event.request
    );
    // console.log("Server hooks new request", new_request);
    let response = await fetch(new_request);
    return response;
  }

  // Continue with the normal resolve process
  return await resolve(event);
}
