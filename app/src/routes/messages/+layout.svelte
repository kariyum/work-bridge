<script lang="ts">
	import { page } from '$app/state';
	import { CircleUserRound } from 'lucide-svelte';

	let { data, children } = $props();
	const discussionIdMatcher = /messages\/(?<id>\d)/
	const selectedDiscussion: string | undefined = $derived(discussionIdMatcher.exec(page.url.pathname)?.groups?.id || undefined);
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
		<div class="menu">
			<div class="header">
				<div>Discussions</div>
			</div>
			<div class="discussions">
				{#each data.discussions as discussion}
					{@const title = discussion.title ?? titles.get(discussion.id)}
					<div class="discussion-container">
						<a href="/messages/{discussion.id}" data-selected={selectedDiscussion === discussion.id.toString()}>
							<CircleUserRound />
							{title}
						</a>
					</div>
				{/each}
			</div>
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

	.discussion-container {
		margin-left: 0.5rem;
	}

	.container {
		display: flex;
		width: 100%;
		margin: auto;
		height: 100%;
	}

	.menu {
		min-width: 33ch;
		overflow-y: auto;
		display: flex;
		flex-direction: column;
		gap: 1rem;
		margin-top: 1rem;
		/* border-right: 2px solid var(--blue); */
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
	
	a {
        position: relative;
		gap: 1rem;
		display: flex;
		text-decoration: none;
		background-color: transparent;
		border: 2px solid transparent;
		color: var(--font-color);
        padding: 0.5rem;
        border-radius: 3px 0 0 3px;
	}

    a[data-selected = "true"] {
        background-color: var(--selected-color);
    }

    a[data-selected = "true"]::before {
        content: "";
        position: absolute;
        width: 5px;
        height: 2.2rem;
        border-radius: 10px;
        background-color: var(--blue);
        left: -0.6rem;
        top: 0.15rem;
    }

    a:hover {
        background-color: var(--hover-color);
    }

	.discussions {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}
</style>
