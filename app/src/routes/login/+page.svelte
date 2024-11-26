<script lang="ts">
	import { goto, invalidate, invalidateAll } from '$app/navigation';
	import AlreadyLoggedIn from '$lib/components/AlreadyLoggedIn.svelte';
	import { userStore } from '$lib/storage';
	import { cyrb53, validateEmail } from '$lib/utils';
	import { onMount } from 'svelte';

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
	let final_error_message = '';

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
					await invalidateAll();
					await goto('/');
				} else {
					error_message = 'Wrong combination';
				}
			},
			(reason) => console.log("Connection issues, retry later")
		);

		final_error_message = error_message;
	}
</script>

{#if $userStore}
	<AlreadyLoggedIn />
{:else}
	<div class="container">
		<h1>Welcome</h1>
		<form class="fields-container" method="post" on:submit|preventDefault bind:this={form_element}>
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
					<button type="submit" on:click|preventDefault={handleSubmit}>Login</button>
					<a href="/register">
						<button style="width: 100%;"> Register </button>
					</a>
				</div>
			</div>
		</form>
	</div>
{/if}

<style>
	.action-buttons {
		display: flex;
		flex-flow: row-reverse;
		justify-content: space-between;
		align-items: center;
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
		display: flex;
		justify-content: space-between;
		flex-direction: row;
		flex-wrap: wrap;
		/* align-items: center; */
		width: 50%;
		padding: 0% 10% 0% 10%;
		position: absolute;
		top: 50%;
		left: 50%;
		transform: translate(-50%, -50%);
	}
	.fields-container {
		display: flex;
		flex-direction: column;
		align-items: baseline;
		width: min-content;
		gap: 10px;
		width: 50%;
	}
</style>
