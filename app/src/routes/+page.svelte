<script>
	/** @type {import('./$types').PageData} */
	export let data;
</script>

{data.error}
{#if data.error}
	<h1>Welcome to Work Bridge</h1>
	<h2>Where you can transform abstract ideas into working and concrete projects</h2>
	<div class="container">
		<p>
			Please <a href="/login">login</a> or <a href="/register">create</a> an account to continue
		</p>
	</div>
{:else}
	<h1>Some posts lmao</h1>
{/if}

<style>
	h1,
	h2 {
		text-align: center;
	}
	.container {
		display: flex;
		justify-content: space-between;
		/* align-items: center; */
		flex-wrap: wrap;
		padding: 10%;
		position: absolute;
		top: 50%;
		left: 50%;
		transform: translate(-50%, -50%);
	}
</style>
