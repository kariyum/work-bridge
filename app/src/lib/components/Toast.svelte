<script lang="ts">
	import { fade, fly } from 'svelte/transition';
	import type { ToastInterface } from './Navbar.svelte';
	import { untrack } from 'svelte';
	import type { ProposalNotification } from '$lib/types';

	const TIMEOUT = 4000;
	const PAUSE_BETWEEN_TOASTS = 1000;
	let { toasts = $bindable([]) }: { toasts: ToastInterface[] } = $props();
	let currentToast: ToastInterface | undefined = $state();
	let loopId: number | undefined = undefined;

	let timer = $state(0);
	let show = $state(false);
	let setIntervalId: number | undefined = undefined;

	function renderToast() {
		if (!currentToast && toasts.length > 0) {
			console.log('IN f()');
			show = true;
			currentToast = toasts.at(0);
			timer = TIMEOUT / 1000;
			clearInterval(setIntervalId);
			setIntervalId = setInterval(() => {
				timer -= 1;
				if (timer < 0) {
					timer = 0;
				}
			}, 1000);

			setTimeout(() => {
				show = false;
				setTimeout(() => {
					currentToast?.remove();
					currentToast = undefined;
					console.log('Removed', currentToast);
					if (toasts.length > 0) {
						console.log('Called again');
						show = false;
						console.log('IN SET TIMEOUT???');
						renderToast();
					}
				}, PAUSE_BETWEEN_TOASTS);
			}, TIMEOUT);
		}
		console.log('OUT OF f()');
	}

	$effect(() => {
		toasts.at(0); // trigger effect run
		renderToast();
	});
</script>

{#snippet renderProposalNotificationToast(notif: ProposalNotification)}
	<div>
		Your application #{notif.content.proposal_id} has been {notif.content.proposal_status}
	</div>
{/snippet}

{#if currentToast && show}
	<div transition:fly={{ y: 20, duration: 1000 }} class="toast">
		<!-- TIMER = {timer} -->
		{#if currentToast.notification.notification_type === 'proposal'}
			{@render renderProposalNotificationToast(currentToast.notification as ProposalNotification)}
		{:else}
			<div>
				Which notification to render?
			</div>
		{/if}
	</div>
{/if}

<style>
	.toast {
		position: absolute;
		left: 50%;
		transform: translateX(-50%);
		bottom: 1.5rem;
		background-color: var(--blue);
		padding: 1rem;
		border-radius: 5px;
	}
</style>
