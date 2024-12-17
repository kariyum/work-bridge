<script lang="ts">
	import { userStore } from "$lib/storage";


</script>

<div class="container">
    <h1>You are {$userStore?.email}</h1>
</div>

<style>
    .container {
        display: flex;
        justify-content: center;
        align-items: center;
    }
</style>
