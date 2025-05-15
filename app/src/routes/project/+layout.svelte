<script lang="ts">
	let { children } = $props();
</script>

<div class="page">
	{@render children()}
</div>

<style>
    .page {
        padding: 1rem;
        padding-top: 0;
    }
</style>