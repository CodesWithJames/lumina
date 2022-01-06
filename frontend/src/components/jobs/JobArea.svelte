<script lang="ts">
import Job from "@components/jobs/Job.svelte"

export let heading: string
export let jobs: Job[]

</script>

<div class="main">
    <div class="heading">
        {heading}
    </div>
    <div class="jobs-wrapper">
        {#each jobs as job}
            <Job 
                job={
                    {
                        title: job.title,
                        icon: job.icon,
                        attributes: {
                            icon: any,
                            text: string,
                            color: string,
                        }
                    }
                }
            />
        {/each}
    </div>
</div>

<style lang="stylus">


</style>