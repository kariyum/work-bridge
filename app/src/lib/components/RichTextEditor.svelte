<script lang="ts">
	import { onMount, onDestroy } from 'svelte';
	import { Editor } from '@tiptap/core';
	import StarterKit from '@tiptap/starter-kit';
	import Link from '@tiptap/extension-link';

	let element: HTMLDivElement;
	let rerender = $state(false);
	let editor: Editor | undefined = $state();

	$effect(() => {
		if (editor) {
			console.log('EDITOR STATE CHANGED!!!');
		}
	});

	let { x = $bindable() }: { x: string } = $props();
	onMount(() => {
		console.log('Onmount richtexteditor');
		editor = new Editor({
			editorProps: {
				attributes: {
					class: 'editor'
				}
			},
			element: element,
			extensions: [
				StarterKit,
				Link.configure({
					openOnClick: false,
					autolink: true,
					defaultProtocol: 'https'
				})
			],
			content: x,
			onTransaction: () => {
				// force re-render so `editor.isActive` works as expected
				// editor = editor;
				// rerender = !rerender;
			}
		});
		editor.on('update', ({ editor }) => {
			x = editor.getHTML();
		});
	});

	onDestroy(() => {
		if (editor) {
			editor.destroy();
		}
	});
</script>

<div>
	{#if editor}
		<div class="outer-button-container">
			<div class="button-container">
				{#key rerender}
					<div class="button-group">
						<button
							onclick={() => {
								editor?.chain().focus().toggleHeading({ level: 1 }).run();
								rerender = !rerender;
							}}
							class:active={editor.isActive('heading', { level: 1 })}
						>
							H1
						</button>
						<button
							onclick={() => {
								editor?.chain().focus().toggleHeading({ level: 2 }).run();
								rerender = !rerender;
							}}
							class:active={editor.isActive('heading', { level: 2 })}
						>
							H2
						</button>
						<button
							onclick={() => {
								editor?.chain().focus().setParagraph().run();
								rerender = !rerender;
							}}
							class:active={editor.isActive('paragraph')}
						>
							P
						</button>
					</div>

				<div class="vertical-separator"></div>
				<div class="button-group">
					<button
						onclick={() => {
							editor?.chain().focus().toggleBold().run();
							rerender = !rerender;
						}}
						class:active={editor.isActive('bold')}
					>
						B
					</button>
					<button
						onclick={() => {
							editor?.chain().focus().toggleItalic().run();
							rerender = !rerender;
						}}
						class:active={editor.isActive('italic')}
					>
						I
					</button>
				</div>
				<div class="vertical-separator"></div>
				<div class="button-group">
					<button
						onclick={() => {
							editor?.chain().focus().toggleOrderedList().run();
							rerender = !rerender;
						}}
						class:active={editor.isActive('orderedList')}
					>
						Ol
					</button>
					<button
						onclick={() => {
							editor?.chain().focus().toggleBulletList().run();
							rerender = !rerender;
						}}
						class:active={editor.isActive('bulletList')}
					>
						Ul
					</button>
				</div>
				{/key}

			</div>
		</div>
	{/if}
	<div bind:this={element}></div>
</div>

<!-- 
<hr />

Displayed:
<pre>
	{@html content}
</pre> -->

<style>
	button.active {
		background: #ddd;
	}

	button {
		display: flex;
		align-items: center;
		justify-content: center;
		border: 1px solid #fff;
		border-radius: 5px;
		padding: 0.2rem 0.5rem;
		background-color: #fff;
		cursor: pointer;
		width: 2rem;
	}

	.vertical-separator {
		width: 1px;
		align-self: stretch;
		background-color: #ccc;
	}
	.outer-button-container {
		border: 1px solid #ccc;
		border-radius: 5px 5px 0 0;
		border-bottom: 0;
	}

	.button-container {
		display: flex;
		border-radius: 5px 5px 0 0;
		flex-direction: row nowrap;
		padding: 0.4rem;
		width: max-content;
		justify-content: space-around;
	}

	.button-group {
		display: flex;
		flex-direction: row nowrap;
		justify-content: space-between;
		margin-left: 0.5rem;
		margin-right: 0.5rem;
	}

	.button-group:first-child {
		margin-right: 0.5rem;
		margin-left: 0rem;
	}

	.button-group:last-child {
		margin-left: 0.5rem;
		margin-right: 0rem;
	}

	:global(.editor) {
		padding: 0.5rem;
		border: 1px solid #ccc;
		border-radius: 0 0 5px 5px;
		min-height: 30vh;
	}

	:global(.editor:focus) {
		outline: none;
	}

	:global(ol) {
		margin-left: 2rem;
	}

	:global(ul) {
		margin-left: 2rem;
	}

	:global(p) {
		display: block;
		min-height: 1rem;
	}
</style>
