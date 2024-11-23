/** 
 * @param {String} email
 * @param {String} password
 */
export function login(email, password) {
    return fetch('/api/login', {
        method: 'POST',
        headers: {
            'Content-Type': 'application/x-www-form-urlencoded',
        },
        body: new URLSearchParams({
            'email': email,
            'password': password,
        })
    });
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