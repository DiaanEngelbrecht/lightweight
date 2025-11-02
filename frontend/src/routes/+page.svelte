<script lang="ts">
	import { createPromiseClient } from '@connectrpc/connect';
	import { Accounts } from '$lib/generated/accounts_connect';
	import { transport } from '$lib/api/client';
	import { LoginRequest } from '$lib/generated/accounts_pb';
	import { authStore } from '$lib/stores/auth';
	import { goto } from '$app/navigation';
	import { onMount } from 'svelte';

	const accountsClient = createPromiseClient(Accounts, transport);

	let email = '';
	let password = '';
	let message = '';
	let isLoading = false;

	// Redirect to dashboard if already logged in
	onMount(() => {
		if ($authStore.isAuthenticated) {
			goto('/dashboard');
		}
	});

	async function login() {
		isLoading = true;
		message = '';

		try {
			const request = new LoginRequest({
				email,
				password
			});

			const response = await accountsClient.login(request);

			if (response.success && response.authToken) {
				message = `✅ ${response.message}`;

				// Store token and redirect to dashboard
				authStore.login(response.authToken, { email });
				goto('/dashboard');
			} else {
				message = `❌ ${response.message || 'Login failed'}`;
				isLoading = false;
			}
		} catch (error) {
			message = `❌ Error: ${error}`;
			console.error('Failed to login:', error);
			isLoading = false;
		}
	}
</script>

<main>
	<h1>Lightweight Fitness Tracker</h1>

	<section>
		<h2>Login</h2>
		<form on:submit|preventDefault={login}>
			<input type="email" bind:value={email} placeholder="Email" required disabled={isLoading} />
			<input type="password" bind:value={password} placeholder="Password" required disabled={isLoading} />
			<button type="submit" disabled={isLoading}>
				{isLoading ? 'Logging in...' : 'Login'}
			</button>
		</form>

		<p class="link-text">
			Don't have an account? <a href="/signup">Sign up</a>
		</p>
	</section>

	{#if message}
		<div class="message">{message}</div>
	{/if}
</main>

<style>
	main {
		max-width: 600px;
		margin: 2rem auto;
		padding: 0 1rem;
		font-family: system-ui, sans-serif;
	}

	section {
		margin: 2rem 0;
		padding: 1.5rem;
		border: 1px solid #ddd;
		border-radius: 8px;
	}

	form {
		display: flex;
		flex-direction: column;
		gap: 1rem;
	}

	input {
		padding: 0.75rem;
		border: 1px solid #ccc;
		border-radius: 4px;
		font-size: 1rem;
	}

	button {
		padding: 0.75rem 1.5rem;
		background: #007bff;
		color: white;
		border: none;
		border-radius: 4px;
		font-size: 1rem;
		cursor: pointer;
	}

	button:hover:not(:disabled) {
		background: #0056b3;
	}

	button:disabled {
		opacity: 0.6;
		cursor: not-allowed;
	}

	input:disabled {
		opacity: 0.6;
		cursor: not-allowed;
	}

	.message {
		margin-top: 1rem;
		padding: 1rem;
		background: #f8f9fa;
		border-radius: 4px;
	}

	.link-text {
		margin-top: 1rem;
		text-align: center;
		color: #666;
	}

	.link-text a {
		color: #007bff;
		text-decoration: none;
	}

	.link-text a:hover {
		text-decoration: underline;
	}
</style>
