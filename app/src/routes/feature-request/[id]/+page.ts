export async function load({ params, parent }) {
    const parentData = await parent();

    return {
        featureRequest: parentData.featureRequests.find((fr) => fr.id.toString() === params.id)
    }
}