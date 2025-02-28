<script lang="ts">
	import type { TaskClass } from '$lib/states.svelte';
	import type { TaskGET } from '$lib/types/task';
	let { onclick, taskObject }: { onclick: (event: MouseEvent) => void, taskObject: TaskClass } = $props();
</script>

{#if taskObject}
	<!-- svelte-ignore a11y_click_events_have_key_events -->
	<!-- svelte-ignore a11y_no_static_element_interactions -->
	<div class="container" onclick={onclick}>
		<div class="task-id">#{taskObject.id}</div>
		<div class="sub-container">
			<div>
				<div class="task-title">{taskObject.title}</div>
			</div>
			<div class="right-container">
				<div>{taskObject.assignee_id}</div>
				<div class="status">{taskObject.status}</div>
			</div>
		</div>
	</div>
{/if}

<style>
	.task-title {
		font-size: larger;
		font-weight: 500;
	}
	.right-container {
		display: flex;
		gap: 0.5rem;
		align-items: center;
	}
	.status {
		background-color: var(--btn-bg);
		border: 2px solid var(--border);
		border-radius: 5px;
		padding: 0.5rem;
		min-width: 10ch;
		text-align: center;
	}
	div {
		border-radius: 5px;
		background-color: inherit;
	}
	.container {
		display: flex;
		flex-direction: column;
		justify-content: space-between;
		border: 2px solid var(--border);
		border-radius: 5px;
		padding: 0.5rem 0.5rem 0.5rem 0.5rem;
	}
	.sub-container {
		display: flex;
		justify-content: space-between;
		align-items: center;
	}
	.container:hover {
		cursor: pointer;
		border: 2px solid var(--blue);
		background-color: var(--blue);
	}
</style>
