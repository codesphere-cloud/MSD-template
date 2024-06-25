<script>
    async function signUp(event) {
        event.preventDefault();

        const form = event.target;
        const formData = new FormData(form);

        const userName = formData.get('userName');
        const password = formData.get('password');

        const newUser = {
            name: userName,
            password: password
        };

        try {
            const response = await fetch('https://58260-3000.2.codesphere.com/backend/users', {  // change that to the Codesphere URL
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json'
                },
                body: JSON.stringify(newUser)
            });

            if (!response.ok) {
                throw new Error('Failed to create user');
            }

            const result = await response.text();
            console.log(result); // Output response from server

            // Handle success (e.g., show a success message, redirect user)
            alert('User created successfully');
            form.reset(); // Clear form inputs

            // Save logged-in user to local storage
            localStorage.setItem('loggedInUser', userName); // Assuming userName is the identifier

            window.location.href = '/'; // Redirect to main route
        } catch (error) {
            console.error('Error creating user:', error);
            alert('Failed to create user');
        }
    }
</script>

<div>
    <h1>Sign Up</h1>
    <form class="form-container" on:submit|preventDefault={signUp}>
        <label for="userName">User name</label>
        <input type="userName" id="userName" name="userName" required>
        <label for="password">Password</label>
        <input type="password" id="password" name="password" required>
        <button type="submit">Sign Up</button>
    </form>
</div>

<style>
    .form-container {
        display: flex;
        flex-direction: column;
        justify-content: center;
        align-items: center;
        gap: 1rem;
    }
</style>