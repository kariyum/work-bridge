import { processProjectJson } from '$lib/types/project';
import type { LayoutLoad } from './$types';

export const load: LayoutLoad = async ({ fetch, params }) => {
    if (!params.id) {
        return {
            status: 404,
            project: undefined,
        }
    }
    const request = await fetch(`/api/projects/${params.id}`);
    if (!request.ok) return {
        status: request.status,
        error: new Error('Failed to fetch data'),
    }
    try {
        const response = await request.json();
        const project = processProjectJson(response);
        return {
            status: request.status,
            project: project,
        }
    } catch (error) {
        return {
            status: request.status,
            error: new Error('Failed to parse data'),
        }
    }
};