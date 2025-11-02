<script lang="ts">
	import { authStore } from '$lib/stores/auth';

	function handleLogout() {
		authStore.logout();
	}
</script>

<main>
	<header>
		<h1>Dashboard</h1>
		<button class="logout-btn" on:click={handleLogout}>Logout</button>
	</header>

	<section class="welcome">
		<h2>Hello{$authStore.user?.name ? `, ${$authStore.user.name}` : ''}!</h2>
		<p>Welcome to your fitness tracker.</p>
	</section>
</main>

<style>
	main {
		max-width: 1200px;
		margin: 2rem auto;
		padding: 0 1rem;
		font-family: system-ui, sans-serif;
	}

	header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 2rem;
		padding-bottom: 1rem;
		border-bottom: 2px solid #ddd;
	}

	h1 {
		margin: 0;
		font-size: 2rem;
		color: #333;
	}

	.logout-btn {
		padding: 0.5rem 1rem;
		background: #dc3545;
		color: white;
		border: none;
		border-radius: 4px;
		font-size: 0.9rem;
		cursor: pointer;
		transition: background 0.2s;
	}

	.logout-btn:hover {
		background: #c82333;
	}

	.welcome {
		padding: 2rem;
		background: #f8f9fa;
		border-radius: 8px;
		text-align: center;
	}

	.welcome h2 {
		margin: 0 0 1rem 0;
		font-size: 1.8rem;
		color: #333;
	}

	.welcome p {
		margin: 0;
		font-size: 1.1rem;
		color: #666;
	}
</style>
