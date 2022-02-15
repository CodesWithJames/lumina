<script lang="ts">
import JobCard from "@components/jobs/JobCard.svelte"
import type { JobField } from "./jobs_types";

export let jobField: JobField

</script>

<div class="main">
    <div class="heading">
        {jobField.title}
    </div>
    <div class="jobs-wrapper">
        {#each jobField.jobs as job}
            <JobCard bind:job />
        {/each}
    </div>
</div>

<style lang="stylus">
.heading
    opacity 0.5
    font-size 14px
    text-transform uppercase
    font-weight 600
    margin-bottom 10px

.jobs-wrapper
    display grid   
    grid-template-columns 1fr 1fr 1fr
    grid-gap 20px
</style>