<script lang="ts">
import type { VerticalDiagramCardType } from "@components/general_components/vertical_linked_diagram/vertical_diagram_types"
import { tick } from "svelte"
export let cardInfo: VerticalDiagramCardType

let main
let seperatorHeight = 0
async function getSeperatorHeight() {
    await tick
    let cardHeight = main.offsetHeight
    seperatorHeight = cardHeight - 48 - 10
}
getSeperatorHeight()

</script>

<div class="main" bind:this={main}>
    <div class="seperator-wrapper">
        <div 
            class="icon-wrapper" 
            style={(cardInfo.passedOnScroll) ? "color: white; background-color: #492C9C;" : ""}
        >
            <svelte:component this={cardInfo.icon} />
        </div>
        <div 
            class="seperator" 
            style="height: {seperatorHeight}px; {(cardInfo.passedOnScroll) ? "background-color: #492C9C;" : ""}" 
        />
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

.seperator-wrapper
    margin-right 15px

.icon-wrapper
    display flex
    align-items center
    justify-content center
    background-color rgba(73, 44, 156, 0.15)
    font-size 24px
    padding 8px
    height 40px
    width 40px
    color $brand
    border-radius 40px
    margin-bottom 8px

.seperator
    width 4px
    background-color rgba(73, 44, 156, 0.15)
    margin 0 auto
    overflow hidden

.heading
    font-size 22px
    line-height 34px
    font-weight 600

.description
    font-size 20px
    line-height 24px
    margin-bottom 40px

</style>