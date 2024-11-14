import { error } from '@sveltejs/kit';
import type { LayoutLoad } from './$types';
export const ssr = false;
export const load: LayoutLoad = async ({ fetch, params }) => {
    if (!params.id) {
        return {
            status: 404,
            project: undefined
        }
    }
    const request = await fetch(`/api/projects/${params.id}`);
    if (!request.ok) return {
        status: request.status,
        error: new Error('Failed to fetch data')
    }
    try {
        const response = await request.json();
        return {
            status: request.status,
            project: {
                id: response.id,
                user_id: response.user_id,
                title: response.title,
                content: response.content,
                budget: response.budget,
                currency_code: response.currency_code,
                deadline: new Date(response.deadline),
                created_at: new Date(response.created_at)
            }
        }
    } catch (error) {
        return {
            status: request.status,
            error: new Error('Failed to parse data')
        }
    }

    error(404, 'Not found');
};