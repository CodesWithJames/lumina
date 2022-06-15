<script lang="ts">
export let href: string = undefined;
export let icon: any = undefined;
export let active: boolean = false;
export let opacity: boolean = true;

$: tag = href ? 'a' : 'div';
</script>

<svelte:element
    on:click|stopPropagation
    this={tag}
    href={href}
    class="button"
    tabindex="0"
    class:active
    class:opacity
>
    <svelte:component this={icon} />
</svelte:element>

<style lang="stylus">
@import 'variables'

.button
    padding 8px
    background none
    color white
    display inline-flex
    align-items center
    justify-content center
    border-radius 4px
    cursor pointer
    font-weight 600
    font-size 24px
    &.opacity
        color transparify(white, 40%)
    &.active
        background transparify(white, 8%)
        color white
    &:hover
        background transparify(white, 12%)
        color white
    &:focus
        outline 2px solid $brand


</style>