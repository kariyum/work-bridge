<script lang="ts">
	let progress = $state(100);
	let startTimestamp: number | null = $state(null);
	let { isPaused, duration }: { isPaused: boolean, duration: number } = $props();

    let pauseStart: number | undefined = $state();
	let pauseDuration = $state(0);

	function animateProgress(timestamp: DOMHighResTimeStamp) {
		if (!startTimestamp) {
			startTimestamp = timestamp;
		}
        if (pauseStart) {
            pauseDuration += timestamp - pauseStart;
        }
        if (isPaused) {
			pauseStart = timestamp;
			return;
		}
        pauseStart = undefined;
		const elapsed = timestamp - startTimestamp - pauseDuration;
		progress = Math.max(100 - (elapsed / duration) * 100, 0);
		if (progress > 0) {
			requestAnimationFrame(animateProgress);
		}
	}
	$effect(() => {
		isPaused;
		if (!isPaused) {
			requestAnimationFrame(animateProgress);
		}
	});
</script>

<div class="container">
	<div class="bar" style:width="{progress}%"></div>
</div>

<style>
	.container {
		--width: 3rem;
		width: var(--width);
		background-color: black;
		border-radius: 15px;

		.bar {
			height: 0.3rem;
			background-color: red;
			border-radius: 15px;
			transition: width 0.03s linear;

			height: 0.3rem;
		}
	}
</style>
