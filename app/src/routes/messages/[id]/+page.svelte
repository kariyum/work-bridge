<script lang="ts">
	import Conversation from '$lib/components/conversation/Conversation.svelte';
	const { data } = $props();
</script>

{#if data.user && data.discussions}
	<!-- <Conversation
		discussion_id={data.discussion_id}
		receivers={[]}
		remoteMessages={data.messages}
		user={data.user}
	/> -->
{:else}
	<div>User is undefined or discussions are undefined</div>
{/if}
