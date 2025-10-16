<script lang="ts">
	import { createPromiseClient } from '@bufbuild/connect-web';
	import { Accounts } from '$lib/generated/accounts_connect';
	import { Exercises } from '$lib/generated/exercises_connect';
	import { transport } from '$lib/api/client';
	import type { CreateAccountRequest } from '$lib/generated/accounts_pb';

	// Create typed clients from generated code
	const accountsClient = createPromiseClient(Accounts, transport);
	const exercisesClient = createPromiseClient(Exercises, transport);

	let name = '';
	let email = '';
	let password = '';
	let message = '';

	async function createAccount() {
		try {
			const request: CreateAccountRequest = {
				name,
				email,
				password
			};

			const response = await accountsClient.createAccount(request);

			if (response.success) {
				message = `✅ ${response.message}`;
			} else {
				message = `❌ ${response.message}`;
			}
		} catch (error) {
			message = `❌ Error: ${error}`;
			console.error('Failed to create account:', error);
		}
	}

	async function fetchCategories() {
		try {
			const response = await exercisesClient.listCategories({});
			console.log('Categories:', response.categories);
			message = `✅ Found ${response.categories.length} categories`;
		} catch (error) {
			message = `❌ Error: ${error}`;
			console.error('Failed to fetch categories:', error);
		}
	}
</script>

<main>
	<h1>Lightweight Fitness Tracker</h1>

	<section>
		<h2>Create Account</h2>
		<form on:submit|preventDefault={createAccount}>
			<input type="text" bind:value={name} placeholder="Name" required />
			<input type="email" bind:value={email} placeholder="Email" required />
			<input type="password" bind:value={password} placeholder="Password" required />
			<button type="submit">Create Account</button>
		</form>
	</section>

	<section>
		<h2>Exercise Categories</h2>
		<button on:click={fetchCategories}>Load Categories</button>
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

	button:hover {
		background: #0056b3;
	}

	.message {
		margin-top: 1rem;
		padding: 1rem;
		background: #f8f9fa;
		border-radius: 4px;
	}
</style>
