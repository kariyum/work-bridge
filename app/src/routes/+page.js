/** @type {import('./$types').PageLoad} */
export async function load({ fetch, params }) {
	// we try to fetch posts for example
    // if that failed with a 403 unauthorized error
    // we display
    const response = await fetch("/api/posts");
    console.log("response", response);
    if (response.status == 200) {
        return {
            props: {
                posts: await response.json()
            }
        }
    } else {
        return {
            error: "You are not authorized to view this page"
        }
    }

}