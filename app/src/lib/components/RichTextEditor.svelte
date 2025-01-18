<script lang="ts">
	import { onMount, onDestroy, untrack } from 'svelte';
	import { Editor } from '@tiptap/core';
	import StarterKit from '@tiptap/starter-kit';
	import Link from '@tiptap/extension-link';
	import { Bold, Heading1, Heading2, Italic, List, ListOrdered, Pilcrow } from 'lucide-svelte';

	let element: HTMLDivElement;
	let myEditor: Editor | undefined = $state();

	let { contentIn, x = $bindable() }: { x: string; contentIn: string } = $props();
	$effect.pre(() => {
		contentIn;
		untrack(() => {
			myEditor?.commands.setContent(contentIn);
		});
	});
	onMount(() => {
		myEditor = new Editor({
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
			onTransaction: (editor) => {
				// force re-render so `editor.isActive` works as expected
				myEditor = undefined;
				myEditor = editor.editor;
			}
		});
		myEditor.on('update', ({ editor }) => {
			x = editor.getHTML();
		});
	});

	onDestroy(() => {
		if (myEditor) {
			myEditor.destroy();
		}
	});
</script>

<div>
	<div class="outer-button-container">
		<div class="button-container">
			<div class="button-group">
				<button
					onclick={() => {
						myEditor?.chain().focus().toggleHeading({ level: 1 }).run();
					}}
					class:active={myEditor?.isActive('heading', { level: 1 })}
				>
					<Heading1 />
				</button>
				<button
					onclick={() => {
						myEditor?.chain().focus().toggleHeading({ level: 2 }).run();
					}}
					class:active={myEditor?.isActive('heading', { level: 2 })}
				>
					<Heading2 />
				</button>
				<button
					onclick={() => {
						myEditor?.chain().focus().setParagraph().run();
					}}
					class:active={myEditor?.isActive('paragraph')}
				>
					<Pilcrow />
				</button>
			</div>

			<div class="vertical-separator"></div>
			<div class="button-group">
				<button
					onclick={() => {
						myEditor?.chain().focus().toggleBold().run();
					}}
					class:active={myEditor?.isActive('bold')}
				>
					<Bold />
				</button>
				<button
					onclick={() => {
						myEditor?.chain().focus().toggleItalic().run();
					}}
					class:active={myEditor?.isActive('italic')}
				>
					<Italic />
				</button>
			</div>
			<div class="vertical-separator"></div>
			<div class="button-group">
				<button
					onclick={() => {
						myEditor?.chain().focus().toggleOrderedList().run();
					}}
					class:active={myEditor?.isActive('orderedList')}
				>
					<ListOrdered />
				</button>
				<button
					onclick={() => {
						myEditor?.chain().focus().toggleBulletList().run();
					}}
					class:active={myEditor?.isActive('bulletList')}
				>
					<List />
				</button>
			</div>
		</div>
	</div>
	<div bind:this={element}></div>
</div>

<!-- 
<hr />

Displayed:
<pre>
	{@html content}
</pre> -->

<style>
	div {
		background-color: Canvas;
	}

	.active {
		background-color: var(--border);
	}

	button {
		display: flex;
		align-items: center;
		justify-content: center;
		border: none;
		border-radius: 5px;
		padding: 0.2rem 0.5rem;
		cursor: pointer;
		aspect-ratio: 1;
		background-color: canvas;
	}

	.vertical-separator {
		width: 2px;
		align-self: stretch;
		background-color: var(--border);
	}
	.outer-button-container {
		border: 2px solid var(--border);
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
		gap: 0.5rem;
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
		border: 2px solid var(--border);
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
