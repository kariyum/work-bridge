<script>
	/** @type {import('./$types').PageData} */
	export let data;
	import { onMount } from 'svelte';
	import storage from '$lib/storage';
	import Navbar from '$lib/components/Navbar.svelte';
	import CreatePost from '$lib/components/CreatePost.svelte';
	/**
	 * @type {import('svelte/store').Writable<boolean>}
	 */
	let store;
	onMount(() => {
		store = storage('auth', false);
		if (data.error) {
			store.set(false);
		}
	});

	// setInterval(() => {
	// 	if ($store) {
	// 		console.log('store', $store);
	// 	}
	// }, 1000);
</script>

{#if data.error}
		<div class="container">
			<div style="margin:auto; text-align:center; font-size:48px;">
				<h1>Recruit freelancers</h1>
				<h1>& form teams</h1>
			</div>
			<p class="subtitle">Connect with thousands of Tunisian freelancers.</p>
			<div class="buttons">
				<a href="/login">
					<button style="width: 7em;">Login</button>
				</a>
				<a href="/register">
					<button style="width: 7em; background-color: var(--dark-purple); color:white;">Register</button>
				</a>
			</div>
			<!-- <p>
				Please <a href="/login">login</a> or <a href="/register">create</a> an account to continue
			</p> -->
		</div>
{:else}
	<Navbar></Navbar>
	<CreatePost></CreatePost>
{/if}

<style>
	.buttons {
		padding: 4em;
	}
	.buttons > a {
		margin-left: .5em;
	}
	.subtitle {
		color: gray;
		font-size: 36px;
	}
	.container {
		display: flex;
		justify-content: center;
		align-items: center;
		flex-flow: column;
		padding: 10%;
		position: absolute;
		top: 50%;
		left: 50%;
		width: 80%;
		transform: translate(-50%, -60%);
	}
</style>
