<script lang="ts">
import InfoCard from "$components/jobs/InfoCard.svelte"
import ArrowRight from "svelte-material-icons/ArrowRight.svelte"
import type { JobType } from "$components/jobs/jobs_types"

export let job: JobType

</script>

<a href="{job.link}" class="main">
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
    cardify()
    display flex
    color white
    cursor pointer
    flex-direction column
    gap 10px
    border-radius 5px
    padding 16px
    border 2px solid transparent
    &:hover
        background transparify(white, 8%)

    .heading
        display flex
        align-items center
        gap 10px

    .icon
        font-size 24px
        display inline-flex
        color $brand

    .title
        font-size 18px
        font-weight 600
    .info-wrapper
        display flex
        align-items center
    .arrow
        margin-left auto

</style>