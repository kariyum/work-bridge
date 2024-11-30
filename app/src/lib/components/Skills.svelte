<script lang="ts">
	import Tag from './Tag.svelte';

	let { skills = $bindable() }: { skills: string[] } = $props();
	let skillInput = $state('');
	function remove(index: number) {
		skills.splice(index, 1);
	}
	function onKeydown(event: KeyboardEvent) {
		if (event.key === 'Enter') {
			skills.push(skillInput);
			skillInput = '';
			event.preventDefault();
		}
	}
	$effect(() => {
		console.log(skillInput, skillInput.length);
	});
</script>

<div class="container">
	{#each skills as skill, index}
		<div>
			<Tag onClose={() => remove(index)}>
                <p>
                    {skill}
                </p>
            </Tag>
		</div>
	{/each}
	<input type="text" placeholder="Add a skill" bind:value={skillInput} onkeydown={onKeydown} />
</div>

<style>
	.container {
        width: max-width;
		display: flex;
		flex-wrap: wrap;
		align-items: center;
		gap: 0.5rem;
	}
	input {
		margin: 0;
		padding: 0.2rem;
		height: min-content;
		flex-grow: 1;
		border: none;
		width: max-width;
		word-wrap: break-word;
		resize: none;
	}
	input:focus {
		outline: none;
	}
    p {
        line-height: 1.2;
    }
</style>
