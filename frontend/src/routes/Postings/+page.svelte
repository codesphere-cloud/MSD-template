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
	  console.log(tweets);
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

	// Funktion zum Liken eines Tweets
    async function likeTweet(tweetId) {
        const response = await fetch(`https://58260-3000.2.codesphere.com/backend/tweets/${tweetId}/like`, {
            method: 'PUT',
        });

        if (response.ok) {
            // Wenn erfolgreich, aktualisiere die Tweets
            await fetchTweets();
        } else {
            console.error('Error liking tweet:', response.statusText);
        }
    }

    // Funktion zum Disliken eines Tweets
    async function dislikeTweet(tweetId) {
        const response = await fetch(`https://58260-3000.2.codesphere.com/backend/tweets/${tweetId}/dislike`, {
            method: 'PUT',
        });

        if (response.ok) {
            // Wenn erfolgreich, aktualisiere die Tweets
            await fetchTweets();
        } else {
            console.error('Error disliking tweet:', response.statusText);
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
			<div class="tweetBox">
				<strong>Title: {tweet.title}</strong>
				<p>{tweet.text}</p>
				<div class="info-section-tweet">
					<div class="likeContainer">
						<button on:click={() => likeTweet(tweet.id)}>Like</button>
						<p class="likeButton">Likes: {tweet.likes ?? 0}</p>
						<button on:click={() => dislikeTweet(tweet.id)}>Dislike</button>
						<p class="likeButton">Dislikes: {tweet.dislikes ?? 0}</p>
					</div>
					<p>OP: {tweet.user_name}</p>
				</div>
			</div>
		{/each}
	</ul>
  {/if}
  
  <style>
	/* Fügen Sie hier Ihre CSS-Stile hinzu */
	.tweetBox {
	  border: 1px solid #ccc;
	  padding: 10px;
	  margin: 10px 0;
	}

	.info-section-tweet{
		display: flex;
		justify-content: space-between;
		flex-direction: row;
	}

	.likeContainer {
		display: flex;
		flex-direction: row;
		gap: 10px;
	
	}

	.likeButton {
		cursor: pointer;
		border-radius: 10px;
		padding: 5px;
		background-color: #f0f0f0;
	}


  </style>
  