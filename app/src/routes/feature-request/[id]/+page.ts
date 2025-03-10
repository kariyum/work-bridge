import type { FeatureRequestGET } from "../+layout";

export async function load({ params, parent }) {
    const parentData = await parent();

    return {
        featureRequest: parentData.featureRequests.find((fr: FeatureRequestGET) => fr.id.toString() === params.id)
    }
}