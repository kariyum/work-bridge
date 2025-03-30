export async function load({ url }) {
    return {
        url: url.pathname.slice(1).split("/")[1]
    }
}