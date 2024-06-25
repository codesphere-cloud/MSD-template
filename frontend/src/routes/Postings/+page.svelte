<script>
	import { onMount } from 'svelte';
	import { createEventDispatcher } from 'svelte';
  
	let loggedInUserName;
	let loggedInUserId;
	
	let tweets = [];
	let newTweet = {
		  userId: loggedInUserId,
		  title: '',
		  likes: 0,
		  dislikes: 0,
		  text: ''
		};
	
	const dispatch = createEventDispatcher();
  
	// Funktion zum Abrufen der Tweets von der API
	async function fetchTweets() {
	  const response = await fetch('https://58260-3000.2.codesphere.com/backend/tweets');
	  if (response.ok) {
		tweets = await response.json();
	  } else {
		console.error('Error fetching tweets:', response.statusText);
	  }
	}
  
	// Funktion zum Erstellen eines neuen Tweets
	async function createTweet() {
	  newTweet.userId = loggedInUserId;  // Setzt die Benutzer-ID vor dem Senden des Tweets
  
	  const response = await fetch('https://58260-3000.2.codesphere.com/backend/tweets', {
		method: 'POST',
		headers: {
		  'Content-Type': 'application/json'
		},
		body: JSON.stringify(newTweet)
	  });
  
	  if (response.ok) {
		// Tweet erfolgreich erstellt, Liste der Tweets aktualisieren
		await fetchTweets();
		// Formulardaten zurücksetzen
		newTweet = {
		  userId: loggedInUserId,
		  title: '',
		  likes: 0,
		  dislikes: 0,
		  text: ''
		};
		dispatch('tweetCreated');
	  } else {
		console.error('Error creating tweet:', response.statusText);
	  }
	}
  
	// Ruft die Funktion beim Laden der Komponente auf
	onMount(fetchTweets);
	onMount(async () => {
	  // Überprüfe, ob ein Benutzer im Local Storage gespeichert ist
	  loggedInUserName = await localStorage.getItem('loggedInUserName');
	  loggedInUserId = await parseInt(localStorage.getItem('loggedInUserId'));
	  newTweet.userId = loggedInUserId;  // Setzt die Benutzer-ID beim Laden der Komponente
	});
  </script>
  
  <svelte:head>
	<title>Tweet List</title>
	<meta name="description" content="A list of tweets" />
  </svelte:head>
  
  <h1>List of Tweets</h1>
  
  <!-- Formular zum Erstellen eines neuen Tweets -->
  <form on:submit|preventDefault={createTweet}>
	<label>
	  Title:
	  <input type="text" bind:value={newTweet.title} required />
	</label>
	<label>
	  Text:
	  <textarea bind:value={newTweet.text} required></textarea>
	</label>
	<button type="submit">Create Tweet</button>
  </form>
  
  {#if tweets.length === 0}
	<p>No tweets available.</p>
  {:else}
	<ul>
	  {#each tweets as tweet}
		<li>
		  <h2>{tweet.title}</h2>
		  <p>{tweet.text}</p>
		  <p>Likes: {tweet.likes ?? 0} | Dislikes: {tweet.dislikes ?? 0}</p>
		  <p>{tweet.name}</p>
		</li>
	  {/each}
	</ul>
  {/if}
  
  <style>
	/* Fügen Sie hier Ihre CSS-Stile hinzu */
  </style>
  