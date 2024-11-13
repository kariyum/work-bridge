<script lang="ts">
	import RichTextEditor from "./RichTextEditor.svelte";

	let { taskObject }: { taskObject: TaskObject | undefined } = $props();
    let content = $state('');
    let modal: HTMLDialogElement;
</script>

{#if taskObject}
	<div class="container">
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
