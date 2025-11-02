<script lang="ts">
	import { authStore } from '$lib/stores/auth';
	import { goto } from '$app/navigation';
	import { onMount } from 'svelte';

	// Protect all dashboard routes
	onMount(() => {
		const unsubscribe = authStore.subscribe(state => {
			if (!state.isAuthenticated) {
				goto('/');
			}
		});

		// Check immediately on mount
		if (!$authStore.isAuthenticated) {
			goto('/');
		}

		return unsubscribe;
	});
</script>

<slot />
