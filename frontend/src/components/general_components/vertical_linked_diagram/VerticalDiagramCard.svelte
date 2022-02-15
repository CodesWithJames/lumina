<script lang="ts">
import type { VerticalDiagramCardType } from "@components/general_components/vertical_linked_diagram/vertical_diagram_types"
export let cardInfo: VerticalDiagramCardType
export let isLastCard: boolean
</script>

<div class="main" bind:this={cardInfo.el}>
    <div class="seperator-wrapper">
        <div 
            class="icon-wrapper" 
            class:active={cardInfo.active}
        >
            <svelte:component this={cardInfo.icon} />
        </div>
        {#if !isLastCard}
            <div class="seperator" class:active={cardInfo.separatorActive}/>
        {/if}
    </div>
    <div class="content-wrapper">
        <div class="heading">
            {cardInfo.heading}
        </div>
        <div class="description">
            {cardInfo.description}
        </div>
    </div>
</div>

<style lang="stylus">
@import "variables"

.main
    display flex
    padding 10px 20px

.seperator-wrapper
    margin-right 50px
    display flex
    flex-direction column

.icon-wrapper
    display flex
    align-items center
    justify-content center
    background-color rgba(73, 44, 156, 0.15)
    font-size 34px
    padding 10px
    height 48px
    width 48px
    color $brand
    border-radius 40px
    margin-bottom 15px
    &.active
        background-color $brand
        color white

.seperator
    width 4px
    background-color rgba(73, 44, 156, 0.15)
    margin 0 auto
    overflow hidden
    height 0
    // animation for seperator height
    height 100%
    &::after
        content ""
        display inline-block
        height 0
        width 100%
        background-color $brand
        transition height 0.5s ease-in-out
    &.active
        &::after
            height 100%

.content-wrapper
    padding 10px

.heading
    font-size 32px
    line-height 34px
    font-weight 600
    margin-bottom 24px

.description
    font-size 22px
    margin-bottom 40px

</style>