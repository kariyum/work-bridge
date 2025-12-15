<script lang="ts">
    let { data } = $props();
</script>
<h1>Change Your Password</h1>
<div class="input-label">
    <input type="text" placeholder=" ">
    <label for="email">Current Password</label>
</div>
<div class="input-label">
    <input type="text" placeholder=" ">
    <label for="email">New Password</label>
</div>
<div class="input-label">
    <input type="text" placeholder=" ">
    <label for="email">Confirm New Password</label>
</div>
<button>Update!</button>
<style>
    input {
        max-width: 60%;
        width: 100%;
    }
</style>