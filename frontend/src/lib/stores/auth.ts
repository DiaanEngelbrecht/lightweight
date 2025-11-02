import { writable } from 'svelte/store';
import { goto } from '$app/navigation';
import { browser } from '$app/environment';

const TOKEN_KEY = 'auth_token';

interface AuthState {
    isAuthenticated: boolean;
    token: string | null;
    user: {
        email?: string;
        name?: string;
    } | null;
}

// Helper function to decode JWT and extract payload
function parseJWT(token: string): any {
    try {
        const base64Url = token.split('.')[1];
        const base64 = base64Url.replace(/-/g, '+').replace(/_/g, '/');
        const jsonPayload = decodeURIComponent(
            atob(base64)
                .split('')
                .map((c) => '%' + ('00' + c.charCodeAt(0).toString(16)).slice(-2))
                .join('')
        );
        return JSON.parse(jsonPayload);
    } catch (error) {
        console.error('Failed to parse JWT:', error);
        return null;
    }
}

// Helper function to check if token is expired
function isTokenExpired(token: string): boolean {
    const payload = parseJWT(token);
    if (!payload || !payload.exp) {
        return true;
    }
    return Date.now() >= payload.exp * 1000;
}

// Initialize auth state from localStorage
function initAuthState(): AuthState {
    if (!browser) {
        return { isAuthenticated: false, token: null, user: null };
    }

    const token = localStorage.getItem(TOKEN_KEY);
    if (token && !isTokenExpired(token)) {
        const payload = parseJWT(token);
        return {
            isAuthenticated: true,
            token,
            user: payload ? { email: payload.email, name: payload.name } : null
        };
    }

    // Clear invalid/expired token
    if (token) {
        localStorage.removeItem(TOKEN_KEY);
    }

    return { isAuthenticated: false, token: null, user: null };
}

// Create the auth store
function createAuthStore() {
    const { subscribe, set, update } = writable<AuthState>(initAuthState());

    return {
        subscribe,

        // Login with token
        login(token: string, user?: { email?: string; name?: string }) {
            if (browser) {
                localStorage.setItem(TOKEN_KEY, token);
            }

            const payload = parseJWT(token);
            const userData = user || (payload ? { email: payload.email, name: payload.name } : null);

            set({
                isAuthenticated: true,
                token,
                user: userData
            });
        },

        // Logout
        logout() {
            if (browser) {
                localStorage.removeItem(TOKEN_KEY);
            }
            set({
                isAuthenticated: false,
                token: null,
                user: null
            });
            goto('/');
        },

        // Check if authenticated (useful for guards)
        checkAuth(): boolean {
            const state = initAuthState();
            if (!state.isAuthenticated) {
                set(state);
                return false;
            }
            return true;
        },

        // Get token for API calls
        getToken(): string | null {
            if (!browser) return null;
            const token = localStorage.getItem(TOKEN_KEY);
            if (token && !isTokenExpired(token)) {
                return token;
            }
            return null;
        }
    };
}

export const authStore = createAuthStore();
