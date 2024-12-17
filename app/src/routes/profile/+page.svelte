<script lang="ts">
    let { data } = $props();
</script>

<div class="container">
    <h1>You are {data.user?.email}</h1>
</div>

<style>
    .container {
        display: flex;
        justify-content: center;
        align-items: center;
    }
</style>
