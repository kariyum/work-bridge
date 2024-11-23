<script lang="ts">
	import type { TaskClass } from "$lib/states.svelte";
	import RichTextEditor from "./RichTextEditor.svelte";

	// let { taskObject, onclick }: { taskObject: TaskClass | undefined } = $props();
	let props = $props();
	let taskObject = props.taskObject;
    let content = $state('');
    let modal: HTMLDialogElement;
</script>

{#if taskObject}
	<!-- svelte-ignore a11y_click_events_have_key_events -->
	<!-- svelte-ignore a11y_no_static_element_interactions -->
	<div class="container" onclick={props.onclick}>
		<div>{taskObject.title}</div>
        <div class="right-container">
            <div>{taskObject.assignee_id}</div>
            <div class="status">{taskObject.status}</div>
        </div>
	</div>
{/if}
  
<style>
    .right-container {
        display: flex;
        gap: 0.5rem;
        align-items: center;
    }
	.status {
		background-color: #eee;
        border: 1px solid #ddd;
		border-radius: 5px;
		padding: 0.5rem;    
        min-width: 10ch;
        text-align: center;
	}
	div {
		border-radius: 5px;
	}
	.container {
		display: flex;
		justify-content: space-between;
        align-items: center;
		border: 1px solid #eee;
		border-radius: 5px;
		padding: 0.5rem 0.5rem 0.5rem 0.5rem;
	}
</style>
