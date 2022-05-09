<script lang="ts">
import InfoCard from "$components/jobs/InfoCard.svelte"
import ArrowRight from "svelte-material-icons/ArrowRight.svelte"
import type { JobType } from "$components/jobs/jobs_types"

export let job: JobType

</script>

<a href="/" class="main">
    <div class="heading">
        <div class="icon">
            <svelte:component this={job.icon}/>
        </div>
        <div class="title">
            {job.title}
        </div>
    </div>
    <div class="info-wrapper">
        {#each job.attributes as info}
            <InfoCard bind:info />
        {/each}
        <div class="arrow icon">
            <svelte:component this={ArrowRight} />
        </div>
    </div>
</a>

<style lang="stylus">
@import 'variables'

.main
    text-decoration none
    display flex
    flex-direction column
    gap 10px
    border 2px solid mix($dark, transparent, 10%)
    border-radius: 5px
    padding 20px
    color $dark

.main:hover
    background-color mix($dark, transparent, 5%)
    border 2px solid $brand

.heading
    display flex
    align-items center
    gap 10px

.icon
    font-size 24px
    display inline-flex

.title
    font-size 18px
    font-weight 600
.info-wrapper
    display flex
    align-items center

.arrow
    margin-left auto
    color $brand

</style>