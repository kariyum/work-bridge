import { error } from '@sveltejs/kit';
import { processProjectJson, type ProjectJSON } from '$lib/types/project';

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
            project: processProjectJson(response)
        }
    } catch (error) {
        return {
            status: request.status,
            error: new Error('Failed to parse data')
        }
    }

    error(404, 'Not found');
};