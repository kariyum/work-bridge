<!-- Quick temp button for testing purposes only... we can remove it in prod -->
<script lang="ts">
	import { Moon, Sun } from 'lucide-svelte';
	import { onMount } from 'svelte';
	import { themeService } from '$lib/theme/theme-service';
	import { BROWSER } from 'esm-env';
	import Button from './Button.svelte';

	onMount(() => {
		themeService.setupSystemPreferenceListener();
	});
</script>

<Button on:click={() => themeService.toggle()} style="width: 32px; padding: 8px">
	{#if BROWSER}
		{#if themeService.getStore().current === 'light'}
			<Moon size={15} />
		{:else}
			<Sun size={15} />
		{/if}
	{:else}
		<div></div>
	{/if}
</Button>