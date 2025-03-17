<script lang="ts">
	import { browser } from '$app/environment';
	import { WebSocketService } from '$lib/realtime';
	import type { BaseNotification, ProposalNotification } from '$lib/types';
	import { onMount } from 'svelte';

	let webSocketService: WebSocketService;
	function onProposalNotificationHandler(payload: ProposalNotification) {
		baseNotifications.push(payload);
	}
	
	let { notifications }: { notifications: BaseNotification[] } = $props();
	
	let baseNotifications: BaseNotification[] = $derived.by(() => {
		notifications;
		let result = $state([]);
		return result;
	})
	let allNotifications = $derived.by(() => {
		let sortedNotifications = [...notifications, ...baseNotifications];
		sortedNotifications.sort((a, b) => b.created_at.getTime() - a.created_at.getTime());
		let merged = $state(sortedNotifications);

		return merged;
	});

	onMount(() => {
		if (browser) {
			webSocketService = WebSocketService.getInstance();
			webSocketService.subscribeToProposalNotifications(onProposalNotificationHandler);
		}
	});
</script>

{#snippet renderProposalNotification(notif: ProposalNotification)}
	<div class="proposal">
		{#if notif.content.proposal_status === 'accepted'}
			<p>
				<span style="color:green;">Congradulations</span>! Your application #{notif.content
					.proposal_id} have been accepted!
			</p>
		{:else if notif.content.proposal_status === 'rejected'}
			<p>
				Your application #{notif.content.proposal_id} have been declined.
			</p>
		{/if}
		<div>
			{notif.created_at.getMinutes()}:{notif.created_at.getSeconds()}
		</div>
	</div>
{/snippet}

<div>
	<h1>Notifications</h1>
	<div>
		{#each allNotifications as notification}
			{#if notification.notification_type === 'proposal'}
				{@render renderProposalNotification(notification as ProposalNotification)}
			{/if}
		{/each}
	</div>
</div>

<style>
	.proposal {
		margin: 0.5rem 0 0.5rem 0;
	}
</style>
