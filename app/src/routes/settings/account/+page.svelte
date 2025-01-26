<script lang="ts">
    let { data } = $props();
</script>
<div>
    <label for="email">Email</label>
    <p>{data.user?.email}</p>
</div>