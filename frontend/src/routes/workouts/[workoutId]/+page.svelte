<script lang="ts">
	import { page } from '$app/stores';
	import { onMount } from 'svelte';
	import { createPromiseClient } from '@connectrpc/connect';
	import { Workouts } from '$lib/generated/workouts_connect';
	import { transport } from '$lib/api/client';
	import {
		GetWorkoutRequest,
		CreateSetRequest,
		UpdateSetRequest,
		DeleteSetRequest,
		RemoveExerciseFromWorkoutRequest,
		type Workout,
		type WorkoutExercise,
		type ExerciseSet
	} from '$lib/generated/workouts_pb';
	import { goto } from '$app/navigation';

	const workoutsClient = createPromiseClient(Workouts, transport);

	let workout: Workout | null = null;
	let isLoading = true;
	let error = '';

	// Set form state
	let activeExerciseId: bigint | null = null;
	let newSetReps = '';
	let newSetWeight = '';
	let isAddingSet = false;

	// Edit set state
	let editingSetId: bigint | null = null;
	let editSetReps = '';
	let editSetWeight = '';
	let isUpdatingSet = false;

	$: workoutId = $page.params.workoutId;

	onMount(async () => {
		await loadWorkout();
	});

	async function loadWorkout() {
		isLoading = true;
		error = '';

		if (!workoutId) {
			error = 'Invalid workout ID';
			isLoading = false;
			return;
		}

		try {
			const response = await workoutsClient.getWorkout(
				new GetWorkoutRequest({ workoutId: BigInt(workoutId) })
			);

			if (response.success && response.workout) {
				workout = response.workout;
			} else {
				error = response.message || 'Workout not found';
			}
		} catch (e) {
			console.error('Failed to load workout:', e);
			error = `Failed to load workout: ${e}`;
		} finally {
			isLoading = false;
		}
	}

	function formatDate(dateStr: string): string {
		const date = new Date(dateStr);
		return date.toLocaleDateString('en-US', {
			weekday: 'long',
			year: 'numeric',
			month: 'long',
			day: 'numeric'
		});
	}

	function showAddSet(exerciseId: bigint) {
		activeExerciseId = exerciseId;
		newSetReps = '';
		newSetWeight = '';
	}

	function cancelAddSet() {
		activeExerciseId = null;
		newSetReps = '';
		newSetWeight = '';
	}

	async function addSet(workoutExerciseId: bigint) {
		if (!newSetReps && !newSetWeight) return;

		isAddingSet = true;
		error = '';

		try {
			const response = await workoutsClient.createSet(
				new CreateSetRequest({
					workoutExerciseId,
					reps: parseInt(newSetReps) || 0,
					weight: newSetWeight || '',
					notes: ''
				})
			);

			if (response.success) {
				cancelAddSet();
				await loadWorkout();
			} else {
				error = response.message;
			}
		} catch (e) {
			console.error('Failed to add set:', e);
			error = `Failed to add set: ${e}`;
		} finally {
			isAddingSet = false;
		}
	}

	function startEditSet(set: ExerciseSet) {
		editingSetId = set.id;
		editSetReps = set.reps.toString();
		editSetWeight = set.weight;
	}

	function cancelEditSet() {
		editingSetId = null;
		editSetReps = '';
		editSetWeight = '';
	}

	async function updateSet() {
		if (!editingSetId) return;

		isUpdatingSet = true;
		error = '';

		try {
			const response = await workoutsClient.updateSet(
				new UpdateSetRequest({
					setId: editingSetId,
					reps: parseInt(editSetReps) || 0,
					weight: editSetWeight || '',
					notes: ''
				})
			);

			if (response.success) {
				cancelEditSet();
				await loadWorkout();
			} else {
				error = response.message;
			}
		} catch (e) {
			console.error('Failed to update set:', e);
			error = `Failed to update set: ${e}`;
		} finally {
			isUpdatingSet = false;
		}
	}

	async function deleteSet(setId: bigint) {
		if (!confirm('Delete this set?')) return;

		try {
			const response = await workoutsClient.deleteSet(new DeleteSetRequest({ setId }));

			if (response.success) {
				await loadWorkout();
			} else {
				error = response.message;
			}
		} catch (e) {
			console.error('Failed to delete set:', e);
			error = `Failed to delete set: ${e}`;
		}
	}

	async function removeExercise(workoutExerciseId: bigint) {
		if (!confirm('Remove this exercise from the workout?')) return;

		try {
			const response = await workoutsClient.removeExerciseFromWorkout(
				new RemoveExerciseFromWorkoutRequest({ workoutExerciseId })
			);

			if (response.success) {
				await loadWorkout();
			} else {
				error = response.message;
			}
		} catch (e) {
			console.error('Failed to remove exercise:', e);
			error = `Failed to remove exercise: ${e}`;
		}
	}
