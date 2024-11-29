export async function load({ url }) {
    const index = url.searchParams.get('s')
    const result = { selectedIndex: index ? parseInt(index) : undefined };
    return result;
}