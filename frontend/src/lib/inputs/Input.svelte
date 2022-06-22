<script lang="ts">
import { onMount } from 'svelte';

export let name: string;
export let placeholder: string = '';
export let type: string = '';
export let left_icon: any = undefined;
export let right_icon: any = undefined;
export let left_icon_handler: ((e: Event) => void) | undefined = undefined;
export let right_icon_handler: ((e: Event) => void) | undefined = undefined;
export let focus_on_mount = false;
export let value: any = undefined;

export let input_ref: HTMLInputElement = undefined;

function handle_icon_click(e: Event, handler: ((e: Event) => void) | undefined) {
    if (handler) {
        e.stopPropagation();
        handler(e);
    }
}

onMount(() => {
    if (focus_on_mount) {
        input_ref.focus();
    }
});
</script>

<div class="input-wrapper" on:click={() => input_ref.focus()}>
    <span class="input-label">{name}</span>
    <div class="input-pseudo-wrapper">
        {#if left_icon}
            <div class="icon"
                class:clickable={!!left_icon_handler}
                on:click={e => handle_icon_click(e, right_icon_handler)}>
                <svelte:component this={left_icon} />
            </div>
        {/if}
        <input
            on:keypress
            on:keydown
            on:keyup
            bind:this={input_ref}
            {placeholder}
            {type}
            on:input={(event) => (value = event.currentTarget.value)}
            {value}
        />
        {#if right_icon}
            <div class="icon"
                class:clickable={!!right_icon_handler}
                on:click={e => handle_icon_click(e, right_icon_handler)}>
                <svelte:component this={right_icon} />
            </div>
        {/if}
    </div>
</div>

<style lang="stylus">
@import 'variables';

.input-wrapper
    display grid
    grid-auto-flow row
    grid-gap 5px
    color white
    width 100%

.clickable
    cursor pointer
    border-radius 50px
    &:hover
        background transparify(white, 6%)
    &:active
        background transparify(white, 12%)

.input-label
    font-size 14px
    font-weight 700
    text-transform uppercase
    opacity 0.4

input
    appearance none
    border none
    outline none
    background none
    color inherit
    flex 1
    width 100%
    padding 12px 16px
    padding-left 6px
    margin 0
    &::placeholder
        opacity 0.4

.input-pseudo-wrapper
    padding 0 6px
    border-radius 4px
    align-items center
    cursor text
    display flex
    background rgba(255,255,255,0.1)

.icon
    font-size 20px
    padding 6px
    display inline-flex
    align-items center
    color rgba(255, 255, 255, 0.5)

</style>
