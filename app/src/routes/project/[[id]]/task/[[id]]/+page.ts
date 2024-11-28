export async function load({ url }) {
    const index = url.searchParams.get('s')
    console.log("index: ", index);
    const result = { selectedIndex: index ? parseInt(index) : undefined };
    console.log("result: ", result);
    return result;
}