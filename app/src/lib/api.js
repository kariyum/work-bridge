/** 
 * @param {String} email
 * @param {String} password
 */
export async function login(email, password) {
    const response = await fetch('/api/login', {
        method: 'POST',
        headers: {
            'Content-Type': 'application/x-www-form-urlencoded',
        },
        body: new URLSearchParams({
            'email': email,
            'password': password,
        })
    });
    return response;
}

export async function register() {
    const response = await fetch('/api');
    if (response.ok) {
        const result = await response.text();
        console.log('Form submitted successfully:', result);
    } else {
        console.error('Form submission failed:', response.statusText);
    }
}