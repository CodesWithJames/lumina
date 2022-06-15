<script lang="ts">
    import { createEventDispatcher } from 'svelte';

    let dispatch = createEventDispatcher();

    export let href: string = undefined;
    export let left_icon: any = undefined;
    export let right_icon: any = undefined;
    export let style: 'translucent' | 'branded' = 'translucent';
    export let disabled = false;

    $: tag = href ? 'a' : 'div';
</script>

<svelte:element
    this={tag}
    href={href}
    on:click={(e) => !disabled && dispatch('click', e)}
    class:disabled
    class="segment {style}"
>
    {#if left_icon}
        <span class="icon">
            <svelte:component this={left_icon} />
        </span>
    {/if}
    <slot />
    {#if right_icon}
        <span class="icon">
            <svelte:component this={right_icon} />
        </span>
    {/if}
</svelte:element>

<style lang="stylus">
@import 'variables'

.segment
    padding 8px
    background transparify(white, 6%)
    color white
    display inline-flex
    align-items: center
    justify-content: center
    border-radius 50px
    cursor pointer
    gap 8px
    font-weight 500
    .icon
        font-size: 20px
        display: inline-flex
    &:hover
        background transparify(white, 10%)
    &.brand
        background $brand
        &:hover
            background lighten($brand, 12%)
    &.disabled
        cursor default
        opacity 0.5
        background transparent
        border 1px solid transparify(white, 10%)
</style>
