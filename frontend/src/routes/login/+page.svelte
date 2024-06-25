<script>
    import LogIn from './LogIn.svelte';
    import SignUp from './SignUp.svelte';


</script>

<div class="container">
    <LogIn />
    <strong class="or-text">or</strong>
    <SignUp />
</div>

<style>
    /* your styles go here */
    .container {
        display: flex;
        flex-direction: row;
        justify-content: center;
        align-items: center;
        height: 100vh;
    }

    .or-text {
        margin: 0 5rem;
        font-size: 2rem;
    }
</style>