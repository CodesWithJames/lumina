<script lang="ts">
import ScaleBalance from "@icons/ScaleBalance.svelte"
import Pillar from "@icons/Pillar.svelte"
import PassportBiometric from "@icons/PassportBiometric.svelte"
import AccountOutline from "@icons/AccountOutline.svelte"
import Telescope from "@icons/Telescope.svelte"
import BadgeAccountHorizontal from "@icons/AccountBadgeHorizontal.svelte"
import Hub from "@components/custom-icons/Hub.svelte"
import HubCard from "@components/hub/HubCard.svelte"

let twoWideHubCards = [
    {
        tag: {
            text: "Info",
            color: "#2570B6",
            
        },
        title: {
            icon: ScaleBalance,
            text: "Rights & Duties",
        },
        description: "Learn about your rights and duties required for our society to function.",
        greyedOut: false
    },
    {
        tag: {
            text: "Info",
            color: "#2570B6",
            
        },
        title: {
            icon: Pillar,
            text: "Principles & Values",
        },
        description: "Our mission, principles and values define the emotional, moral, and spiritual core of our society.",
        greyedOut: false
    },
    {
        tag: {
            text: "Service",
            color: "#492C9C",
            
        },
        title: {
            icon: PassportBiometric,
            text: "Citizenship",
        },
        description: "Find out the benefits of becoming a citizen in Lumina. Get involved, along with thousands others today.",
        greyedOut: false
    },
    {
        tag: {
            text: "Service",
            color: "#492C9C",
            
        },
        title: {
            icon: AccountOutline,
            text: "myLumina",
        },
        description: "Lumina's cloud government portal that lets citizens access government services.",
        greyedOut: false
    },
    {
        tag: {
            text: "Info",
            color: "#2570B6",
            
        },
        title: {
            icon: Telescope,
            text: "Our Vision",
        },
        description: "Learn about your rights and duties required for our society to function.",
        greyedOut: false
    },
    {
        tag: {
            text: "Service",
            color: "#2570B6",
            
        },
        title: {
            icon: BadgeAccountHorizontal,
            text: "Media & Press",
        },
        description: "Are you a member of the press looking to speak to a Luminar representative? Contact us here.",
        greyedOut: false
    }
]

let threeWideHubCards = [
    {
        tag: {
            text: "Coming Soon",
            color: "#B4B4B4",
            
        },
        title: {
            icon: ScaleBalance,
            text: "Legislation",
        },
        description: "Learn about your rights and duties required for our society to function.",
        greyedOut: true
    },
    {
        tag: {
            text: "Coming Soon",
            color: "#B4B4B4",
            
        },
        title: {
            icon: ScaleBalance,
            text: "Legislation",
        },
        description: "Learn about your rights and duties required for our society to function.",
        greyedOut: true
    },
    {
        tag: {
            text: "Coming Soon",
            color: "#B4B4B4",
            
        },
        title: {
            icon: ScaleBalance,
            text: "Legislation",
        },
        description: "Learn about your rights and duties required for our society to function.",
        greyedOut: true
    },
    {
        tag: {
            text: "Coming Soon",
            color: "#B4B4B4",
            
        },
        title: {
            icon: ScaleBalance,
            text: "Legislation",
        },
        description: "Learn about your rights and duties required for our society to function.",
        greyedOut: true
    },
    {
        tag: {
            text: "Coming Soon",
            color: "#B4B4B4",
            
        },
        title: {
            icon: ScaleBalance,
            text: "Legislation",
        },
        description: "Learn about your rights and duties required for our society to function.",
        greyedOut: true
    },
    {
        tag: {
            text: "Coming Soon",
            color: "#B4B4B4",
            
        },
        title: {
            icon: ScaleBalance,
            text: "Legislation",
        },
        description: "Learn about your rights and duties required for our society to function.",
        greyedOut: true
    },
    {
        tag: {
            text: "Coming Soon",
            color: "#B4B4B4",
            
        },
        title: {
            icon: ScaleBalance,
            text: "Legislation",
        },
        description: "Learn about your rights and duties required for our society to function.",
        greyedOut: true
    },
    {
        tag: {
            text: "Coming Soon",
            color: "#B4B4B4",
            
        },
        title: {
            icon: ScaleBalance,
            text: "Legislation",
        },
        description: "Learn about your rights and duties required for our society to function.",
        greyedOut: true
    },
    {
        tag: {
            text: "Coming Soon",
            color: "#B4B4B4",
            
        },
        title: {
            icon: ScaleBalance,
            text: "Legislation",
        },
        description: "Learn about your rights and duties required for our society to function.",
        greyedOut: true
    },
]

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