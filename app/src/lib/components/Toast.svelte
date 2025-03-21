<script lang="ts">
	import { fade, fly } from 'svelte/transition';
	import type { ToastInterface } from './Navbar.svelte';
	import { untrack } from 'svelte';
	import type { ProposalNotification } from '$lib/types';
	import { flip } from 'svelte/animate';
	import Progress from './Progress.svelte';
	import { X } from 'lucide-svelte';
	import { SvelteMap } from 'svelte/reactivity';

	const TIMEOUT = 3000;
	let isPaused: boolean = $state(false);
	let pauseStartTimestamp: number | undefined = $state();
	let { toasts = $bindable([]) }: { toasts: ToastInterface[] } = $props();
	
	let pauseDurations = new SvelteMap<number, number>();
	let timeouts = new SvelteMap<number, number>();
	let startTimes = new SvelteMap<number, number>();

	function renderToast(currentToast: ToastInterface) {
		clearTimeout(timeouts.get(currentToast.id));
		if (!startTimes.get(currentToast.id)) {
			startTimes.set(currentToast.id, performance.now());
		}
		const elapsed =
			performance.now() - ((startTimes.get(currentToast.id) ?? 0) + (pauseDurations.get(currentToast.id) ?? 0));
		const remaining = Math.max(TIMEOUT - elapsed, 0);
		const timeoutId = setTimeout(() => {
			if (
				startTimes.get(currentToast.id) &&
				performance.now() - (startTimes.get(currentToast.id) ?? 0) - (pauseDurations.get(currentToast.id) ?? 0) >= TIMEOUT
			) {
				pauseDurations.delete(currentToast.id);
				currentToast.remove();
			} else {
				renderToast(currentToast);
			}
		}, remaining);
		timeouts.set(currentToast.id, timeoutId);
	}

	$effect(() => {
		toasts.length; // trigger effect run
		untrack(() => {
			const tail = toasts.at(-1);
			tail && renderToast(tail);
		});
	});
</script>

{#snippet renderProposalNotificationToast(notif: ProposalNotification)}
	<div>
		<Progress {isPaused} duration={TIMEOUT} />
		<div>
			Your application #{notif.content.proposal_id} has been {notif.content.proposal_status}
		</div>
		<!-- <button>
			<X />
		</button> -->
	</div>
{/snippet}

<!-- TODO REMOVE ROLE!!!! -->
<div
	role="alert"
	class="container"
	style:--toasts={toasts.length}
	onmouseenter={() => {
		isPaused = true;
		pauseStartTimestamp = performance.now();
		toasts.forEach((toast) => {
			clearInterval(timeouts.get(toast.id));
		});
	}}
	onmouseleave={() => {
		isPaused = false;
		if (pauseStartTimestamp) {
			toasts.forEach((toast) => {
				if (pauseStartTimestamp && startTimes.get(toast.id)) {
					const duration =
						(pauseDurations.get(toast.id) ?? 0) + performance.now() - pauseStartTimestamp;
					pauseDurations.set(toast.id, duration);
					renderToast(toast);
				}
			});
		}
		pauseStartTimestamp = undefined;
	}}
>
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
		width: 20rem;

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
