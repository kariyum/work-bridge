<script lang="ts">
	import { page } from '$app/state';
	import { ChevronRight } from 'lucide-svelte';

	let { children } = $props();
</script>

{#snippet breadcrumbs()}
	{#if !page.state.projectEditMode}
		<div class="links">
			<div class="link">
				<a href="/">Projects</a>
				<ChevronRight size="16" />
				{#if page.params.task_id}
					<a href="/project/{page.params.id}">{page.params.id}</a>
				{:else}
					<div>{page.params.id}</div>
				{/if}
			</div>
			{#if page.params.task_id}
				<div class="link">
					<ChevronRight size="16" />
					<div>Tasks</div>
					<ChevronRight size="16" />
					<div>{page.params.task_id}</div>
				</div>
			{/if}
		</div>
	{/if}
{/snippet}

<div class="page">
	{@render breadcrumbs()}
	{@render children()}
</div>

<style>
    a {
        color: unset;
        text-decoration: unset;
    }
	.page {
		padding: 1rem;
		padding-top: 0;
	}

	.links {
        margin: auto;
        max-width: var(--page-width);
		display: flex;
		view-transition-name: links;
	}

	.link {
		display: flex;
		gap: 0.5rem;
		margin: 1rem 1rem 0rem 0rem;
		color: var(--sub-title);
		> :last-child {
			color: var(--font-color);
		}
	}
</style>
