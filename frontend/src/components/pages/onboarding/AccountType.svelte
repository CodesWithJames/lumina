<script lang="ts">
import ShieldAccount from '$icons/ShieldAccount.svelte'
import HumanGreetingVariant from '$icons/HumanGreetingVariant.svelte'
import HumanMaleFemaleChild from '$icons/HumanMaleFemaleChild.svelte'
import ChevronRight from '$icons/ChevronRight.svelte'

import { createEventDispatcher } from 'svelte'
import SelectCard from '$lib/inputs/SelectCard.svelte';
import Button from '$lib/buttons/Button.svelte';

let dispatch = createEventDispatcher();

let cards = [
    {
        title: "For myself",
        description: "One account. All of Lumina. Sign up to become a citizen here.",
        icon: HumanGreetingVariant,
        enabled: true
    },
    {
        title: "For my child",
        description: "Create an account for my child linked to my account",
        icon: HumanMaleFemaleChild,
        tag: "Coming Soon",
    }
]

let current = 0;
</script>
<div class="account-page">
    <div class="top-part">
        <h1>
            <span class="icon h1-icon">
                <ShieldAccount/>
            </span>
            Choose account type
        </h1>
        <div class="paras">
            <p>
                We’ll streamline your setup experience accordingly.
            </p>
            <p>
                This account can be used for your citizenship application later.
            </p>
        </div>
    </div>
    <div class="bottom-part">
        <div class="cards">
            {#each cards as card, i}
                <SelectCard info={card} bind:current i={i}/>
            {/each}
        </div>
        <Button
            on:click={() => dispatch('next')}
            right_icon={ChevronRight}>
            Next
        </Button>
    </div>
</div>


<style lang="stylus">
@import 'variables'

.account-page
    display flex
    flex-direction column
    align-items center
    gap 40px


.top-part
    display flex
    flex-direction column
    align-items center
    gap 12px


.bottom-part
    display: flex
    flex-direction: column
    gap: 32px


h1
    margin 0
    display flex
    gap 10px
    align-items: center
    font-weight 600

p
    opacity: 0.5
    margin: 0
    padding: 4px 0px
    text-align: center

.icon
    display: inline-flex

.h1-icon
    color $brand


.cards
    display: grid
    grid-auto-columns: 1fr
    grid-auto-flow: column
    gap: 32px
    max-width: 600px
</style>