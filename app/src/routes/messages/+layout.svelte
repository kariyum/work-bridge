<script lang="ts">
	import { CircleUserRound } from 'lucide-svelte';

	let { data, children } = $props();
	let titles = $derived.by(() => {
		const result = new Map<number, string>();
		data.discussions.map((discussion) => {
			const title = discussion.user_ids
				.filter((email) => email != data.user?.email)
				.reduce((userA, userB) => userA + userB, '');
			result.set(discussion.id, title);
		});
		return result;
	});
	
</script>

<div class="component">
	<div class="container">
		<div class="discussions">
			<div class="header">
				<div>Discussions</div>
			</div>
			{#each data.discussions as discussion}
				{@const title = discussion.title ?? titles.get(discussion.id)}
				<div class="discussion-container">
					<a href="/messages/{discussion.id}">
						<CircleUserRound />
						{title}
					</a>
				</div>
			{/each}
		</div>
		<div class="messages-col">
			{@render children()}
		</div>
	</div>
</div>

<style>
	.component {
		height: calc(100vh - 4rem);
	}

	.container {
		display: flex;
		width: 100%;
		margin: auto;
		height: 100%;
	}

	.discussions {
		flex: 1;
		min-width: 33ch;
		overflow-y: auto;
		border-right: 2px solid var(--blue);
	}

	.messages-col {
		flex: 5;
		display: flex;
		flex-direction: column;
	}

	.header {
		font-size: 1.5rem;
	}

	.header > div {
		font-weight: 500;
	}

	.discussion-container > a {
		gap: 1rem;
		display: flex;
		padding: 1rem;
		text-decoration: none;
		background-color: transparent;
		border: 2px solid transparent;
		font-size: large;
		color: var(--font-color);
	}

	.discussion-container > a:hover {
		border: 2px solid var(--blue);
		background-color: var(--blue);
	}

</style>
