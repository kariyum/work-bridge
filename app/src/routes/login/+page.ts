
export async function load({ url, fetch }) {
    return {
        redirectionUrl: decodeURIComponent(url.searchParams.get("redirect") ?? "/")
    };
}