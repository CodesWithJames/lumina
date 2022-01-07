<script lang="ts">
import type { SvelteComponentDev } from "svelte/internal"
import JobField from "@components/jobs/JobField.svelte"

import AccountGroup from "@icons/AccountGroup.svelte"
import Domain from "@icons/Domain.svelte"
import AccountOutline from "@icons/AccountOutline.svelte"
import Flash from "@icons/Flash.svelte"
import DiscordBanner from "@components/banners/DiscordBanner.svelte"
import MainButton from "@components/inputs/MainButton.svelte"
import ChevronRight from "@icons/ChevronRight.svelte"
import CodeBraces from "@icons/CodeBraces.svelte"
import Clock from "@icons/Clock.svelte"
import Wifi from "@icons/Wifi.svelte"

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
        color: "#5D35D5"
    },
    {
        icon: Domain,
        name: "Government Ministries",
        value: "15",
        color: "#4F63CE"
    },
    {
        icon: AccountOutline,
        name: "Government Staff",
        value: "36",
        color: "#4488C9"
    },
    {
        icon: Flash,
        name: "Renewable",
        value: "100%",
        color: "#34BFC1"
    }
]

</script>
<div class="hero-wrapper">
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
                Welcome to Lumina, a social experiment with the goals of setting up 
                a new innovative and environmentally sustainable city.
            </p>
            <p>
                Lumina proposes to set up a Autonomous Zone, 
                which is a city with it's own government, laws, and currency.
            </p>
            <div>
                <MainButton
                    href="/onboarding"
                    right_icon={ChevronRight}
                    inline={true}
                    gradient={true}
                >
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
</div>

<div class="job-positions">
    <div class="job-info">
        <h2 class="job-heading">
            Open positions
        </h2>
        <p class="job-description">
            We are a technology driven society comprised of principled and disciplined people bonded by deeply held mission: to create a modern solution to governance.
        </p>
        <div class="jobs-wrapper">
            
        </div>
    </div>
</div>

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

.hero-wrapper
    // min height instead of height because some devices have a smaller height
    // header is 80px, so we need to subtract that from the height
    min-height calc(100vh \- 80px)
    display flex
    flex-direction column
    .main-hero-layout
        flex 1 // take up all available space

.gradient-text
    background linear-gradient(90deg, #5c32d4, #32c0c0);
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

.job-positions
    padding 80px 256px
    display flex

.job-info
    display flex
    flex-direction column
    align-items center
    max-width 600px
    margin 0 auto

.job-heading
    font-size 40px
    font-weight 600
    margin 0
    padding 0

.job-description
    opacity 0.6
    text-align center
    line-height 28px
</style>