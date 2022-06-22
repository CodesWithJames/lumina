<script lang="ts" context="module">
import type { Load } from '@sveltejs/kit';
import { alerts_init } from '$stores/alerts'
import { graph_init } from '$stores/graph';

    export const load: Load = function load({ session }) {

        return {
            stuff: {
                alerts: alerts_init(session.messages),
                graph: graph_init(session.auth_token)
            }
        };
    }
</script>
<script lang="ts">
import { setContext } from 'svelte';
import { writable } from 'svelte/store';
import { page, session } from '$app/stores';
import { MessageType } from '$types/message';
import { browser } from '$app/env';

</script>
<slot/>