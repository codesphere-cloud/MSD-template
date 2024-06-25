<script>
    let userName = '';
    let password = ''; 

    

</script>

<div>
    <h1>Log In</h1>
    <form class="form-container">
        <label for="userName">User name</label>
        <input type="userName" id="userName" name="userName" required>
        <label for="password">Password</label>
        <input type="password" id="password" name="password" required>
        <button type="submit">Log In</button>
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