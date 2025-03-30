
export async function load({ url }) {
    return {
        redirectionUrl: decodeURIComponent(url.searchParams.get("redirect") ?? "/")
    };
}