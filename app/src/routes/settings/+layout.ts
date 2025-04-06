import { fetchIntoResult } from '$lib/utils.js'

interface ProfileJSON {
    user_id: string,
    first_name: string,
    last_name: string,
    role: string,
    bio?: string,
    skills?: string[],
    linkedin_link?: string,
    github_link?: string,
    portfolio_link?: string,
    created_at?: string,
}

export interface ProfileGET {
    user_id: string,
    first_name: string,
    last_name: string,
    role: string,
    bio?: string,
    skills?: string[],
    linkedin_link?: string,
    github_link?: string,
    portfolio_link?: string,
    created_at?: Date,
}

function processProfileJSON(data: ProfileJSON): ProfileGET {
    return {
        ...data,
        created_at: data.created_at ? new Date(data.created_at) : undefined
    };
}

export async function load({ fetch }) {
    const userProfile = await fetchIntoResult<ProfileJSON>(() => fetch("/api/profile"));
    const profileData = userProfile.map(processProfileJSON);
    return {
        profileData: profileData,
    }
}