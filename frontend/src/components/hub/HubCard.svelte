
<script lang="ts">
import type {HubType} from "@components/hub/hub_type"
import ArrowRight from "svelte-material-icons/ArrowRight.svelte"
export let info: HubType

let greyedOut = info.greyedOut

</script>

{#if !greyedOut}
    <a href="/" class="main" class:greyedOut>
        <div class="header">
            <div class="tag" style="background-color: {info.tag.color};">
                {info.tag.text}
            </div>
            <div class="arrow">
                <svelte:component this={ArrowRight}/>
            </div>
        </div>
        <div class="title">
            <div class="icon">
                <svelte:component this={info.title.icon}/>
            </div>
            <div class="heading">
                {info.title.text}
            </div>
        </div>
        <p>
            {info.description}
        </p>
    </a>
{:else}
    <div class="main" class:greyedOut>
        <div class="header">
            <div class="tag" style="background-color: {info.tag.color};">
                {info.tag.text}
            </div>
        </div>
        <div class="title">
            <div class="icon">
                <svelte:component this={info.title.icon}/>
            </div>
            <div class="heading">
                {info.title.text}
            </div>
        </div>
        <p>
            {info.description}
        </p>
    </div>
{/if}

<style lang="stylus">
@import 'variables'

.main
    background-color white
    border-radius 5px
    padding 17px
    border 3px solid white
    text-decoration none
    color $dark

.main:not(.greyedOut):hover
    border 3px solid $brand

.greyedOut
    opacity 0.5

.header
    display flex
    margin-bottom 10px

.tag
    padding 4px 6px
    border-radius 5px
    color white

.arrow
    margin-left auto
    font-size 24px

.title
    display flex

.icon
    font-size 24px
    margin-right 10px
    height 24px
    width 24px

.heading
    font-size 20px
    font-weight 600
    margin auto 0

p
    line-height 24px
    margin 0
    padding 0

</style>
