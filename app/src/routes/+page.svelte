<script lang="ts">
	import Project from '$lib/components/Project.svelte';
	import { authStore } from '$lib/storage.js';
	let { data } = $props();
	if (data.status === 401) {
		authStore.set(false);
	}
	console.log(data.projects);
</script>

{#if $authStore}
	<h1 style="width:60%; margin:auto;">My Projects</h1>
	{#each data.projects as project}
		<Project {project} />
	{/each}
{:else}
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
				<button style="width: 7em; background-color: var(--dark-purple); color:white;"
					>Register</button
				>
			</a>
		</div>
		<!-- <p>
				Please <a href="/login">login</a> or <a href="/register">create</a> an account to continue
			</p> -->
	</div>
{/if}

<style>
	.buttons {
		padding: 4em;
	}
	.buttons > a {
		margin-left: 0.5em;
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
