<script lang="ts">
	import { ArrowUp, ArrowDown, SendHorizontal } from 'lucide-svelte';

	let { data } = $props();
</script>

<div class="container">
	<div class="sub-container">
		<h1>#{data.featureRequest?.id} - {data.featureRequest?.title}</h1>
		<div class="status" data-type="todo">TODO</div>
		<p class="content">
			{data.featureRequest?.description}
		</p>
		<div class="actions">
			<button class="vote left-chip">
				<div class="emoji no-line-height"><ArrowUp size="18" /></div>
				<div>10</div>
			</button>
			<div class="vertical-line"></div>
			<button class="vote right-chip">
				<div class="emoji no-line-height"><ArrowDown size="18" /></div>
				<div>10</div>
			</button>
		</div>
		<div class="comments">
			<h2>Comments</h2>
			<div class="all-comments">No comments yet.</div>
			<div class="comment-input">
				<input type="text" name="comment" placeholder="write a comment" id="comment" />
				<SendHorizontal />
			</div>
		</div>
	</div>
</div>

<style>
	.emoji {
		margin: 0;
		padding: 0.3rem;
		background-color: transparent;
		border: none;
	}

	.sub-container {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
		max-width: 1100px;
		margin: auto;
	}

	.content {
		min-height: 5rem;
		margin-top: 0.5rem;
		border-radius: 5px;
	}

	.actions {
		display: flex;
		width: fit-content;
	}

	.left-chip {
		padding: 0.3rem 1rem 0.3rem 0.7rem;
		border-radius: 2rem 0 0 2rem;
	}

	.right-chip {
		padding: 0.3rem 1rem 0.3rem 0.7rem;
		border-radius: 0 2rem 2rem 0;
	}

	.vote {
		background-color: var(--btn-bg);
		display: flex;
		align-items: center;
		gap: 0.5rem;
		margin: 0;
		border: none;
	}

	.vote:hover {
		background-color: var(--blue);
	}

	.vertical-line {
		background-color: var(--grey);
		width: 1px;
	}

	.comments {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}

	.comment-input {
		width: 100%;
		display: flex;
		gap: 1rem;
		align-items: center;
		margin-top: 1rem;
	}

	.comment-input > input {
		width: 100%;
	}
</style>
