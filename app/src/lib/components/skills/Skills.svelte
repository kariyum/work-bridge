<script lang="ts">
	import Tag from './Tag.svelte';

	let {
		skillsIn = $bindable([]),
		addSkill,
		removeSkillAtIndex
	}: {
		skillsIn: string[];
		addSkill: (skill: string) => void;
		removeSkillAtIndex: (index: number) => void;
	} = $props();

	let skillInput = $state('');

	function onKeydown(event: KeyboardEvent) {
		if (event.key === 'Enter') {
			addSkill(skillInput);
			skillInput = '';
			event.preventDefault();
		}
	}
	let isFocused = $state(false);
</script>

<div class="container" class:blue-outline={isFocused}>
	{#each skillsIn as skill, index}
		<div>
			<Tag onClose={() => removeSkillAtIndex(index)}>
				<p>
					{skill}
				</p>
			</Tag>
		</div>
	{/each}
	<input
		type="text"
		onfocus={() => (isFocused = true)}
		onblur={() => (isFocused = false)}
		placeholder=" "
		bind:value={skillInput}
		onkeydown={onKeydown}
	/>
	<label for="" class:focused-label={isFocused || skillsIn.length > 0}>Skills</label>
</div>

<style>
	label {
		position: absolute;
		top: 50%;
		left: 6px;
		transform: translateY(-50%);
		color: #5f6368;
		pointer-events: none;
		background-color: var(--input-bg);
		transition:
			top 0.2s cubic-bezier(0.4, 0, 0.2, 1),
			left 0.2s cubic-bezier(0.4, 0, 0.2, 1),
			font-size 0.2s cubic-bezier(0.4, 0, 0.2, 1);
		border-radius: 5px;
		font-size: 16px;
		padding: 0 6px;
	}
	.focused-label {
		top: 0;
		left: 6px;
		font-size: 14px;
		transform: translateY(-50%);
	}
	.container {
		position: relative;
		width: max-width;
		display: flex;
		flex-wrap: wrap;
		align-items: center;
		gap: 0.5rem;
		background-color: var(--input-bg);
		border: 2px solid var(--border);
		padding: 0.5rem;
		border-radius: 5px;
	}
	input {
		color: inherit;
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
	.blue-outline {
		outline: 2px solid var(--blue);
		outline-offset: -2px;
	}
	p {
		line-height: 1.2;
	}
</style>
