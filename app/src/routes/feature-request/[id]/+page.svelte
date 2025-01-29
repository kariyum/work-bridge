<script lang="ts">
	import { ThumbsUp, ThumbsDown, SendHorizontal } from 'lucide-svelte';

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
            <div class="vote">
                <button class="emoji no-line-height"><ThumbsUp size="18" /></button>
                <div>10</div>
            </div>
            <div class="vertical-line"></div>
            <div class="vote">
                <button class="emoji no-line-height"><ThumbsDown size="18" /></button>
                <div>10</div>
            </div>
		</div>
		<div class="comments">
			<h2>Comments</h2>
			<div class="all-comments">
				No comments yet.
			</div>
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
        gap: 1rem;
		background-color: var(--btn-bg);
		width: fit-content;
		padding: 0.3rem 0.7rem;
		border-radius: 2rem;
    }

    .vote {
        display: flex;
        align-items: center;
        gap: 0.5rem;
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
