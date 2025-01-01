<script lang="ts">
	import { goto, invalidate, invalidateAll } from '$app/navigation';
	import AlreadyLoggedIn from '$lib/components/AlreadyLoggedIn.svelte';
	import { cyrb53, validateEmail } from '$lib/utils.js';
	import { onMount } from 'svelte';
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

	let form_element: HTMLFormElement;
	let email_element: HTMLInputElement;
	let password_element: HTMLInputElement;
	let error_message = '';
	let final_error_message = $state('');

	async function handleSubmit() {
		error_message = '';
		form_element.reportValidity();
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
					await goto('/', { invalidateAll: true });
				} else {
					error_message = 'Wrong combination';
				}
			},
			(reason) => console.log('Connection issues, retry later')
		);

		final_error_message = error_message;
	}
</script>

{#if data.user}
	<AlreadyLoggedIn />
{:else}
	<div class="container">
		<div class="sub-container">
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
	.forgot-password {
		background-color: transparent;
		border: none;
		color: #666;
		cursor: pointer;
	}

	.buttons {
		width: 100%;
		display: flex;
		flex-direction: column;
	}

	.container {
		position: absolute;
		top: 5%;
		/* transform: translate(-50%, 0); */
		width: 100%;
	}

	.fields-container {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
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
