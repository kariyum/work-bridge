/** @type {import('./$types').PageServerLoad} */
export async function load({ cookies }) {
	// const user = await db.getUserFromSession(cookies.get('sessionid'));
	console.log("Running load function on page.server.js");
    const user = {"name": "Karim"};
	return { user };
}

/** @type {import('./$types').Actions} */
export const actions = {
	login: async ({ cookies, request }) => {
		const data = await request.formData();
		const email = data.get('email');
		const password = data.get('password');
		const payload = Object.fromEntries(data.entries());
        console.log(email, password);
        try {
			const response = await fetch('http://localhost:8080/login', {
				method: 'POST',
				headers: {
					'Content-Type': 'application/json',
				},
				body: JSON.stringify(payload)
			});
			const cookie = response.headers.get('set-cookie');
			console.log("Response headers are", );
			cookies.set('Authorization', "setfromsvelteserver", { path: '/' });

			if (response.ok) {
				const result = await response.json();
				console.log('Form submitted successfully:', result);
			} else {
				console.error('Form submission failed:', response.statusText);
			}
		} catch (error) {
			console.error('Error submitting form:', error);
		}

		return { success: true, redirect: '/profile' };
	},
	register: async (event) => {
		// TODO register the user
	},
	send: async (cookies) => {
		console.log("Running send function on page.server.js");
		console.log("Cookies are", cookies);
	}
};