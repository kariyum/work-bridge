<script lang="ts">
	import { goto } from '$app/navigation';
	import RichTextEditor from '$lib/components/RichTextEditor.svelte';

	let modal: HTMLDialogElement;
	let content = $state('');
</script>

<div class="blur">
	<div class="popover">
		<div class="create-task">
			<h2>Add task</h2>
			<div class="input-container">
				<input type="text" placeholder="Title" />
				<RichTextEditor bind:x={content}></RichTextEditor>
				<input type="text" placeholder="Assignee" />
				<input type="text" placeholder="Skills" />
				<input type="text" placeholder="Deadline" />
				<input type="text" placeholder="Estimated Efforts" />
			</div>
			<div class="act-task">
				<button onclick={() => history.back()}>Cancel</button>
				<button>Add</button>
			</div>
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
	.popover2 {
		margin: auto;
		padding: 1rem;
		border: 1px solid #eee;
		border-radius: 5px;
		width: 50%;
		max-width: 1000px;
	}
	.popover {
		position: absolute;
		top: 50%;
		left: 50%;
		width: 50%;
		max-width: 1000px;
		transform: translate(-50%, -50%);
		padding: 1rem;
		border: 1px solid #eee;
		border-radius: 5px;
		box-shadow: 0 0 10px rgba(0, 0, 0, 0.1);
		/* blur */
	}

	.blur {
		position: absolute;
		top: 0;
		left: 0;
		width: 100%;
		height: 100%;
		backdrop-filter: blur(2px);
		background-color: rgba(0, 0, 0, 0.1);
	}
</style>
