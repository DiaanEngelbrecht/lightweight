<script lang="ts">
	import { page } from '$app/stores';
	import { onMount } from 'svelte';
	import { createPromiseClient } from '@connectrpc/connect';
	import { Exercises } from '$lib/generated/exercises_connect';
	import { transport } from '$lib/api/client';
	import { ListCategoriesRequest, type Category } from '$lib/generated/exercises_pb';

	const exercisesClient = createPromiseClient(Exercises, transport);

	let categories: Category[] = [];
	let isLoading = true;
	let error = '';

	$: workoutId = $page.params.workoutId;

	onMount(async () => {
		await loadCategories();
	});

	async function loadCategories() {
		isLoading = true;
		error = '';

		try {
			const response = await exercisesClient.listCategories(
				new ListCategoriesRequest({ filter: '' })
			);
			categories = response.categories;
		} catch (e) {
			console.error('Failed to load categories:', e);
			error = `Failed to load categories: ${e}`;
		} finally {
			isLoading = false;
		}
	}
</script>

<main>
	<header>
		<h1>Select Category</h1>
		<a href="/workouts/{workoutId}" class="btn-secondary">Cancel</a>
	</header>

	{#if error}
		<div class="error">{error}</div>
	{/if}

	{#if isLoading}
		<div class="loading">Loading categories...</div>
	{:else if categories.length === 0}
		<div class="empty">
			<p>No categories found</p>
			<p>Create categories in the Exercise Management section first.</p>
			<a href="/exercises" class="btn-primary">Manage Exercises</a>
		</div>
	{:else}
		<div class="categories-grid">
			{#each categories as category}
				<a href="/workouts/{workoutId}/add-exercise/{category.id}" class="category-card">
					<h2>{category.name}</h2>
					<span class="arrow">→</span>
				</a>
			{/each}
		</div>
	{/if}
</main>

<style>
	main {
		max-width: 800px;
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
		font-size: 1.8rem;
		color: #333;
	}

	.btn-secondary {
		padding: 0.5rem 1rem;
		background: #6c757d;
		color: white;
		border: none;
		border-radius: 4px;
		font-size: 0.9rem;
		cursor: pointer;
		transition: background 0.2s;
		text-decoration: none;
		display: inline-block;
	}

	.btn-secondary:hover {
		background: #5a6268;
	}

	.btn-primary {
		padding: 0.5rem 1rem;
		background: #007bff;
		color: white;
		border: none;
		border-radius: 4px;
		font-size: 0.9rem;
		cursor: pointer;
		transition: background 0.2s;
		text-decoration: none;
		display: inline-block;
	}

	.btn-primary:hover {
		background: #0056b3;
	}

	.error {
		margin-bottom: 1rem;
		padding: 1rem;
		background: #f8d7da;
		color: #721c24;
		border: 1px solid #f5c6cb;
		border-radius: 4px;
	}

	.loading,
	.empty {
		text-align: center;
		padding: 3rem 1rem;
		color: #666;
		font-size: 1.1rem;
	}

	.empty p {
		margin: 0.5rem 0;
	}

	.empty .btn-primary {
		margin-top: 1rem;
	}

	.categories-grid {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(200px, 1fr));
		gap: 1rem;
	}

	.category-card {
		position: relative;
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 1.5rem;
		background: white;
		border: 2px solid #007bff;
		border-radius: 8px;
		text-decoration: none;
		color: #333;
		transition: all 0.2s;
	}

	.category-card:hover {
		background: #f8f9fa;
		transform: translateY(-2px);
		box-shadow: 0 4px 8px rgba(0, 0, 0, 0.1);
	}

	.category-card h2 {
		margin: 0;
		font-size: 1.2rem;
	}

	.arrow {
		font-size: 1.5rem;
		color: #007bff;
	}
</style>
