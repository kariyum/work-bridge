import { redirect } from '@sveltejs/kit';

export async function load({ url, fetch }) {
    redirect(303, "/settings/profile");
}