export const ssr = false;

export const load = async () => {
    const response = await fetch("/api/discussions");
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
  return { status: 200, body: 'Hello, world!' };
};