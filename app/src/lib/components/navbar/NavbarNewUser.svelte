<script>
	import ThemeToggler from '$lib/components/utility/ThemeToggler.svelte';
</script>

<div>
	<div class="navbar">
		<div class="left-group">
			<div class="logo"></div>
		</div>

		<div class="right-group">
			<a href="/projects">View Projects</a>
			<div>
				<ThemeToggler></ThemeToggler>
			</div>
			<a href="/login">
				<button class="primary-btn">Login</button>
			</a>
		</div>
	</div>

	<div class="container">
		<div>
			<h1 class="title">
				<p>Recruit Top Tunisian Freelancers</p>
				<p>& Build Your Dream Team</p>
			</h1>
			<p class="subtitle">Effortlessly connect with skilled local talent!</p>
		</div>
		<div class="register-btn">
			<a href="/register">
				<button class="primary-btn">Get Started - It's Free!</button>
			</a>
		</div>
	</div>
</div>

<style>
	.title {
		color: var(--title);
	}
	.subtitle {
		margin: 2rem 0;
		color: var(--sub-title);
	}
	a {
		text-decoration: none;
		color: inherit;
	}
	.navbar {
		display: flex;
		flex-direction: row nowrap;
		justify-content: space-between;
		align-items: center;
		padding: 1rem 1rem 0 1rem;
		> * {
			flex-grow: 1;
		}

		.left-group,
		.right-group {
			display: flex;
			flex-direction: row;
			gap: 1rem;
			align-items: center;
		}

		.right-group {
			justify-content: end;
		}
	}

	.logo {
		background-image: url('/logo_dark.svg');
		height: 1.2rem;
		aspect-ratio: 50/12;
		background-size: cover;
		margin-left: 0.7rem;

		:global(.dark) & {
			background-image: url('/logo_white.svg');
		}
	}
	.container {
		margin-top: 8rem;
		font-size: x-large;
		display: flex;
		flex-direction: column;
		padding: 2rem 1rem 0 1rem;
		gap: 1rem;
		align-items: center;
		text-align: center;
	}
</style>
