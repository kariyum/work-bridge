<script lang="ts">
	import { ProposalNotification, type BaseNotification } from '$lib/types';
	import { computeTimeAgo } from '$lib/utils';

	let { notifications }: { notifications: BaseNotification[] } = $props();
</script>

{#snippet renderProposalNotification(notif: ProposalNotification)}
	<a class="notif-container proposal" href={ProposalNotification.getHref(notif)}>
		<p>
			{ProposalNotification.getContent(notif)}
		</p>
		<div class="timeago">
			{computeTimeAgo(notif.created_at)}
		</div>
	</a>
{/snippet}

<div>
	<h1>Notifications</h1>
	<div>
		{#if notifications.length}
			{#each notifications as notification}
				{#if notification.notification_type === 'proposal'}
					{@render renderProposalNotification(notification as ProposalNotification)}
				{/if}
			{/each}
		{:else}
			<div>You do not have notifications yet!</div>
		{/if}
	</div>
</div>

<style>
	.proposal {
		margin: 0.5rem 0 0.5rem 0;
	}

	.notif-container {
		display: flex;
		flex-direction: column;
		text-decoration: none;
		color: inherit;

		padding: 0.5rem;
		border-radius: 10px;

		&:hover {
			background-color: var(--toast-bg);
		}

		.timeago {
			font-size: smaller;
			font-weight: 500;
		}
	}
</style>
