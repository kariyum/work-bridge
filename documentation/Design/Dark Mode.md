
```
<script>
	document.documentElement.classList.add(
		JSON.parse(localStorage.getItem('svelte:theme'))?.current ??
			(window.matchMedia('(prefers-color-scheme: dark)').matches ? 'dark' : 'light')
	);
	document.documentElement.classList.add(
		`font-${localStorage.getItem('svelte:font') ?? 'elegant'}`
	);
</script>
```
