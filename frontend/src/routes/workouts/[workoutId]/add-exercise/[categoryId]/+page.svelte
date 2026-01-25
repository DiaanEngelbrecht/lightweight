<script lang="ts">
	import { page } from '$app/stores';
	import { onMount } from 'svelte';
	import { createPromiseClient } from '@connectrpc/connect';
	import { Exercises } from '$lib/generated/exercises_connect';
	import { Workouts } from '$lib/generated/workouts_connect';
	import { transport } from '$lib/api/client';
	import { ListExercisesRequest, type Exercise } from '$lib/generated/exercises_pb';
	import { AddExerciseToWorkoutRequest } from '$lib/generated/workouts_pb';
	import { goto } from '$app/navigation';

	const exercisesClient = createPromiseClient(Exercises, transport);
	const workoutsClient = createPromiseClient(Workouts, transport);

	let exercises: Exercise[] = [];
	let isLoading = true;
	let error = '';
	let addingExerciseId: bigint | null = null;

	$: workoutId = $page.params.workoutId;
	$: categoryId = $page.params.categoryId;

	onMount(async () => {
		await loadExercises();
	});

	async function loadExercises() {
		isLoading = true;
		error = '';

		if (!categoryId) {
			error = 'Invalid category ID';
			isLoading = false;
			return;
		}

		try {
			const response = await exercisesClient.listExercises(
				new ListExercisesRequest({ categoryId: BigInt(categoryId) })
			);
			exercises = response.exercises;
		} catch (e) {
			console.error('Failed to load exercises:', e);
			error = `Failed to load exercises: ${e}`;
		} finally {
			isLoading = false;
		}
	}

	async function addExercise(exerciseId: bigint) {
		if (!workoutId) {
			error = 'Invalid workout ID';
			return;
		}

		addingExerciseId = exerciseId;
		error = '';

		try {
			const response = await workoutsClient.addExerciseToWorkout(
				new AddExerciseToWorkoutRequest({
					workoutId: BigInt(workoutId),
					exerciseId,
					notes: ''
				})
			);

			if (response.success) {
				goto(`/workouts/${workoutId}`);
			} else {
				error = response.message;
			}
		} catch (e) {
			console.error('Failed to add exercise:', e);
			error = `Failed to add exercise: ${e}`;
		} finally {
			addingExerciseId = null;
		}
	}
</script>

<main>
	<header>
		<h1>Select Exercise</h1>
		<a href="/workouts/{workoutId}/add-exercise" class="btn-secondary">Back</a>
	</header>

	{#if error}
		<div class="error">{error}</div>
	{/if}

	{#if isLoading}
		<div class="loading">Loading exercises...</div>
	{:else if exercises.length === 0}
		<div class="empty">
			<p>No exercises in this category</p>
			<p>Add exercises in the Exercise Management section first.</p>
			<a href="/exercises/{categoryId}" class="btn-primary">Add Exercises</a>
		</div>
	{:else}
		<div class="exercises-list">
			{#each exercises as exercise}
				<button
					class="exercise-card"
					on:click={() => addExercise(exercise.id)}
					disabled={addingExerciseId !== null}
				>
					<span class="exercise-name">{exercise.name}</span>
					{#if addingExerciseId === exercise.id}
						<span class="adding">Adding...</span>
					{:else}
						<span class="add-icon">+</span>
					{/if}
				</button>
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

	.exercises-list {
		display: flex;
		flex-direction: column;
		gap: 0.75rem;
	}

	.exercise-card {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 1rem 1.5rem;
		background: white;
		border: 2px solid #28a745;
		border-radius: 8px;
		cursor: pointer;
		transition: all 0.2s;
		text-align: left;
		width: 100%;
	}

	.exercise-card:hover:not(:disabled) {
		background: #f8f9fa;
		transform: translateX(4px);
	}

	.exercise-card:disabled {
		opacity: 0.6;
		cursor: not-allowed;
	}

	.exercise-name {
		font-size: 1.1rem;
		color: #333;
	}

	.add-icon {
		font-size: 1.5rem;
		color: #28a745;
		font-weight: bold;
	}

	.adding {
		font-size: 0.9rem;
		color: #666;
	}
</style>