</script>

<main>
	{#if isLoading}
		<div class="loading">Loading workout...</div>
	{:else if error && !workout}
		<div class="error-page">
			<h1>Error</h1>
			<p>{error}</p>
			<a href="/dashboard" class="btn-primary">Back to Dashboard</a>
		</div>
	{:else if workout}
		<header>
			<div class="header-info">
				<h1>Workout</h1>
				<p class="workout-date">{formatDate(workout.workoutDate)}</p>
			</div>
			<div class="header-actions">
				<a href="/workouts/{workoutId}/add-exercise" class="btn-primary">+ Add Exercise</a>
				<a href="/dashboard" class="btn-secondary">Done</a>
			</div>
		</header>

		{#if error}
			<div class="error">{error}</div>
		{/if}

		{#if workout.exercises.length === 0}
			<div class="empty">
				<p>No exercises yet</p>
				<p>Add exercises to start tracking your workout!</p>
				<a href="/workouts/{workoutId}/add-exercise" class="btn-primary">+ Add Exercise</a>
			</div>
		{:else}
			<div class="exercises">
				{#each workout.exercises as exercise}
					<div class="exercise-card">
						<div class="exercise-header">
							<div>
								<h2>{exercise.exerciseName}</h2>
								<p class="category">{exercise.categoryName}</p>
							</div>
							<button
								class="btn-remove"
								on:click={() => removeExercise(exercise.id)}
								title="Remove exercise"
							>
								&times;
							</button>
						</div>

						<div class="sets">
							{#if exercise.sets.length > 0}
								<table class="sets-table">
									<thead>
										<tr>
											<th>Set</th>
											<th>Reps</th>
											<th>Weight</th>
											<th></th>
										</tr>
									</thead>
									<tbody>
										{#each exercise.sets as set, index}
											<tr>
												{#if editingSetId === set.id}
													<td>{index + 1}</td>
													<td>
														<input
															type="number"
															bind:value={editSetReps}
															class="set-input"
															min="0"
														/>
													</td>
													<td>
														<input
															type="text"
															bind:value={editSetWeight}
															class="set-input"
															placeholder="kg"
														/>
													</td>
													<td class="actions">
														<button
															class="btn-save"
															on:click={updateSet}
															disabled={isUpdatingSet}
														>
															{isUpdatingSet ? '...' : 'Save'}
														</button>
														<button class="btn-cancel" on:click={cancelEditSet}>
															Cancel
														</button>
													</td>
												{:else}
													<td>{index + 1}</td>
													<td>{set.reps || '-'}</td>
													<td>{set.weight ? `${set.weight} kg` : '-'}</td>
													<td class="actions">
														<button class="btn-edit" on:click={() => startEditSet(set)}>
															Edit
														</button>
														<button class="btn-delete" on:click={() => deleteSet(set.id)}>
															Delete
														</button>
													</td>
												{/if}
											</tr>
										{/each}
									</tbody>
								</table>
							{:else}
								<p class="no-sets">No sets recorded yet</p>
							{/if}

							{#if activeExerciseId === exercise.id}
								<div class="add-set-form">
									<input
										type="number"
										bind:value={newSetReps}
										placeholder="Reps"
										class="set-input"
										min="0"
									/>
									<input
										type="text"
										bind:value={newSetWeight}
										placeholder="Weight (kg)"
										class="set-input"
									/>
									<button
										class="btn-primary"
										on:click={() => addSet(exercise.id)}
										disabled={isAddingSet || (!newSetReps && !newSetWeight)}
									>
										{isAddingSet ? 'Adding...' : 'Add'}
									</button>
									<button class="btn-cancel" on:click={cancelAddSet}>Cancel</button>
								</div>
							{:else}
								<button class="btn-add-set" on:click={() => showAddSet(exercise.id)}>
									+ Add Set
								</button>
							{/if}
						</div>
					</div>
				{/each}
			</div>
		{/if}
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
		align-items: flex-start;
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

	.workout-date {
		margin: 0.25rem 0 0 0;
		color: #666;
		font-size: 1rem;
	}

	.header-actions {
		display: flex;
		gap: 0.5rem;
		flex-wrap: wrap;
	}

	.btn-primary {
		padding: 0.5rem 1rem;
		background: #28a745;
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
		background: #218838;
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

	.error-page {
		text-align: center;
		padding: 3rem 1rem;
	}

	.error-page h1 {
		color: #dc3545;
	}

	.error {
		margin-bottom: 1rem;
		padding: 1rem;
		background: #f8d7da;
		color: #721c24;
		border: 1px solid #f5c6cb;
		border-radius: 4px;
	}

	.exercises {
		display: flex;
		flex-direction: column;
		gap: 1.5rem;
	}

	.exercise-card {
		background: white;
		border: 1px solid #ddd;
		border-radius: 8px;
		padding: 1.5rem;
	}

	.exercise-header {
		display: flex;
		justify-content: space-between;
		align-items: flex-start;
		margin-bottom: 1rem;
		padding-bottom: 0.75rem;
		border-bottom: 1px solid #eee;
	}

	.exercise-header h2 {
		margin: 0;
		font-size: 1.3rem;
		color: #333;
	}

	.category {
		margin: 0.25rem 0 0 0;
		font-size: 0.85rem;
		color: #666;
	}

	.btn-remove {
		background: none;
		border: none;
		font-size: 1.5rem;
		color: #999;
		cursor: pointer;
		padding: 0;
		line-height: 1;
	}

	.btn-remove:hover {
		color: #dc3545;
	}

	.sets-table {
		width: 100%;
		border-collapse: collapse;
		margin-bottom: 1rem;
	}

	.sets-table th,
	.sets-table td {
		padding: 0.5rem;
		text-align: left;
		border-bottom: 1px solid #eee;
	}

	.sets-table th {
		font-weight: 600;
		color: #666;
		font-size: 0.85rem;
	}

	.sets-table .actions {
		text-align: right;
	}

	.set-input {
		width: 80px;
		padding: 0.3rem 0.5rem;
		border: 1px solid #ccc;
		border-radius: 4px;
		font-size: 0.9rem;
	}

	.btn-edit,
	.btn-delete,
	.btn-save,
	.btn-cancel {
		padding: 0.25rem 0.5rem;
		border: none;
		border-radius: 4px;
		font-size: 0.8rem;
		cursor: pointer;
		margin-left: 0.25rem;
	}

	.btn-edit {
		background: #007bff;
		color: white;
	}

	.btn-edit:hover {
		background: #0056b3;
	}

	.btn-delete {
		background: #dc3545;
		color: white;
	}

	.btn-delete:hover {
		background: #c82333;
	}

	.btn-save {
		background: #28a745;
		color: white;
	}

	.btn-save:hover:not(:disabled) {
		background: #218838;
	}

	.btn-cancel {
		background: #6c757d;
		color: white;
	}

	.btn-cancel:hover {
		background: #5a6268;
	}

	.no-sets {
		color: #666;
		font-style: italic;
		margin: 0.5rem 0;
	}

	.add-set-form {
		display: flex;
		gap: 0.5rem;
		align-items: center;
		flex-wrap: wrap;
		margin-top: 0.5rem;
	}

	.btn-add-set {
		padding: 0.5rem 1rem;
		background: #f8f9fa;
		color: #007bff;
		border: 1px dashed #007bff;
		border-radius: 4px;
		font-size: 0.9rem;
		cursor: pointer;
		width: 100%;
		margin-top: 0.5rem;
	}

	.btn-add-set:hover {
		background: #e9ecef;
	}
</style>
