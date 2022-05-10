<script lang="ts">
import HubCard from "$components/hub/HubCard.svelte"

import Hub from "$lib/icons/Hub.svelte"
import { three_wide, two_wide } from './hub_data';
</script>

<div class="hub-wrapper">
    <div class="title hub">
        <div class="hub-icon">
            <svelte:component this={Hub} />
        </div>
        <h2 class="hub-heading">
            Lumina Hub
        </h2>
    </div>
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

<style lang="stylus">
.hub-wrapper
    gap 20px
    display flex
    flex-direction column

.title.hub
    display flex
    gap 20px

.hub-icon
    font-size 46px
    display inline-flex

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