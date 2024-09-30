<script>
	import { goto } from '$app/navigation';
	import { login, register } from '$lib/api.js';
	import { onMount } from 'svelte';
	import storage from "$lib/storage";
	import { writable } from 'svelte/store';
	import { cyrb53, validateEmail } from '$lib/utils';
	import { error } from '@sveltejs/kit';


	/**
	 * @type {import('svelte/store').Writable<boolean>}
	 */
	let store;
	onMount(() => {
		store = storage("auth", false);
		if ($store) {
			goto('/');
		}
	})
	
	/**
	 * @type {HTMLInputElement}
	 */
	let email_element;
	/**
	 * @type {HTMLFormElement}
	 */
	let form_element;
	/**
	 * @type {HTMLInputElement}
	 */
	let password_element;
	let error_message = '';
	let final_error_message = '';
	async function handleSubmit() {
		error_message = '';
		form_element.reportValidity();
		const email = email_element.value;
		const password = password_element.value;
		if (!email || !validateEmail(email)) {
			email_element.style.border = '2px solid red';
			return
		} else {
			email_element.style.border = '';
		}
		if (!password) {
			password_element.style.border = '2px solid red';
			return
		} else {
			password_element.style.border = '';
		}
		const response = await login(email, cyrb53(password).toString());
		if (response.ok) {
			store.set(true);
			goto('/');
		} else {
			error_message = "Wrong combination"
		}
		final_error_message = error_message;
	}
</script>

<div class="container">
	<h1>Welcome</h1>
	<form class="fields-container" method="post" on:submit|preventDefault bind:this={form_element}>

		<input name="email" type="email" placeholder="Email" required bind:this={email_element} />
		<input name="password" type="password" placeholder="Password" required bind:this={password_element}/>
		<p>
			{final_error_message}
		</p>
		<div class="buttons">
			<div class="action-buttons">
				<button type="submit" on:click|preventDefault={handleSubmit} >Login</button>
				<!-- <a href="/register">
					<button style="width: 100%;">
						Register
					</button>
				</a> -->
			</div>
		</div>
	</form>
</div>

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
		/* align-items: center; */
		flex-wrap: wrap;
		width: 30%;
		padding: 10%;
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
