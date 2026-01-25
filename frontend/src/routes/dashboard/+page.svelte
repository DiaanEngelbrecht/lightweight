<script lang="ts">
	import { authStore } from '$lib/stores/auth';
	import { createPromiseClient } from '@connectrpc/connect';
	import { Workouts } from '$lib/generated/workouts_connect';
	import { transport } from '$lib/api/client';
	import { GetTodaysWorkoutRequest, CreateWorkoutRequest } from '$lib/generated/workouts_pb';
	import { goto } from '$app/navigation';
	import { onMount } from 'svelte';

	const workoutsClient = createPromiseClient(Workouts, transport);

	let isStartingWorkout = false;
	let workoutError = '';

	function handleLogout() {
		authStore.logout();
	}

	async function startWorkout() {
		isStartingWorkout = true;
		workoutError = '';

		try {
			// Check if there's already a workout for today
			const todayResponse = await workoutsClient.getTodaysWorkout(new GetTodaysWorkoutRequest());

			if (todayResponse.hasWorkout && todayResponse.workout) {
				// Navigate to existing workout
				goto(`/workouts/${todayResponse.workout.id}`);
			} else {
				// Create new workout for today
				const today = new Date().toISOString().split('T')[0];
				const createResponse = await workoutsClient.createWorkout(
					new CreateWorkoutRequest({ workoutDate: today, notes: '' })
				);

				if (createResponse.success) {
					goto(`/workouts/${createResponse.workoutId}`);
				} else {
					workoutError = createResponse.message;
				}
			}
		} catch (e) {
			console.error('Failed to start workout:', e);
			workoutError = `Failed to start workout: ${e}`;
		} finally {
			isStartingWorkout = false;
		}
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

	{#if workoutError}
		<div class="error">{workoutError}</div>
	{/if}

	<section class="actions">
		<button class="action-btn primary" on:click={startWorkout} disabled={isStartingWorkout}>
			<span class="icon">🏋️</span>
			<span class="label">{isStartingWorkout ? 'Starting...' : 'Start Workout'}</span>
		</button>
		<a href="/workouts" class="action-btn">
			<span class="icon">📋</span>
			<span class="label">Workout History</span>
		</a>
		<a href="/exercises" class="action-btn">
			<span class="icon">💪</span>
			<span class="label">Manage Exercises</span>
		</a>
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

	.actions {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(250px, 1fr));
		gap: 1.5rem;
		margin-top: 2rem;
	}

	.action-btn {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: 2rem;
		background: white;
		border: 2px solid #007bff;
		border-radius: 8px;
		text-decoration: none;
		color: #007bff;
		transition: all 0.2s;
		cursor: pointer;
	}

	.action-btn:hover {
		background: #007bff;
		color: white;
		transform: translateY(-2px);
		box-shadow: 0 4px 8px rgba(0, 0, 0, 0.1);
	}

	.action-btn .icon {
		font-size: 3rem;
		margin-bottom: 0.5rem;
	}

	.action-btn .label {
		font-size: 1.1rem;
		font-weight: 600;
	}

	.action-btn.primary {
		background: #28a745;
		border-color: #28a745;
		color: white;
	}

	.action-btn.primary:hover:not(:disabled) {
		background: #218838;
		border-color: #218838;
		color: white;
	}

	.action-btn:disabled {
		opacity: 0.6;
		cursor: not-allowed;
		transform: none;
	}

	.error {
		margin: 1rem 0;
		padding: 1rem;
		background: #f8d7da;
		color: #721c24;
		border: 1px solid #f5c6cb;
		border-radius: 4px;
	}
</style>
