export async function load({ url, fetch }) {
    return {
        url: url.pathname.slice(1).split("/")[1]
    }
}