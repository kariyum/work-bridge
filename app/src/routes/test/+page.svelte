<script lang="ts">
	import { onMount, onDestroy } from 'svelte';
	import { Editor } from '@tiptap/core';
	import StarterKit from '@tiptap/starter-kit';
	import Link from '@tiptap/extension-link';

	let element: HTMLDivElement;
	let editor: Editor | undefined;

	let content = '';
	onMount(() => {
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
			content: '<p>Placeholder</p>',
			onTransaction: () => {
				// force re-render so `editor.isActive` works as expected
				editor = editor;
			}
		});
		editor.on('update', ({ editor }) => {
			// console.log('editor html', editor.getHTML());
			content = editor.getHTML();
		});
	});

	onDestroy(() => {
		if (editor) {
			editor.destroy();
		}
	});
</script>

{#if editor}
	<div class="button-container">
		<button
			onclick={() => editor?.chain().focus().toggleHeading({ level: 1 }).run()}
			class:active={editor.isActive('heading', { level: 1 })}
		>
			H1
		</button>
		<button
			onclick={() => editor?.chain().focus().toggleHeading({ level: 2 }).run()}
			class:active={editor.isActive('heading', { level: 2 })}
		>
			H2
		</button>
		<button
			onclick={() => editor?.chain().focus().setParagraph().run()}
			class:active={editor.isActive('paragraph')}
		>
			P
		</button>
		<button
			onclick={() => editor?.chain().focus().toggleBold().run()}
			class:active={editor.isActive('bold')}
		>
			B
		</button>
		<button
			onclick={() => editor?.chain().focus().toggleItalic().run()}
			class:active={editor.isActive('i')}
		>
			I
		</button>
		<button
			onclick={() => editor?.chain().focus().toggleOrderedList().run()}
			class:active={editor.isActive('orderedList')}
		>
			Ordered List
		</button>
		<button
			onclick={() => editor?.chain().focus().toggleBulletList().run()}
			class:active={editor.isActive('bulletList')}
		>
			Unordered List
		</button>
	</div>
{/if}

<div bind:this={element}></div>

<!-- 
<hr />

Displayed:
<pre>
	{@html content}
</pre> -->

<style>
	button.active {
		background: black;
		color: white;
	}
	button-container {
		display: flex;
	}

	:global(.editor) {
		margin: 1rem;
		padding: 0.5rem;
		border: 1px solid #ccc;
		border-radius: 4px;
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
