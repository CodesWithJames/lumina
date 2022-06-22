<div class="action-container-wrapper">
    {#each $alerts as message}
        <div
            bind:this={elements[message.id]}
            class={'action-bar ' + MessageType[message.type]}
            on:click={() => remove(message.id)}>
            <div class="icon">
                <svelte:component this={icons[message.type]} size="22px"/>
            </div>
            <div class="text">
                { message.text }
            </div>
            <Close size="22px"/>
        </div>
    {/each}
</div>
<script lang="ts">
import Alert from '$icons/Alert.svelte'
import Info from '$icons/AlertCircle.svelte'
import Close from '$icons/Close.svelte'
import Check from '$icons/CheckCircle.svelte'
import { onMount, tick } from 'svelte';
import { browser } from '$app/env';
import { MessageType } from '$types/message';
import { page } from '$app/stores';

let alerts = $page.stuff.alerts.store;

let elements = []

let icons = {
    [MessageType.Info]: Info,
    [MessageType.Warning]: Alert,
    [MessageType.Error]: Alert,
    [MessageType.Success]: Check
}

function remove(id: string){
    $alerts = $alerts.filter(val => val.id !== id)
}

let interval = null

function intervalFn () {
    $alerts.shift()
    setBottomHeights()
    $alerts = $alerts
}

$: {
    if(browser){
        if($alerts.length === 0 && interval){
            clearInterval(interval)
            interval = null
        } else if($alerts.length !== 0) {
            setBottomHeights()
            if(interval == null) interval = setInterval(intervalFn, 3000)
        }
    }
}

onMount(setBottomHeights)

async function setBottomHeights () {
    await tick()
    let elms = $alerts.map(message => elements[message.id])
    let heights = 0
    let margin = 10

    for(let i in elms) {
        let el = elms[i]
        let elHeight = el.offsetHeight
        heights += margin
        el.style.bottom = heights + 'px'
        heights += elHeight
    }
}

</script>
<style lang="stylus">

@import 'variables'

.action-container-wrapper
    position fixed
    bottom 10px
    left 0
    right 0
    height 0
    display flex
    align-items flex-end
    justify-content center
    .action-bar
        cursor pointer
        background $brand
        position fixed
        width 100%
        max-width 500px
        padding 8px
        font-weight 500
        box-shadow 0 0 5px rgba(0,0,0,0.1)
        border-radius 5px
        z-index 60
        color white
        transition 0.2s ease-in-out
        display grid
        grid-template-columns min-content 1fr min-content
        align-items center
        &.Warning
            background #b8992a
        &.Error
            background #b44
        &.Success
            background #4a6
        .icon
            padding-right 8px
            display inline-flex

</style>