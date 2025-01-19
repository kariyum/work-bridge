<script>
    let pageTitle = "Landing Page";
</script>

<svelte:head>
    <title>{pageTitle}</title>
</svelte:head>

<div class="container">
	<div style="margin:auto; text-align:center; font-size:48px;">
		<h1>Recruit freelancers</h1>
		<h1>& form teams</h1>
	</div>
	<p class="subtitle">Connect with thousands of Tunisian freelancers.</p>
	<div class="buttons">
		<a href="/login">
			<button data-testid="landing-page-login-button">Login</button>
		</a>
		<a href="/register">
			<button data-testid="landing-page-registration-button">Register</button>
		</a>
	</div>
	<!-- <p>
            Please <a href="/login">login</a> or <a href="/register">create</a> an account to continue
        </p> -->
</div>

<style>
	.buttons {
		padding: 4em;
	}
	.buttons > a {
		margin-left: 0.5em;
	}
	.subtitle {
		font-size: 36px;
	}
	.container {
		display: flex;
		justify-content: center;
		align-items: center;
		flex-flow: column;
		padding: 10%;
		position: absolute;
		top: 50%;
		left: 50%;
		width: 80%;
		transform: translate(-50%, -60%);
	}
</style>
