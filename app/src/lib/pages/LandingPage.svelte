<script>
	import Footer from '$lib/components/navbar/Footer.svelte';
	import NavbarNewUser from '$lib/components/navbar/NavbarNewUser.svelte';
</script>

<NavbarNewUser></NavbarNewUser>
<div class="can-do">
	<h2>Features</h2>
	<ul>
		<li>Post projects and tasks</li>
		<li>Invite & Work with Freelancers</li>
		<li>Accept & decline work propoals</li>
		<li>Real time communication</li>
		<li>Get relevant projects to your skills</li>
		<li>Scout for freelancers</li>
	</ul>
</div>

<div class="body">
	<h1>Meet the Bond Team!</h1>
	<p>
		Born from a passion for empowering Tunisian talent, Bond was founded by a team of young Tunisian
		software engineers committed to building high-quality products for our community. This is our
		first app, and we're excited to connect you with the best.
	</p>
</div>
<Footer></Footer>

<style>
	.body {
		padding: 1rem;
		max-width: 50rem;
		margin: auto;
		margin-top: 2rem;
	}

	.can-do {
		display: flex;
		flex-direction: column;
		padding: 1rem;
		gap: 0.5rem;
		font-size: large;
		max-width: 50rem;
		margin: auto;
		margin-top: 3rem;
	}

	ul {
		list-style: none;
		display: grid;
		grid-template-columns: 1fr 1fr 1fr;
		gap: 1rem;
	}

	li {
		background-color: var(--card-bg);
		border-radius: 10px;
		padding: 1rem;
		text-align: center;
		position: relative;
		display: flex;
		justify-content: center;
		align-items: center;
	}

	h2 {
		letter-spacing: 0.1ch;
	}

	@media (width < 600px) {
		ul {
			grid-template-columns: 1fr 1fr;
		}
	}
</style>
