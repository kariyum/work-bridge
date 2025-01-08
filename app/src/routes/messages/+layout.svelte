<script lang="ts">
	import { goto } from '$app/navigation';

	let { data, children } = $props();
	if (data.status === 401) {
		goto('/');
	}
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
		width: 100%;
		display: flex;
		height: calc(100vh - 4rem);
	}

	.container {
		display: flex;
		width: 100%;
		margin: auto;
		height: 100%;
		/* gap: 0.5rem; */
	}

	.discussions {
		flex: 1;
		overflow-y: auto;
		/* border-radius: 5px; */
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
		padding: 1rem;
		font-weight: 500;
	}

	.discussion-container > a {
		display: block;
		padding: 1rem;
		text-decoration: none;
		background-color: transparent;
	}

</style>
