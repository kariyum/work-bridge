<div class="messages-col">
	<div class="messages">
		<h1>Select a discussion please! 😊</h1>
	</div>
</div>

<style>
	h1 {
		font-size: 1.5rem;
		background-color: var(--bg);
		position: relative;
		top: 50%;
		left: 50%;
		transform: translate(-50%, -50%);
		width: fit-content;
	}
	
	.messages-col {
		flex: 3;
		display: flex;
		flex-direction: column;
		background-color: var(--bg);
	}

	.messages {
		flex-grow: 1;
		overflow-y: auto;
		padding: 1rem;
		border: 1px solid #ddd;
		border-left: none;
		color: var(--dark-text);
		/* border-radius: 5px; */
		background-color: var(--bg);
	}
</style>
