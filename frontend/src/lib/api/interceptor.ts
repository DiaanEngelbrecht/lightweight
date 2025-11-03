import type { Interceptor } from '@connectrpc/connect';
import { authStore } from '$lib/stores/auth';
import { get } from 'svelte/store';

/**
 * Creates an interceptor that adds the JWT token to all requests
 * Uses the "authorization: Bearer {token}" header format for gRPC
 */
export function createAuthInterceptor(): Interceptor {
	return (next) => async (req) => {
		// Get the current token from the auth store
		const token = authStore.getToken();

		// If a token exists, add it to the request headers
		if (token) {
			req.header.set('authorization', `Bearer ${token}`);
		}

		// Continue with the request
		return await next(req);
	};
}
