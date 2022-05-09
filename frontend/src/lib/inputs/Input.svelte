<script lang="ts">
export let name: string
export let placeholder: string = ''
export let type: string = ''
export let left_icon: any = undefined
export let right_icon: any = undefined
export let right_icon_fn: any = undefined
export let value: any = undefined

let input: HTMLInputElement

function right_icon_click(event) {
    event.stopPropagation()
    if(right_icon_fn) right_icon_fn()
}

</script>
<div class="input-wrapper" on:click={() => input.focus()}>
    <span class="input-label">{name}</span>
    <div class="input-pseudo-wrapper">
        {#if left_icon}
            <div class="icon">
                <svelte:component this={left_icon}/>
            </div>
        {/if}
        <input bind:this={input} {placeholder} {type} on:input={event => value = event.target.value} {value}>
        {#if right_icon}
            <div class="icon" class:clickable={!!right_icon_fn} on:click={right_icon_click}>
                <svelte:component this={right_icon}/>
            </div>
        {/if}
    </div>
</div>
<style lang="stylus">

.input-wrapper
    display grid
    grid-auto-flow row
    grid-gap 5px
    color white

.clickable
    cursor pointer
    border-radius 50px
    &:hover
        background transparify(white, 4%)
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