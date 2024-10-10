<div class="outer-container">
	<div class="container">
		<h1>Post a project</h1>
		<form action="">
            <div style="width: 100%;">
                <label for="title" hidden>Project title: </label>
                <input style="width: 100%;" type="text" placeholder="Project title" />
            </div>
			<textarea name="" id="" placeholder="Project description"></textarea>
			<input type="text" placeholder="Project budget" />
			<input type="date" placeholder="Project deadline" />
			<input type="text" placeholder="Project categories" />
			<input type="text" placeholder="Project duration" />
			<input type="text" placeholder="Project status" />
			<input type="text" placeholder="Project team" />
			<br />

			<button>Create post</button>
		</form>
	</div>
</div>

<style>
	.outer-container {
		display: flex;
		flex-direction: column;
		margin: auto;
		width: 100%;
	}
    .container {
        width: 40%;
        margin: auto;
    }
	form {
		display: flex;
		flex-direction: column;
		justify-content: center;
		align-items: center;
	}

	* {
		border: none;
	}

	*:focus {
		outline: none;
	}

	input, textarea {
		padding: 0;
		padding: 1.5% 0% 1.5% 0%;
	}

</style>
