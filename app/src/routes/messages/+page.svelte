<div class="messages-col">
	<div class="messages">
		<h1>Select a discussion please! 😊</h1>
	</div>
</div>

<style>
	h1 {
		font-size: 1.5rem;
		color: #333;
		background-color: #f7f7f7;
		position: relative;
		top: 50%;
		left: 50%;
		transform: translate(-50%, -50%);
		width: fit-content;
	}
	.component {
		width: 100%;
		height: 80vh;
		display: flex;
	}

	.container {
		display: flex;
		width: 90%;
		margin: auto;
		height: 100%;
		gap: 1rem;
	}

	.discussions {
		flex: 1;
		padding: 1rem;
		overflow-y: auto;
		border: 1px solid rgb(204, 204, 204);
		border-radius: 5px;
	}

	.messages-col {
		flex: 3;
		display: flex;
		flex-direction: column;
		background-color: #fff;
	}

	.header {
		font-size: 1.5rem;
		margin-bottom: 1rem;
		color: #333;
		width: min-content;
		margin: auto;
	}

	.messages {
		flex-grow: 1;
		overflow-y: auto;
		padding: 1rem;
		border: 1px solid #ddd;
		border-radius: 5px;
		background-color: #f7f7f7;
	}

	.message {
		padding: 0.75rem;
		border-radius: 9px;
		background-color: #e0e0e0;
		width: fit-content;
		max-width: 70%;
	}

	.message[data-sender='me'] {
		margin-left: auto;
		background-color: #007bff;
		color: #fff;
	}

	.message {
		margin: 0.5rem 0.5rem 0.5rem 0.5rem;
	}

	.input {
		display: flex;
		width: 100%;
		padding-top: 1rem;
	}

	.input-form {
		display: flex;
		width: 100%;
	}

	.input-form input[type='text'] {
		flex: 1;
		padding: 0.75rem;
		border: 1px solid #ccc;
		border-radius: 5px;
		margin-right: 0.5rem;
	}

	.input-form input[type='submit'] {
		padding: 0.75rem 1rem;
		border-radius: 5px;
		cursor: pointer;
	}
</style>
