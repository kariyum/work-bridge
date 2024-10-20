<div class="outer-container">
	<div class="container">
		<h1>Create a project</h1>
		<form action="">
			<div class="input">
				<label for="title">Project Title</label>
				<input type="text" placeholder="e.g. Business Website" />
			</div>
			<div class="input">
				<label for="">Project Description</label>
				<textarea name="" id="" placeholder="The project is about designing and implementing..."></textarea>
			</div>

			<div class="input">
				<label for="">Budget</label>
				<input type="text" placeholder="500 DT" />
			</div>

			<div class="input">
				<label for="">Deadline</label>
				<input type="date" placeholder="Project deadline" />
			</div>
			<div class="input">
				<label for="">Project Categories</label>
				<select name="" id="">
					<option value="" disabled selected>Select your option</option>
					<option value="kk">Web developer</option>
					<option value="kk">UI/UX designer</option>
					<option value="kk">Architect</option>
				</select>
			</div>

			<div class="input">
				<label for="">Start Date</label>
				<input type="text" placeholder="Start Date" />
			</div>

			<button>Create post</button>
		</form>

		<div>
			<div class="header">
				<div>Mobile fitness app</div>
				<div>500 DT</div>
			</div>

			<div>Start date: 10-10-2024</div>

			<div style="margin-top: 1rem;">
				<p>
					This project is about implementing and designin a mobile fitness app. Lorem ipsum dolor
					sit amet consectetur adipisicing elit. Eius natus ex alias deserunt blanditiis facere hic
					culpa, ipsa voluptas pariatur voluptatem quo deleniti? Unde, nihil consequuntur enim earum
					rem molestiae.
				</p>
			</div>

			<div style="margin-top: 1rem;">
				<div>Applicants</div>
				<div>....</div>
			</div>
		</div>
	</div>
</div>

<style>
	.input {
		width: 100%;
		margin: 0 0 .5rem 0;
	}

	.input > input,
	textarea, select {
		margin-top: 0.5rem;
		width: 100%;
	}

	select {
		border-color: rgb(197, 197, 197);
		border-radius: 5px;
		height: 2rem;
		padding: 0 0.3rem;
	}

	.header {
		display: flex;
		justify-content: space-between;
		margin: 1rem 0 1rem 0;
	}

	.outer-container {
		display: flex;
		flex-direction: column;
		margin: auto;
		width: 100%;
	}
	.container {
		max-width: 1000px;
		padding: 0% 5% 0% 5%;
		margin: auto;
	}
	form {
		display: flex;
		flex-direction: column;
		justify-content: center;
		align-items: center;
	}

	textarea {
		resize: none;
	}

	/* 
	*:focus {
		outline: none;
	} */

	input,
	textarea {
		padding: 0;
		padding: 1%;
		/* border: none; */
	}
</style>
