<script lang="ts">
	let isLoading = $state(false);
	let done = $state(false);
	let { onclick }: { onclick: (event: MouseEvent) => Promise<any> } = $props();

	async function clickHandler(event: MouseEvent) {
		isLoading = true;
		setTimeout(async () => {
			await onclick(event);
			isLoading = false;
			done = true;
			setTimeout(() => (done = false), 3000);
		}, 500);
	}
</script>

<button onclick={clickHandler}>
	{#if isLoading}
		<div>Loading...</div>
	{:else if done}
		<div>Done!</div>
	{:else}
		Click me!
	{/if}
</button>
