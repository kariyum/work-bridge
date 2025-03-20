<script lang="ts">
	import { fade, fly } from 'svelte/transition';
	import type { ToastInterface } from './Navbar.svelte';
	import { untrack } from 'svelte';
	import type { ProposalNotification } from '$lib/types';
	import { flip } from 'svelte/animate';

	const TIMEOUT = 3000;
	const PAUSE_BETWEEN_TOASTS = 0;
	let { toasts = $bindable([]) }: { toasts: ToastInterface[] } = $props();
	let loopId: number | undefined = undefined;

	let timer = $state(0);
	let show = $state(false);
	let setIntervalId: number | undefined = undefined;

	function renderToast() {
		if (toasts.length > 0) {
			console.log('IN f()');
			show = true;
			const currentToast = toasts.at(-1);
			timer = TIMEOUT / 1000;

			setTimeout(() => {
				show = false;
				currentToast?.remove();
			}, TIMEOUT);
		}
	}

	$effect(() => {
		toasts.length; // trigger effect run
		renderToast();
	});
</script>

{#snippet renderProposalNotificationToast(notif: ProposalNotification)}
	<div>
		Your application #{notif.content.proposal_id} has been {notif.content.proposal_status}
	</div>
{/snippet}

<div class="container" style:--toasts={toasts.length}>
	{#each toasts as toast, i (toast.id)}
		<div
			class="toast toast-style"
			in:fly={{ duration: 400, y: '60', opacity: 0.9 }}
			out:fly={{ duration: 400, y: '20' }}
			style:--n={toasts.length - i}
		>
			<!-- TIMER = {timer} -->
			{#if toast.notification.notification_type === 'proposal'}
				{@render renderProposalNotificationToast(toast.notification as ProposalNotification)}
			{:else}
				<div>Which notification to render?</div>
			{/if}
		</div>
	{/each}
</div>

<style>
	.container {
		--gap: 0.75rem;
		--hover-offset: 1rem;
		--toast-height: 5rem;
		--hidden-offset: 0.75rem;

		--hidden-toasts: calc(var(--toasts) - 1);

		position: fixed;
		right: 0rem;
		bottom: 0rem;
		margin: 1rem;

		display: flex;
		flex-direction: column-reverse;

		&:hover {
			height: calc(var(--toast-height) * var(--toasts));
		}

		&:hover .toast {
			scale: 1 !important;
			opacity: 1;
			translate: 0 calc(-1rem + -1 * (var(--n) - 1) * (var(--toast-height) + var(--gap))) !important;

			&:nth-last-child(n + 4) {
				visibility: visible;
			}

		}

		.toast {
			position: absolute;
			bottom: 0;
			right: 0;
			
			height: var(--toast-height);
			width: 20rem;
			
			transform-origin: 50% 0%;
			transition: all 350ms ease;

			&:nth-last-child(-n + 3) {
				z-index: 2;
				scale: 0.95;
				translate: 0 calc(-2 * var(--hidden-offset));
			}

			&:nth-last-child(-n + 2) {
				z-index: 3;
				scale: 0.975;
				translate: 0 calc(-1 * var(--hidden-offset));
			}

			&:nth-last-child(-n + 1) {
				z-index: 4;
				scale: 1;
				translate: 0;
			}

			&:nth-last-child(n + 4) {
				translate: 0 calc(-1 * var(--hidden-offset));
				scale: 0.95;
				visibility: hidden;
			}
		}

		.toast-style {
			pointer-events: auto;

			background-color: var(--toast-bg);
			padding: 1rem;
			border-radius: 10px;
			box-shadow: 0 -2px 10px rgba(0, 0, 0, 0.5);
		}
	}
</style>
