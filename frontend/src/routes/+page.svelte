<script>
	import welcome from '$lib/images/svelte-welcome.webp';
	import welcome_fallback from '$lib/images/svelte-welcome.png';
	import { onMount } from 'svelte';

	$: userName = '';

	onMount(async () => {
		// Überprüfe, ob ein Benutzer im Local Storage gespeichert ist
		const loggedInUserName = await localStorage.getItem('loggedInUserName');
		const loggedInUserId = await localStorage.getItem('loggedInUserId');
		if (loggedInUserName && loggedInUserId) {
			userName = loggedInUserName;
			userId = loggedInUserId;
		} else {
			userLoggedIn = false;
			// Umleitung zur Login-Seite, wenn kein Benutzer angemeldet ist
			if (window.location.pathname !== '/login') {
				window.location.href = '/login';
				}
		}
	});

</script>

<svelte:head>
	<title>Home</title>
	<meta name="description" content="Svelte demo app" />
</svelte:head>

<section>
	<h1>
		<span class="welcome">
			<picture>
				<source srcset={welcome} type="image/webp" />
				<img src={welcome_fallback} alt="Welcome" />
			</picture>
		</span>
		<h1>{userName}</h1>

		Welcome to a Multi Server Deployment with Codesphere!
	</h1>

	<h2>
		This is the frontend Component interacting with the backend component. <br>
		Feel free to create a post! 

		Also, if you want to learn Svelte or Rust you can adopt this template as a project. <br>
		Just fork the repository and start coding!
	</h2>

</section>

<style>
	section {
		display: flex;
		flex-direction: column;
		justify-content: center;
		align-items: center;
		flex: 0.6;
	}

	h1 {
		width: 100%;
	}

	.welcome {
		display: block;
		position: relative;
		width: 100%;
		height: 0;
		padding: 0 0 calc(100% * 495 / 2048) 0;
	}

	.welcome img {
		position: absolute;
		width: 100%;
		height: 100%;
		top: 0;
		display: block;
	}
</style>
