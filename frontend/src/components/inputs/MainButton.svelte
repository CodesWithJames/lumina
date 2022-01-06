
<script lang="ts">
import type { SvelteComponent } from 'svelte/internal'
import { createEventDispatcher } from 'svelte'

const dispatch = createEventDispatcher()

export let left_icon: any = undefined
export let right_icon: any = undefined
export let href: string = undefined
export let branded = false
export let dark = false
export let inline = false
</script>

<a
    class="button"
    class:branded
    class:dark
    class:inline

    on:click={event => dispatch('click', event)}
    {href}>
    {#if left_icon}
        <div class="icon">
            <svelte:component this={left_icon}/>
        </div>
    {/if}
    <slot/>
    {#if right_icon}
        <div class="icon">
            <svelte:component this={right_icon}/>
        </div>
    {/if}
</a>
<style lang="stylus">
@import 'variables'
.icon
    font-size 24px
    display inline-flex

.button
    padding 8px 16px
    border-radius 4px
    justify-content center
    display flex
    align-items center
    font-weight 600
    cursor pointer
    gap 8px
    color $dark
    background $titanium
    line-height 24px
    text-decoration none
    &.inline
        display inline-flex
    &:hover
        background mix($dark, $titanium, 5%)
    &.branded
        background $brand
        color white
        &:hover
            background mix(#000, $brand, 15%)
    &.dark
        background rgba(0, 0, 0, 0.25)
        color white
        &:hover
            background rgba(0, 0, 0, 0.35)
</style>