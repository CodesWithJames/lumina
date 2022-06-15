<script lang="ts">
    import { createEventDispatcher } from 'svelte';

    let dispatch = createEventDispatcher();

    export let href: string = undefined;
    export let left_icon: any = undefined;
    export let right_icon: any = undefined;
    export let style: 'translucent' | 'transparent' | '' = '';
    export let disabled = false;
    export let hug = false;

    $: tag = href ? 'a' : 'div';
</script>

<svelte:element
    this={tag}
    href={href}
    on:click={(e) => !disabled && dispatch('click', e)}
    class:disabled
    class:hug
    class="button {style}"
    class:has_right_icon={!!right_icon}
    class:has_left_icon={!!left_icon}
>
    {#if left_icon}
        <span class="icon">
            <svelte:component this={left_icon} />
        </span>
    {/if}
    <span>
        <slot/>
    </span>
    {#if right_icon}
        <span class="icon">
            <svelte:component this={right_icon} />
        </span>
    {/if}
</svelte:element>

<style lang="stylus">
@import 'variables'

.button
    padding 10px 16px
    background: $brand
    color white
    display inline-flex
    align-items: center
    white-space nowrap
    justify-content: center
    border-radius 4px
    width 100%
    cursor pointer
    gap 8px
    font-weight 600
    &.hug
        width auto
    .icon
        font-size: 20px
        display: inline-flex
    &.has_right_icon
        padding-top: 10px
        padding-right: 10px
        padding-bottom: 10px

    &.has_left_icon
        padding-top: 10px
        padding-left: 10px
        padding-bottom: 10px
    &:hover
        background: lighten($brand, 12%)
    &.translucent
        background transparify(white, 8%)
        &:hover
            background: transparify(white, 14%)
    &.transparent
        color transparify(white, 60%)
        background transparify(white, 0%)
        &:hover
            background: transparify(white, 8%)
            color white
    &.disabled
        cursor default
        opacity 0.5
        background transparent
        border 1px solid transparify(white, 10%)


</style>
