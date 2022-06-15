<script lang="ts" context="module">
    import type { Load } from '@sveltejs/kit';
    import { alerts_init } from '$stores/alerts'

    export const load: Load = function load({ session }) {

        return {
            stuff: {
                alerts: alerts_init(session.messages)
            }
        };
    }
</script>
<slot/>