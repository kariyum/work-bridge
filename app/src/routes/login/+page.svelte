<script lang="ts">
	import { goto } from '$app/navigation';
	import AlreadyLoggedIn from '$lib/pages/AlreadyLoggedIn.svelte';
	import { cyrb53, validateEmail } from '$lib/utils.js';
	import { MoveLeft } from 'lucide-svelte';
	let { data } = $props();

	function login(email: string, password: string): Promise<Response> {
		return fetch('/api/auth/login', {
			method: 'POST',
			headers: {
				'Content-Type': 'application/x-www-form-urlencoded'
			},
			body: new URLSearchParams({
				email: email,
				password: password
			})
		});
	}

	let form_element: HTMLFormElement | undefined = $state();
	let email_element: HTMLInputElement | undefined = $state();
	let password_element: HTMLInputElement | undefined = $state();
	let error_message = '';
	let final_error_message = $state('');

	async function handleSubmit() {
		if (email_element && password_element) {
			error_message = '';
			form_element?.reportValidity();
			const email = email_element.value;
			const password = password_element.value;
			if (!email || !validateEmail(email)) {
				email_element.style.border = '2px solid red';
				return;
			} else {
				email_element.style.border = '';
			}
			if (!password) {
				password_element.style.border = '2px solid red';
				return;
			} else {
				password_element.style.border = '';
			}
			await login(email, cyrb53(password).toString()).then(
				async (response) => {
					if (response.ok) {
						await goto(data.redirectionUrl, { invalidateAll: true });
					} else {
						error_message = 'Wrong combination';
					}
				},
				(reason) => console.log('Connection issues, retry later', reason)
			);

			final_error_message = error_message;
		}
	}
</script>

{#if data.user}
	<AlreadyLoggedIn />
{:else}
	<div class="container">
		<div class="sub-container">
			<a href="/">
				<MoveLeft size="3rem" />
			</a>
			<h1>Welcome</h1>
			<form
				class="fields-container"
				method="post"
				onsubmit={(event) => event.preventDefault()}
				bind:this={form_element}
			>
				<input name="email" type="email" placeholder="Email" required bind:this={email_element} />
				<input
					name="password"
					type="password"
					placeholder="Password"
					required
					bind:this={password_element}
				/>
				<p>
					{final_error_message}
				</p>
				<div class="buttons">
					<div class="action-buttons">
						<button type="submit" onclick={handleSubmit}>Login</button>
					</div>
				</div>
			</form>
			<a href="/register">Don't have an account? Register!</a>
		</div>
	</div>
{/if}

<style>
	.action-buttons {
		display: flex;
		flex-flow: row-reverse;
		align-items: center;
		gap: 0.5rem;
		width: 100%;
	}

	.buttons {
		width: 100%;
		display: flex;
		flex-direction: column;
	}

	.container {
		margin-top: 3rem;
		width: 100%;
	}

	.fields-container {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}

	a {
		color: var(--ucla-blue);
	}

	.sub-container {
		display: flex;
		flex-direction: column;
		gap: 1rem;
		padding: 0 1rem;
		max-width: 45rem;
		margin: auto;
	}
</style>
