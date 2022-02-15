<script lang="ts">
import InfoCard from "@components/jobs/InfoCard.svelte"
import ArrowRight from "svelte-material-icons/ArrowRight.svelte"
import type { JobType } from "@components/jobs/jobs_types"

export let job: JobType

</script>

<a href="/">
    <div class="main">
        <div class="left">
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
            </div>
        </div>
        <div class="arrow">
            <svelte:component this={ArrowRight} />
        </div>
    </div>
</a>

<style lang="stylus">
@import 'variables'

a
    text-decoration none

.main
    display flex
    align-items center
    border 2px solid rgba(35, 36, 44, 0.1)
    border-radius: 5px
    padding 20px
    color $dark

.main:hover
    background-color rgba(45, 55, 68, 0.08)

.left
    display flex
    gap 10px
    flex-direction column

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

.arrow
    margin auto 0
    margin-left auto
    color #492C9C
    font-size 24px

</style>