<script lang="ts">
import JobSection from "$components/jobs/JobSection.svelte"
import type { JobField } from "$components/jobs/jobs_types"

export let jobFields: JobField[]
</script>

<div class="job-info">
    <div class="header">
        <h2 class="job-heading">
            Open positions
        </h2>
        <p class="job-description">
            We are a technology driven society comprised of principled and disciplined people bonded by deeply held mission: to create a modern solution to governance.
        </p>
    </div>
    <div class="jobs-wrapper">
        {#each jobFields as jobField}
            <JobSection bind:jobField />
        {/each}
    </div>
</div>

<style lang="stylus">
.job-info
    display flex
    flex-direction column
    align-items center
    max-width 1000px
    width 100%
    margin 0 auto

.header
    display flex
    flex-direction column
    align-items center
    max-width 600px
    width 100%

.job-heading
    font-size 40px
    font-weight 600
    margin 0
    padding 0

.job-description
    opacity 0.6
    text-align center
    line-height 28px

.jobs-wrapper
    display flex
    flex-direction column
    gap 40px
    padding-top 40px

</style>