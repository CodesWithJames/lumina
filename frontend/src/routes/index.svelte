<script lang="ts">
import type { SvelteComponentDev } from 'svelte/internal';

import AccountGroup from '@icons/AccountGroup.svelte'
import Domain from '@icons/Domain.svelte'
import AccountOutline from '@icons/AccountOutline.svelte'
import Flash from '@icons/Flash.svelte'
import DiscordBanner from '@components/banners/DiscordBanner.svelte'
import MainButton from '@components/inputs/MainButton.svelte';
import ChevronRight from 'svelte-material-icons/ChevronRight.svelte';


type Stat = {
    value: string,
    name: string,
    icon: typeof SvelteComponentDev,
    color: string
}

let stats: Stat[] = [
    {
        icon: AccountGroup,
        name: "Citizens registered",
        value: "2251",
        color: "#2BE0CB"
    },
    {
        icon: Domain,
        name: "Government Ministries",
        value: "15",
        color: "#32B5ED"
    },
    {
        icon: AccountOutline,
        name: "Government Staff",
        value: "36",
        color: "#1E96F4"
    },
    {
        icon: Flash,
        name: "Renewable",
        value: "100%",
        color: "#026AFF"
    }
]

</script>

<div class="inner-hero main-hero-layout">
    <div>
        <h3 class="subtitle">Start a new city</h3>
        <h1>
            <div>
                city of the
            </div>
            <div>
                <span class="gradient-text">future</span>
            </div>
        </h1>
        <p>
            Welcome to Lumina, a social experiment with the goals of setting up a new innovative and environmentally sustainable city.
        </p>
        <p>
            Lumina proposes to set up a Autonomous Zone, which is a city with it's own government, laws, and currency.
        </p>
        <div>
            <MainButton
                href="/onboarding"
                right_icon={ChevronRight}
                inline={true}
                gradient={true}>
                Become a citizen
            </MainButton>
        </div>

    </div>
    <div class="hero-image-wrapper">
        <img class="hero-image" src="/images/hero-image.svg" alt="a city skyline">
    </div>
</div>
<div class="inner-hero">
    <div class="stats">
        {#each stats as stat}
            <div class="stat">
                <div class="stat-icon" style="color: {stat.color};">
                    <svelte:component this={stat.icon} />
                </div>
                <div class="stat-value">
                    <span>{stat.value}</span>
                </div>
                <div class="stat-name">
                    <span>{stat.name}</span>
                </div>
            </div>
        {/each}
    </div>
</div>
<DiscordBanner/>

<style lang="stylus">


// STATS SECTION
.stats
    display grid
    grid-auto-flow column
    grid-auto-columns 1fr
    grid-gap 20px
    padding 40px 0
.stat
    display flex
    flex-direction column
    align-items center
.stat-icon
    font-size 48px
.stat-value
    font-size 36px
    font-weight 600
.stat-name
    font-size 18px
    opacity 0.5

// HERO MAIN

.gradient-text
    background linear-gradient(90deg, #0066FF 0%, #2BE0CA 100%);
    -webkit-background-clip text
    -webkit-text-fill-color transparent

.subtitle
    font-size 20px
    text-transform uppercase
    font-weight 600
    opacity 0.3
    margin 0

h1
    font-size 80px
    font-weight 600
    margin 0

.inner-hero
    max-width 1200px
    width 100%
    padding-left 20px
    padding-right 20px
    margin 0 auto

.main-hero-layout
    display grid
    grid-template-columns 1fr 1fr
    grid-gap 20px
    align-items center
    padding-top 60px
    padding-bottom 40px

.hero-image-wrapper
    position relative
    display flex
    align-items center

.hero-image
    position absolute
    left 0

</style>