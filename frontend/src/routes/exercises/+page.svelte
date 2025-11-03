<script lang="ts">
	import { onMount } from 'svelte';
	import { createPromiseClient } from '@connectrpc/connect';
	import { Exercises } from '$lib/generated/exercises_connect';
	import { transport } from '$lib/api/client';
	import { ListCategoriesRequest, CreateCategoryRequest, type Category } from '$lib/generated/exercises_pb';
	import { goto } from '$app/navigation';

	const exercisesClient = createPromiseClient(Exercises, transport);

	let categories: Category[] = [];
	let isLoading = true;
	let error = '';
	let showCreateCategory = false;
	let newCategoryName = '';
	let isCreating = false;

	onMount(async () => {
		await loadCategories();
	});

	async function loadCategories() {
		isLoading = true;
		error = '';

		try {
			const request = new ListCategoriesRequest({
				filter: ''
			});

			const response = await exercisesClient.listCategories(request);
			categories = response.categories;
		} catch (e) {
			console.error('Failed to load categories:', e);
			error = `Failed to load categories: ${e}`;
		} finally {
			isLoading = false;
		}
	}

	async function createCategory() {
		if (!newCategoryName.trim()) return;

		isCreating = true;
		error = '';

		try {
			const request = new CreateCategoryRequest({
				name: newCategoryName.trim()
			});

			const response = await exercisesClient.createCategory(request);

			if (response.success) {
				newCategoryName = '';
				showCreateCategory = false;
				await loadCategories();
			} else {
				error = response.message;
			}
		} catch (e) {
			console.error('Failed to create category:', e);
			error = `Failed to create category: ${e}`;
		} finally {
			isCreating = false;
		}
	}

	function viewCategory(categoryId: number) {
		goto(`/exercises/${categoryId}`);
	}
</script>

<main>
	<header>
		<h1>Exercise Categories</h1>
		<div class="header-actions">
			<button class="btn-primary" on:click={() => showCreateCategory = !showCreateCategory}>
				{showCreateCategory ? 'Cancel' : '+ New Category'}
			</button>
			<a href="/dashboard" class="btn-secondary">Back to Dashboard</a>
		</div>
	</header>

	{#if showCreateCategory}
		<section class="create-form">
			<h2>Create New Category</h2>
			<form on:submit|preventDefault={createCategory}>
				<input
					type="text"
					bind:value={newCategoryName}
					placeholder="Category name (e.g., Arms, Legs)"
					required
					disabled={isCreating}
				/>
				<button type="submit" class="btn-primary" disabled={isCreating || !newCategoryName.trim()}>
					{isCreating ? 'Creating...' : 'Create Category'}
				</button>
			</form>
		</section>
	{/if}

	{#if error}
		<div class="error">{error}</div>
	{/if}

	{#if isLoading}
		<div class="loading">Loading categories...</div>
	{:else if categories.length === 0}
		<div class="empty">
			<p>No categories found</p>
			<p>Create your first category to get started!</p>
		</div>
	{:else}
		<div class="categories-grid">
			{#each categories as category}
				<button class="category-card" on:click={() => viewCategory(Number(category.id))}>
					<h3>{category.name}</h3>
					<p class="category-meta">
						{category.createdBy === BigInt(0) ? 'System Category' : 'Custom Category'}
					</p>
					<span class="view-arrow">→</span>
				</button>
			{/each}
		</div>
	{/if}
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
		flex-wrap: wrap;
		gap: 1rem;
	}

	h1 {
		margin: 0;
		font-size: 2rem;
		color: #333;
	}

	.header-actions {
		display: flex;
		gap: 0.5rem;
		flex-wrap: wrap;
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

	.btn-primary:hover:not(:disabled) {
		background: #0056b3;
	}

	.btn-primary:disabled {
		opacity: 0.6;
		cursor: not-allowed;
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

	.create-form {
		margin-bottom: 2rem;
		padding: 1.5rem;
		background: #f8f9fa;
		border-radius: 8px;
		border: 1px solid #ddd;
	}

	.create-form h2 {
		margin: 0 0 1rem 0;
		font-size: 1.3rem;
		color: #333;
	}

	.create-form form {
		display: flex;
		gap: 0.5rem;
		flex-wrap: wrap;
	}

	.create-form input {
		flex: 1;
		min-width: 200px;
		padding: 0.5rem;
		border: 1px solid #ccc;
		border-radius: 4px;
		font-size: 1rem;
	}

	.error {
		margin-bottom: 1rem;
		padding: 1rem;
		background: #f8d7da;
		color: #721c24;
		border: 1px solid #f5c6cb;
		border-radius: 4px;
	}

	.loading, .empty {
		text-align: center;
		padding: 3rem 1rem;
		color: #666;
		font-size: 1.1rem;
	}

	.empty p {
		margin: 0.5rem 0;
	}

	.categories-grid {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(250px, 1fr));
		gap: 1.5rem;
	}

	.category-card {
		position: relative;
		padding: 2rem 1.5rem;
		background: white;
		border: 2px solid #007bff;
		border-radius: 8px;
		cursor: pointer;
		transition: all 0.2s;
		text-align: left;
		width: 100%;
	}

	.category-card:hover {
		background: #f8f9fa;
		transform: translateY(-2px);
		box-shadow: 0 4px 8px rgba(0, 0, 0, 0.1);
	}

	.category-card h3 {
		margin: 0 0 0.5rem 0;
		font-size: 1.5rem;
		color: #333;
	}

	.category-meta {
		margin: 0;
		font-size: 0.85rem;
		color: #666;
	}

	.view-arrow {
		position: absolute;
		top: 1.5rem;
		right: 1.5rem;
		font-size: 1.5rem;
		color: #007bff;
	}
</style>
