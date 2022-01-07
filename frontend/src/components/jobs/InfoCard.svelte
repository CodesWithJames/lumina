<script lang="ts">
import type { InfoCard } from "@components/jobs/jobs_types";

export let info: InfoCard

</script>

<div class="main">
    <div class="icon" style="color: {info.color}">
        <svelte:component this={info.icon}/>
    </div>
    <div class="text" style="color: {info.color}">
        {info.text}
    </div>
</div>