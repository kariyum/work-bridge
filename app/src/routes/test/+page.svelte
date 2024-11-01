<script>
	function fetchProposals() {
		fetch('/api/proposals')
			.then((response) => response.json())
			.then((data) => {
				console.log(data);
			});
	}
</script>

<button onclick={fetchProposals}>
	Fetch Proposals
</button>