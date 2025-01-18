<script lang="ts">
    import { X } from 'lucide-svelte';
    let { children, onClose } = $props();

</script>


<div class="container">
    {@render children()}
    <button onclick={onClose}>
        <X size="12" />
    </button>
</div>

<style>
    .container {
        display: flex;
        gap: 0.5rem;
        background-color: var(--background-color);
        padding: 0.2rem 0.5rem;
        align-items: center;
        font-size: small;
        border-radius: 50px;
    }
    button {
        border: none;
        border-radius: 50%;
        margin: 0;
        width: fit-content;
        height: fit-content;
        padding: 0.1rem;
        line-height: 0;
    }
</style>