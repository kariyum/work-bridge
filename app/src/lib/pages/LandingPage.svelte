<div class="logo">
	<svg width="100" height="50" viewBox="0 0 100 50" fill="none" xmlns="http://www.w3.org/2000/svg">
		<circle cx="15" cy="25" r="5" class="dot" />
		<text x="30" y="25" dominant-baseline="middle" class="main-text">bond</text>
	</svg>
</div>
<div class="container">
	<h1 class="title">
		<p>Recruit freelancers</p>
		<p>& form teams</p>
	</h1>
	<p class="subtitle">Connect with Tunisian freelancers.</p>
	<div class="buttons">
		<a href="/register">
			<button>Join us!</button>
		</a>
	</div>
</div>

<style>
	.main-text {
		font-family: 'Poppins', sans-serif;
		font-weight: 600;
		font-size: x-large;
		fill: var(--font-color);
		pointer-events: none;
	}
	.dot {
		fill: var(--blue);
	}
	.logo {
		display: flex;
		gap: 0.5rem;
		font-size: x-large;
		font-weight: 500;
		align-items: center;
		margin: 1rem;
	}
	.title {
		p {
			text-align: center;
		}
	}
	.subtitle {
		text-align: center;
	}
	.buttons {
		text-align: center;
		margin-top: 2rem;
	}
	.container {
		font-size: x-large;
		display: flex;
		flex-direction: column;
		padding: 1rem 1rem 0 1rem;
		position: absolute;
		top: 50%;
		left: 50%;
		transform: translate(-50%, -50%);
	}
</style>
