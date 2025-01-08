<script lang="ts">
	import type { HTMLButtonAttributes } from 'svelte/elements';
	type $$Props = HTMLButtonAttributes;

	let className: $$Props['class'] = '';
	export { className as class };
</script>

<button on:click {...$$restProps} class={className}>
	<slot />
</button>

<style>
	button {
		display: block;
		outline: none;
		border: none;
		background: none;
		color: currentColor;

		padding: 9px 19px;
		height: 40px;
		cursor: pointer;
		border-radius: 8px;
		background-color: transparent;
		white-space: nowrap;
		display: flex;
		align-items: center;

		border: 1px solid var(--separator-color);
	}

	button:not(:disabled):hover {
		background-color: var(--background-color-hover);
	}
</style>