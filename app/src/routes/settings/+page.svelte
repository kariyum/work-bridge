<script lang="ts">
	import { themeService } from '$lib/theme/theme-service';
</script>
<div class="container">
    <h1>Settings</h1>
    <div class="settings-grid">
        <button class="theme-button" on:click={() => themeService.toggle()}>
            {#if themeService.getStore() === "light"}
                Switch to Dark Mode
            {:else}
                Switch to Light Mode
            {/if}
        </button>
    </div>
</div>

<style>
    .container {
        color: var(--dark-text);
    }

	.theme-button {
        background-color: var(--btn-bg);
        color: var(--dark-text);
        border: 2px solid var(--border);
        border-radius: 50px;
        padding: 12px 24px;
        cursor: pointer;
        transition: background-color 0.3s ease, color 0.3s ease;
	}
</style>