<script lang="ts">
	import NotificationMenu from '$lib/components/notification/NotificationMenu.svelte';

	let { data } = $props();
</script>

<NotificationMenu notifications={data.notifications} />
