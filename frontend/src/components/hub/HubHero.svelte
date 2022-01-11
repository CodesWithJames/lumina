<script lang="ts">
import type { HubType } from "@components/hub/hub_type"
import HubCard from "@components/hub/HubCard.svelte"

import Hub from "@components/custom-icons/Hub.svelte"

export let twoWideHubCards: HubType[]
export let threeWideHubCards: HubType[]
</script>

<div class="hub-wrapper">
    <div class="hub-inner-wrapper">
        <div class="title hub">
            <div class="hub-icon">
                <svelte:component this={Hub} />
            </div>
            <h2 class="hub-heading">
                Lumina Hub
            </h2>
        </div>
        <div class="hub-cards-wrapper">
            <div class="hub-cards-two-wide-wrapper">
                {#each twoWideHubCards as info}
                    <HubCard bind:info/>
                {/each}
            </div>
            <div class="hub-cards-three-wide-wrapper">
                {#each threeWideHubCards as info}
                    <HubCard bind:info/>
                {/each}
            </div>
        </div>
    </div>
</div>

<style lang="stylus">
.hub-wrapper 
    background-color #F3F3F2
    padding 60px 0

.hub-inner-wrapper
    max-width 1000px
    margin 0 auto

.title.hub
    display flex
    margin-bottom 20px

.hub-icon
    font-size 46px
    margin-right 10px
    height 46px
    width 46px

.hub-heading
    margin 0
    padding 0
    font-size 40px
    margin auto 0

.hub-cards-two-wide-wrapper
    display grid
    grid-template-columns 1fr 1fr
    grid-gap 20px
    margin-bottom 20px

.hub-cards-three-wide-wrapper
    display grid
    grid-template-columns 1fr 1fr 1fr
    grid-gap 20px
    margin-bottom 20px

</style>