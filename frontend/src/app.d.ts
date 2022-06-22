/// <reference types="@sveltejs/kit" />

import type { AlertsStore } from '$stores/alerts'
import type { GraphStore } from '$stores/graph'
import type { User } from '$types/user'
import type { Message } from './types/message'

// See https://kit.svelte.dev/docs/types#the-app-namespace
// for information about these interfaces

declare global {
    namespace App {
        // interface Locals {}
        // interface Platform {}
        interface Session {
            user: User | null
            messages: Message[];
            auth_token?: string;
        }
        interface Stuff {
            user: User | null;
            alerts: AlertsStore,
            graph: GraphStore
        }
    }
}
