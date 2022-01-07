<script lang="ts">
import Job from "@components/jobs/Job.svelte"
import type { JobField } from "./jobs_types";

export let jobField: JobField

</script>

<div class="main">
    <div class="heading">
        {jobField.title}
    </div>
    <div class="jobs-wrapper">
        {#each jobField.jobs as job}
            <Job 
                bind:job
            />
        {/each}
    </div>
</div>

<style lang="stylus">


</style>