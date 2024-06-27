<script>
	import welcome from '$lib/images/codesphere-welcome.png';
	import welcome_fallback from '$lib/images/codesphere-welcome.png';
	import { onMount } from 'svelte';

	$: userName = '';
	let userId = '';

	onMount(async () => {
		// Überprüfe, ob ein Benutzer im Local Storage gespeichert ist
		const loggedInUserName = await localStorage.getItem('loggedInUserName');
		const loggedInUserId = await parseInt(localStorage.getItem('loggedInUserId'));
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
		<div class="welcome">
			
			<img src={welcome_fallback} alt="Welcome" />
			
		</div>

		<h1>{userName}</h1>

		Welcome to a Landscape Deployment with Codesphere!
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
		position: relative;
		width: 100%;
	}

	.welcome {
		position: absolute;
		display: flex;
		justify-content: center;
		align-items: center;
		height: 0;
		padding: 0 0 calc(100% * 495 / 2048) 0;
	}

	.welcome img {
		position: relative;
		top: 0;
		display: block;
		width: 65%;
		height: auto;
		justify-self: center;
		align-self: center;
	}
</style>
