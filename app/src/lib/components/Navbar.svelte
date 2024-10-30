<script lang="ts">
	import { goto } from "$app/navigation";
	import { userStore } from "$lib/storage";
    let { user }: {user: User} = $props();
    async function logout() {
        const response = await fetch('/api/logout');

        if (response.status === 200) {
            userStore.set(undefined);
            await goto('/');
        }
    }
</script>

<section>
    <div class="container">
        <h1><a href="/">Word-bridge</a></h1>
        <nav>
            <ul>
                <!-- <li><a href="/"><span class="material-symbols-outlined">home</span></a></li> -->
                <li><a href="/profile">(Avatar) {user.email}</a></li>
                <li><a href="/project">Create a project</a></li>
                <li><a href="/messages">Discussions</a></li>
                <li><a href="/notifications">Notifications</a></li>
                <li><a href="/settings">Settings</a></li>
                <li>
                    <button onclick={logout}>Logout</button>
                </li>
                <!-- <li><a href="/"><span class="material-symbols-outlined"> person </span></a></li> -->
            </ul>
        </nav>
    </div>
</section>

<style>
	.material-symbols-outlined {
		font-variation-settings:
			'FILL' 1,
			'wght' 400,
			'GRAD' 0,
			'opsz' 24;
	}
	nav {
        width: 40%;
        margin-left: auto;
	}
    .container {
        /* border: 2px solid black; */
        display: flex;
        flex-direction: row nowrap;
        align-items: safe center;
        height: max-content;
        margin: 1rem;
    }
	ul {
		display: flex;
		justify-content: space-around;
        align-items: safe center;
		list-style: none;
		padding: 0;
		margin: auto;
	}

    li, button {
        height: fit-content;
    }

    button {
        padding: 15%;
    }

</style>
