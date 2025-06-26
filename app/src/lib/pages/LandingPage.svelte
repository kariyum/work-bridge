<script>
	import Footer from '$lib/components/navbar/Footer.svelte';
	import NavbarNewUser from '$lib/components/navbar/NavbarNewUser.svelte';

	let cards = [
		{
			title: 'Post Projects & Tasks',
			description:
				'Effortlessly create and publish detailed project listings. Define scopes, set budgets, and attach files with ease.'
		},
		{
			title: 'Invite & Collaborate',
			description:
				'Find and invite the perfect freelancers for your team. Collaborate seamlessly with built-in tools.'
		},
		{
			title: 'Manage Proposals',
			description:
				'Review, accept, or decline work proposals with a click. Keep track of all your potential hires.'
		},
		{
			title: 'Real-time Chat',
			description:
				'Communicate instantly with clients and freelancers through our integrated real-time chat.'
		},
		{
			title: 'Skill-Matched Projects',
			description:
				'Freelancers, discover projects perfectly aligned with your expertise and career goals.'
		},
		{
			title: 'Scout Top Talent',
			description:
				'Clients, easily browse and discover a diverse pool of skilled freelancers ready for your next project.'
		}
	];
</script>

<NavbarNewUser></NavbarNewUser>
<div class="bg">
	<div class="can-do">
		<h1>Unleash Your Potential with Bond</h1>
		<ul>
			{#each cards as card}
				<li>
					<h2>{card.title}</h2>
					<p>{card.description}</p>
				</li>
			{/each}
		</ul>
	</div>

	<div class="body">
		<h1>Meet the Bond Team!</h1>
		<p>
			Born from a passion for empowering Tunisian talent, Bond was founded by a team of young
			Tunisian software engineers committed to building high-quality products for our community.
			This is our first app, and we're excited to connect you with the best.
		</p>
	</div>

	<div class="opportunity-section">
		<h1>Ready to Bond with Your Next Opportunity?</h1>
		<p>Join Bond today and experience the future of freelance work. It's free to get started!</p>
		<a href="/register">Sing-up Now</a>
	</div>

	<Footer></Footer>
</div>

<style>
	.bg {
		background-color: var(--darker-bg);
		border-top: 1px solid var(--border);
		margin-top: 20rem;
		padding-top: 2rem;
	}
	.opportunity-section {
		display: flex;
		flex-direction: column;
		gap: 1.5rem;
		padding: 1rem;
		max-width: 80rem;
		margin: 2rem auto;
		text-align: center;

		p {
			padding-top: 1rem;
			font-size: 1.5rem;
			color: var(--sub-title);
		}

		a {
			display: inline;
			background-color: var(--primary);
			width: fit-content;
			margin: auto;
			padding: 1rem;
			text-decoration: none;
			color: inherit;
			border-radius: 5px;
		}
	}
	.body {
		padding: 1rem;
		max-width: 80rem;
		margin: auto;
		margin-top: 2rem;
		text-align: center;

		p {
			padding-top: 2rem;
			font-size: 1.5rem;
			color: var(--sub-title);
		}
	}

	.can-do {
		display: flex;
		flex-direction: column;
		padding: 1rem;
		gap: 1rem;
		font-size: large;
		max-width: 80rem;
		margin: auto;

		h2 {
			text-align: center;
			color: var(--title);
		}

		h1:first-child {
			margin-bottom: 2rem;
			text-align: center;
		}
	}

	ul {
		list-style: none;
		display: grid;
		grid-template-columns: 1fr 1fr 1fr;
		gap: 1.5rem;
	}

	li {
		background-color: var(--card-bg);
		border-radius: 10px;
		padding: 2rem;
		text-align: center;
		position: relative;
		display: flex;
		justify-content: center;
		align-items: center;
		flex-direction: column;
		gap: 2rem;

		h2 {
			color: var(--title);
		}

		p {
			color: var(--sub-title);
		}
	}

	h2 {
		letter-spacing: 0.1ch;
	}

	@media (width < 600px) {
		ul {
			grid-template-columns: 1fr;
		}
	}
</style>
