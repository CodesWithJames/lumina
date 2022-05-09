<script lang="ts">
import HubCard from "$components/hub/HubCard.svelte"

import Hub from "$components/icons/Hub.svelte"
import { three_wide, two_wide } from './hub_data';
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
                {#each two_wide as info}
                    <HubCard bind:info/>
                {/each}
            </div>
            <div class="hub-cards-three-wide-wrapper">
                {#each three_wide as info}
                    <HubCard bind:info/>
                {/each}
            </div>
        </div>
    </div>
</div>

<style lang="stylus">
.hub-wrapper
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