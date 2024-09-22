<script>
	import { enhance } from '$app/forms';
	import { goto } from '$app/navigation';
	import { login, register } from '$lib/api.js';

	let email = '';
	let password = '';

	async function handleSubmit() {
		if (!email || !password) {
			alert('Please fill in all fields');
			return;
		}
		const response = await login(email, password);
		if (response.ok) {
			goto('/');
		}
	}
</script>

<div class="container">
	<h1>Welcome</h1>
	<form class="fields-container" method="post" on:submit|preventDefault>
		<input name="email" type="email" placeholder="Email" required bind:value={email} />
		<input name="password" type="password" placeholder="Password" required bind:value={password}/>
		<div class="buttons">
			<div class="action-buttons">
				<button class="forgot-password" on:click={register}>Forgot password?</button>
				<button type="submit" on:click|preventDefault={handleSubmit} >Login</button>
			</div>
			<a href="/register">
				<button>
					Register
				</button>
			</a>
			
		</div>
	</form>
</div>

<style>
	.action-buttons {
		display: flex;
		flex-direction: row;
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
