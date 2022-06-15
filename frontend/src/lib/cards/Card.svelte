<script lang="ts">
export let href: string | undefined = undefined;
export let padding = "0px";
export let click_handler: ((e: Event) => void) | undefined = undefined;
export let max_width: string | undefined = undefined;
export let shadow = true;
export let direction: 'vertical' | 'horizontal' = 'vertical';
export let gap = "0px";
export let reset_bg = false;

$: is_clickable = href || click_handler
$: tag = is_clickable ? 'a' : 'div'
</script>
<svelte:element
    class="card"
    this={tag}
    href={href}
    class:shadow
    class:reset_bg
    class:vertical={direction === 'vertical'}
    class:horizontal={direction === 'horizontal'}
    style="
        max-width: {max_width};
        gap: {gap};
        padding: {padding};
    "
    class:hoverable={is_clickable}>
    <slot/>
</svelte:element>
<style lang="stylus">
@import 'variables'

.card
    border-radius 4px
    background transparify(white, 4%)
    width 100%
    display flex
    align-items center
    justify-content center
    &.shadow
        box-shadow 0 0 4px rgba(0, 0, 0, 0.2)
    &.reset_bg
        background mix(white, $dark_app, 10%)
    &.vertical
        flex-direction column
    &.horizontal
        flex-direction row
    &.padding
        padding 20px
</style>