import { error, redirect } from '@sveltejs/kit';

// /** @type {import('./$types').RequestHandler} */
// export function GET({request}) {
//     const new_request = new Request(
//         request.url.replace('http://localhost:5173/api/add', 'http://localhost:8080/'),
//         request
//     );
//     return fetch(new_request);
//     // const min = Number(url.searchParams.get('min') ?? '0'); const max = Number(url.searchParams.get('max') ?? '1');
//     // const d = max - min;
//     // if (isNaN(d) || d < 0) { error(400, 'min and max must be numbers, and min must be less than max'); }
//     // const random = min + Math.random() * d;
//     // console.log("OK");
//     // return new Response(String(random));
// }