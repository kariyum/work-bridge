<script lang="ts">
	import type { TaskClass } from '$lib/states.svelte';
	import Tag from './Tag.svelte';

	let {
		skillsIn = $bindable([]),
		addSkill, 
		removeSkillAtIndex
	}: {
		skillsIn: string[],
		addSkill: (skill: string) => void,
		removeSkillAtIndex: (index: number) => void
	} = $props();

	let skillInput = $state('');

	function onKeydown(event: KeyboardEvent) {
		if (event.key === 'Enter') {
			addSkill(skillInput);
			skillInput = '';
			event.preventDefault();
		}
	}
</script>

<div class="container">
	{#each skillsIn as skill, index}
		<div>
			<Tag onClose={() => removeSkillAtIndex(index)}>
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
