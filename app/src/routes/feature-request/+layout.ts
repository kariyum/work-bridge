interface FeatureRequestJSON {
    id: number;
    title: string;
    created_by: string;
    description: string;
    ups: string[];
    downs: string[];
    created_at: string;
}

interface FeatureRequestGET {
    id: number;
    title: string;
    created_by: string;
    description: string;
    ups: string[];
    downs: string[];
    created_at: Date;
}

function processResponse(response: FeatureRequestJSON): FeatureRequestGET {
    const featureRequest: FeatureRequestGET = {
        ...response,
        created_at: new Date(response.created_at),
    };
    return featureRequest;
}


export async function load({ fetch }) {
    const response = fetch("/api/feature-request", { method: "GET" });
    const parseJson = (jsonData: Array<FeatureRequestJSON>) => jsonData.map((json) => processResponse(json));
    const result = response
        .then((response) => {
            if (response.ok) {
                return response.json()
            } else {
                return Promise.resolve([]);
            }
        })
        .then((jsonData: Array<FeatureRequestJSON>) => Promise.resolve(parseJson(jsonData)))
        .then((data) => {
            return Promise.resolve({
                featureRequests: data as FeatureRequestGET[],
                error: undefined
            });
        })
        .catch((exception) => {
            console.error(exception);
            return Promise.resolve({
                featureRequests: [] as FeatureRequestGET[],
                error: exception
            });
        });
    return result;
}