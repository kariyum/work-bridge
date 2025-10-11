<script lang="ts">
    import { X } from 'lucide-svelte';
    let { children, onClose } = $props();

</script>


<span class="container">
    {@render children()}
    <button onclick={onClose}>
        <X size="12" />
    </button>
</span>

<style>
    .container {
        background-color: var(--selected-color);
        padding: 0.2rem 0.5rem;
        font-size: small;
        border-radius: 50px;
        text-wrap: nowrap;
    }
    button {
        display: inline;
        border: none;
        border-radius: 50%;
        margin: 0;
        width: fit-content;
        height: fit-content;
        padding: 0.1rem;
        line-height: 0;
    }
</style>