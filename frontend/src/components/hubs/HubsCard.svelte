<script lang="ts">
import ChevronRight from "@icons/ChevronRight.svelte"

export let knowledgeInfo

export let areas

</script>

<div class="main">
    <a href="/">
        <div class="knowledge">
            <div class="left-wrapper">
                <div class="icon">
                    {knowledgeInfo.emoji}
                </div>
                <div class="heading">
                    {knowledgeInfo.title}
                </div>
                <p>
                    {knowledgeInfo.description}
                </p>
            </div>
            <div class="chevron-right">
                <svelte:component this={ChevronRight} />
            </div>
        </div>
    </a>
    {#each areas as info}
        <a href="/">
            <div class="area-wrapper">
                <div class="icon">
                    {info.emoji}
                </div>
                <div class="info-heading">
                    {info.title}
                </div>
                <div class="chevron-right">
                    <svelte:component this={ChevronRight} />
                </div>
            </div>
        </a>
    {/each}
</div>

<style lang="stylus">
@import "variables"

.main
    background-color white
    margin auto 0
    max-width 500px

a
    text-decoration none
    color $dark

.knowledge
    padding 40px
    display flex

.knowledge:hover
    background-color rgba(45, 55, 68, 0.05)

.heading
    margin 10px 0
    font-size 28px
    font-weight 700

p
    font-weight 500
    color rgba(45, 55, 68, 0.6)
    margin 0
    padding 0
    max-width 315px

.area-wrapper
    padding 40px
    display flex

.area-wrapper:hover
    background-color rgba(45, 55, 68, 0.05)

.icon
    font-size 40px
    margin-right 10px

.info-heading
    font-weight 600
    font-size 18px
    margin auto 0

.chevron-right
    margin auto 0
    margin-left auto
    font-size 28px

</style>