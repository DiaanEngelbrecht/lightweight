<script lang="ts">
	import { onMount } from 'svelte';
	import { createPromiseClient } from '@connectrpc/connect';
	import { Workouts } from '$lib/generated/workouts_connect';
	import { transport } from '$lib/api/client';
	import {
		ListWorkoutsRequest,
		DeleteWorkoutRequest,
		type WorkoutSummary
	} from '$lib/generated/workouts_pb';
	import { goto } from '$app/navigation';

	const workoutsClient = createPromiseClient(Workouts, transport);

	let workouts: WorkoutSummary[] = [];
	let isLoading = true;
	let error = '';
	let hasMore = true;
	let offset = 0;
	const limit = 20;

	onMount(async () => {
		await loadWorkouts();
	});

	async function loadWorkouts() {
		isLoading = true;
		error = '';

		try {
			const response = await workoutsClient.listWorkouts(
				new ListWorkoutsRequest({ limit, offset })
			);

			if (response.success) {
				workouts = response.workouts;
				hasMore = response.workouts.length === limit;
			} else {
				error = response.message;
			}
		} catch (e) {
			console.error('Failed to load workouts:', e);
			error = `Failed to load workouts: ${e}`;
		} finally {
			isLoading = false;
		}
	}

	async function loadMore() {
		offset += limit;
		isLoading = true;

		try {
			const response = await workoutsClient.listWorkouts(
				new ListWorkoutsRequest({ limit, offset })
			);

			if (response.success) {
				workouts = [...workouts, ...response.workouts];
				hasMore = response.workouts.length === limit;
			}
		} catch (e) {
			console.error('Failed to load more workouts:', e);
		} finally {
			isLoading = false;
		}
	}

	function formatDate(dateStr: string): string {
		const date = new Date(dateStr);
		return date.toLocaleDateString('en-US', {
			weekday: 'short',
			year: 'numeric',
			month: 'short',
			day: 'numeric'
		});
	}

	function isToday(dateStr: string): boolean {
		const date = new Date(dateStr);
		const today = new Date();
		return (
			date.getDate() === today.getDate() &&
			date.getMonth() === today.getMonth() &&
			date.getFullYear() === today.getFullYear()
		);
	}

	async function deleteWorkout(workoutId: bigint, event: Event) {
		event.stopPropagation();
		if (!confirm('Delete this workout? This cannot be undone.')) return;

		try {
			const response = await workoutsClient.deleteWorkout(
				new DeleteWorkoutRequest({ workoutId })
			);

			if (response.success) {
				workouts = workouts.filter((w) => w.id !== workoutId);
			} else {
				error = response.message;
			}
		} catch (e) {
			console.error('Failed to delete workout:', e);
			error = `Failed to delete workout: ${e}`;
		}
	}

	function viewWorkout(workoutId: bigint) {
		goto(`/workouts/${workoutId}`);
	}
</script>

<main>
	<header>
		<h1>Workout History</h1>
		<a href="/dashboard" class="btn-secondary">Back to Dashboard</a>
	</header>

	{#if error}
		<div class="error">{error}</div>
	{/if}

	{#if isLoading && workouts.length === 0}
		<div class="loading">Loading workouts...</div>
	{:else if workouts.length === 0}
		<div class="empty">
			<p>No workouts yet</p>
			<p>Start a workout from the dashboard to begin tracking!</p>
			<a href="/dashboard" class="btn-primary">Go to Dashboard</a>
		</div>
	{:else}
		<div class="workouts-list">
			{#each workouts as workout}
				<div
					class="workout-card"
					on:click={() => viewWorkout(workout.id)}
					on:keypress={(e) => e.key === 'Enter' && viewWorkout(workout.id)}
					role="button"
					tabindex="0"
				>
					<div class="workout-info">
						<div class="workout-date">
							{formatDate(workout.workoutDate)}
							{#if isToday(workout.workoutDate)}
								<span class="today-badge">Today</span>
							{/if}
						</div>
						<div class="workout-stats">
							<span class="stat">
								<strong>{workout.exerciseCount}</strong>
								{workout.exerciseCount === BigInt(1) ? 'exercise' : 'exercises'}
							</span>
							<span class="stat">
								<strong>{workout.totalSets}</strong>
								{workout.totalSets === BigInt(1) ? 'set' : 'sets'}
							</span>
						</div>
						{#if workout.notes}
							<p class="workout-notes">{workout.notes}</p>
						{/if}
					</div>
					<div class="workout-actions">
						<button
							class="btn-delete"
							on:click={(e) => deleteWorkout(workout.id, e)}
							title="Delete workout"
						>
							Delete
						</button>
						<span class="arrow">→</span>
					</div>
				</div>
			{/each}
		</div>

		{#if hasMore}
			<div class="load-more">
				<button class="btn-load-more" on:click={loadMore} disabled={isLoading}>
					{isLoading ? 'Loading...' : 'Load More'}
				</button>
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

	.workouts-list {
		display: flex;
		flex-direction: column;
		gap: 1rem;
	}

	.workout-card {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 1.25rem 1.5rem;
		background: white;
		border: 1px solid #ddd;
		border-radius: 8px;
		cursor: pointer;
		transition: all 0.2s;
		text-align: left;
		width: 100%;
	}

	.workout-card:hover {
		border-color: #007bff;
		transform: translateX(4px);
		box-shadow: 0 2px 8px rgba(0, 0, 0, 0.08);
	}

	.workout-info {
		flex: 1;
	}

	.workout-date {
		font-size: 1.1rem;
		font-weight: 600;
		color: #333;
		display: flex;
		align-items: center;
		gap: 0.5rem;
	}

	.today-badge {
		background: #28a745;
		color: white;
		font-size: 0.7rem;
		font-weight: 600;
		padding: 0.15rem 0.5rem;
		border-radius: 4px;
		text-transform: uppercase;
	}

	.workout-stats {
		display: flex;
		gap: 1rem;
		margin-top: 0.5rem;
		font-size: 0.9rem;
		color: #666;
	}

	.stat strong {
		color: #333;
	}

	.workout-notes {
		margin: 0.5rem 0 0 0;
		font-size: 0.85rem;
		color: #666;
		font-style: italic;
	}

	.workout-actions {
		display: flex;
		align-items: center;
		gap: 1rem;
	}

	.btn-delete {
		padding: 0.25rem 0.5rem;
		background: #dc3545;
		color: white;
		border: none;
		border-radius: 4px;
		font-size: 0.8rem;
		cursor: pointer;
		opacity: 0;
		transition: opacity 0.2s;
	}

	.workout-card:hover .btn-delete {
		opacity: 1;
	}

	.btn-delete:hover {
		background: #c82333;
	}

	.arrow {
		font-size: 1.5rem;
		color: #007bff;
	}

	.load-more {
		text-align: center;
		margin-top: 2rem;
	}

	.btn-load-more {
		padding: 0.75rem 2rem;
		background: #f8f9fa;
		color: #007bff;
		border: 1px solid #007bff;
		border-radius: 4px;
		font-size: 1rem;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-load-more:hover:not(:disabled) {
		background: #007bff;
		color: white;
	}

	.btn-load-more:disabled {
		opacity: 0.6;
		cursor: not-allowed;
	}
</style>
