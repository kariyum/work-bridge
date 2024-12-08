<script lang="ts">
	import type { TaskClass } from '$lib/states.svelte';
	import Tag from './Tag.svelte';

	let {
		taskClass
	}: {
		taskClass: TaskClass;
	} = $props();

	let skillInput = $state('');

	function onKeydown(event: KeyboardEvent) {
		if (event.key === 'Enter') {
			taskClass.addSkill(skillInput);
			skillInput = '';
			event.preventDefault();
		}
	}
</script>

<div class="container">
	{#each taskClass.skills as skill, index}
		<div>
			<Tag onClose={() => taskClass.removeSkillIndex(index)}>
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
