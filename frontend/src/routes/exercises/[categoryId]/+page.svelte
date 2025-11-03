<script lang="ts">
	import { onMount } from 'svelte';
	import { page } from '$app/stores';
	import { goto } from '$app/navigation';
	import { createPromiseClient } from '@connectrpc/connect';
	import { Exercises } from '$lib/generated/exercises_connect';
	import { transport } from '$lib/api/client';
	import {
		ListExercisesRequest,
		CreateExerciseRequest,
		UpdateExerciseRequest,
		DeleteExerciseRequest,
		type Exercise
	} from '$lib/generated/exercises_pb';

	const exercisesClient = createPromiseClient(Exercises, transport);

	$: categoryId = BigInt($page.params.categoryId || '0');

	let exercises: Exercise[] = [];
	let isLoading = true;
	let error = '';
	let showCreateExercise = false;
	let newExerciseName = '';
	let isCreating = false;

	// Edit state
	let editingExerciseId: bigint | null = null;
	let editExerciseName = '';
	let isUpdating = false;

	// Delete state
	let deletingExerciseId: bigint | null = null;
	let isDeleting = false;

	onMount(async () => {
		await loadExercises();
	});

	$: if ($page.params.categoryId) {
		loadExercises();
	}

	async function loadExercises() {
		isLoading = true;
		error = '';

		try {
			const request = new ListExercisesRequest({
				categoryId
			});

			const response = await exercisesClient.listExercises(request);
			exercises = response.exercises;
		} catch (e) {
			console.error('Failed to load exercises:', e);
			error = `Failed to load exercises: ${e}`;
		} finally {
			isLoading = false;
		}
	}

	async function createExercise() {
		if (!newExerciseName.trim()) return;

		isCreating = true;
		error = '';

		try {
			const request = new CreateExerciseRequest({
				name: newExerciseName.trim(),
				categoryId
			});

			const response = await exercisesClient.createExercise(request);

			if (response.success) {
				newExerciseName = '';
				showCreateExercise = false;
				await loadExercises();
			} else {
				error = response.message;
			}
		} catch (e) {
			console.error('Failed to create exercise:', e);
			error = `Failed to create exercise: ${e}`;
		} finally {
			isCreating = false;
		}
	}

	function startEdit(exercise: Exercise) {
		editingExerciseId = exercise.id;
		editExerciseName = exercise.name;
	}

	function cancelEdit() {
		editingExerciseId = null;
		editExerciseName = '';
	}

	async function saveEdit(exerciseId: bigint) {
		if (!editExerciseName.trim()) return;

		isUpdating = true;
		error = '';

		try {
			const request = new UpdateExerciseRequest({
				exerciseId,
				name: editExerciseName.trim()
			});

			const response = await exercisesClient.updateExercise(request);

			if (response.success) {
				editingExerciseId = null;
				editExerciseName = '';
				await loadExercises();
			} else {
				error = response.message;
			}
		} catch (e) {
			console.error('Failed to update exercise:', e);
			error = `Failed to update exercise: ${e}`;
		} finally {
			isUpdating = false;
		}
	}

	function confirmDelete(exerciseId: bigint) {
		deletingExerciseId = exerciseId;
	}

	function cancelDelete() {
		deletingExerciseId = null;
	}

	async function deleteExercise(exerciseId: bigint) {
		isDeleting = true;
		error = '';

		try {
			const request = new DeleteExerciseRequest({
				exerciseId
			});

			const response = await exercisesClient.deleteExercise(request);

			if (response.success) {
				deletingExerciseId = null;
				await loadExercises();
			} else {
				error = response.message;
			}
		} catch (e) {
			console.error('Failed to delete exercise:', e);
			error = `Failed to delete exercise: ${e}`;
		} finally {
			isDeleting = false;
		}
	}
</script>

