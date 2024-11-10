<script lang="ts">
	import RichTextEditor from "$lib/components/RichTextEditor.svelte";

	let modal: HTMLDialogElement;
	let content = $state('');
</script>

<div class="popover">
	<div class="create-task">
		<h2>Add task</h2>
		<div class="input-container">
			<input type="text" placeholder="Title" />
			<RichTextEditor bind:content></RichTextEditor>
			<input type="text" placeholder="Assignee" />
			<input type="text" placeholder="Skills" />
			<input type="text" placeholder="Deadline" />
			<input type="text" placeholder="Estimated Efforts" />
		</div>
		<div class="act-task">
			<button onclick={() => modal.close()}>Cancel</button>
			<button>Add</button>
		</div>
	</div>
</div>

<style>
	.act-task {
		margin-left: auto;
	}
	.input-container {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}
	.create-task {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}
	.popover {
		margin: auto;
		padding: 1rem;
		border: 1px solid #eee;
		border-radius: 5px;
		width: 50%;
		max-width: 1000px;
	}
</style>