<main>
	<header>
		<h1>Exercises</h1>
		<div class="header-actions">
			<button class="btn-primary" on:click={() => showCreateExercise = !showCreateExercise}>
				{showCreateExercise ? 'Cancel' : '+ New Exercise'}
			</button>
			<a href="/exercises" class="btn-secondary">Back to Categories</a>
		</div>
	</header>

	{#if showCreateExercise}
		<section class="create-form">
			<h2>Create New Exercise</h2>
			<form on:submit|preventDefault={createExercise}>
				<input
					type="text"
					bind:value={newExerciseName}
					placeholder="Exercise name (e.g., Bench Press, Squats)"
					required
					disabled={isCreating}
				/>
				<button type="submit" class="btn-primary" disabled={isCreating || !newExerciseName.trim()}>
					{isCreating ? 'Creating...' : 'Create Exercise'}
				</button>
			</form>
		</section>
	{/if}

	{#if error}
		<div class="error">{error}</div>
	{/if}

	{#if isLoading}
		<div class="loading">Loading exercises...</div>
	{:else if exercises.length === 0}
		<div class="empty">
			<p>No exercises found in this category</p>
			<p>Create your first exercise to get started!</p>
		</div>
	{:else}
		<div class="exercises-list">
			{#each exercises as exercise}
				<div class="exercise-card">
					{#if editingExerciseId === exercise.id}
						<div class="exercise-edit">
							<input
								type="text"
								bind:value={editExerciseName}
								disabled={isUpdating}
								class="edit-input"
							/>
							<div class="edit-actions">
								<button
									class="btn-save"
									on:click={() => saveEdit(exercise.id)}
									disabled={isUpdating || !editExerciseName.trim()}
								>
									{isUpdating ? 'Saving...' : 'Save'}
								</button>
								<button class="btn-cancel" on:click={cancelEdit} disabled={isUpdating}>
									Cancel
								</button>
							</div>
						</div>
					{:else}
						<div class="exercise-info">
							<h3>{exercise.name}</h3>
							<p class="exercise-meta">
								{exercise.createdBy === BigInt(0) ? 'System Exercise' : 'Custom Exercise'}
							</p>
						</div>
						<div class="exercise-actions">
							<button class="btn-edit" on:click={() => startEdit(exercise)}>
								Edit
							</button>
							<button class="btn-delete" on:click={() => confirmDelete(exercise.id)}>
								Delete
							</button>
						</div>
					{/if}
				</div>

				{#if deletingExerciseId === exercise.id}
					<div class="delete-confirm">
						<p>Are you sure you want to delete "{exercise.name}"?</p>
						<div class="confirm-actions">
							<button
								class="btn-confirm-delete"
								on:click={() => deleteExercise(exercise.id)}
								disabled={isDeleting}
							>
								{isDeleting ? 'Deleting...' : 'Yes, Delete'}
							</button>
							<button class="btn-cancel" on:click={cancelDelete} disabled={isDeleting}>
								Cancel
							</button>
						</div>
					</div>
				{/if}
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

	.exercises-list {
		display: flex;
		flex-direction: column;
		gap: 1rem;
	}

	.exercise-card {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 1.5rem;
		background: white;
		border: 1px solid #ddd;
		border-radius: 8px;
		transition: box-shadow 0.2s;
	}

	.exercise-card:hover {
		box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
	}

	.exercise-info h3 {
		margin: 0 0 0.25rem 0;
		font-size: 1.3rem;
		color: #333;
	}

	.exercise-meta {
		margin: 0;
		font-size: 0.85rem;
		color: #666;
	}

	.exercise-actions {
		display: flex;
		gap: 0.5rem;
	}

	.btn-edit {
		padding: 0.4rem 0.8rem;
		background: #28a745;
		color: white;
		border: none;
		border-radius: 4px;
		font-size: 0.85rem;
		cursor: pointer;
		transition: background 0.2s;
	}

	.btn-edit:hover {
		background: #218838;
	}

	.btn-delete {
		padding: 0.4rem 0.8rem;
		background: #dc3545;
		color: white;
		border: none;
		border-radius: 4px;
		font-size: 0.85rem;
		cursor: pointer;
		transition: background 0.2s;
	}

	.btn-delete:hover {
		background: #c82333;
	}

	.exercise-edit {
		display: flex;
		align-items: center;
		gap: 1rem;
		flex: 1;
	}

	.edit-input {
		flex: 1;
		padding: 0.5rem;
		border: 1px solid #007bff;
		border-radius: 4px;
		font-size: 1rem;
	}

	.edit-actions {
		display: flex;
		gap: 0.5rem;
	}

	.btn-save {
		padding: 0.4rem 0.8rem;
		background: #007bff;
		color: white;
		border: none;
		border-radius: 4px;
		font-size: 0.85rem;
		cursor: pointer;
		transition: background 0.2s;
	}

	.btn-save:hover:not(:disabled) {
		background: #0056b3;
	}

	.btn-save:disabled {
		opacity: 0.6;
		cursor: not-allowed;
	}

	.btn-cancel {
		padding: 0.4rem 0.8rem;
		background: #6c757d;
		color: white;
		border: none;
		border-radius: 4px;
		font-size: 0.85rem;
		cursor: pointer;
		transition: background 0.2s;
	}

	.btn-cancel:hover:not(:disabled) {
		background: #5a6268;
	}

	.btn-cancel:disabled {
		opacity: 0.6;
		cursor: not-allowed;
	}

	.delete-confirm {
		margin-top: -0.5rem;
		margin-bottom: 0.5rem;
		padding: 1rem;
		background: #fff3cd;
		border: 1px solid #ffc107;
		border-radius: 4px;
	}

	.delete-confirm p {
		margin: 0 0 0.5rem 0;
		color: #856404;
		font-weight: 500;
	}

	.confirm-actions {
		display: flex;
		gap: 0.5rem;
	}

	.btn-confirm-delete {
		padding: 0.4rem 0.8rem;
		background: #dc3545;
		color: white;
		border: none;
		border-radius: 4px;
		font-size: 0.85rem;
		cursor: pointer;
		transition: background 0.2s;
	}

	.btn-confirm-delete:hover:not(:disabled) {
		background: #c82333;
	}

	.btn-confirm-delete:disabled {
		opacity: 0.6;
		cursor: not-allowed;
	}
</style>
