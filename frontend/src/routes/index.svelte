<script lang="ts">
import type { bind, SvelteComponentDev } from "svelte/internal"
import UpcomingEvents from "@components/events/UpcomingEventsHero.svelte"
import NewsHero from "@components/news/NewsHero.svelte"
import DiscordBanner from "@components/banners/DiscordBanner.svelte"
import KnowledgeHubCard from "@components/hubs/HubsCard.svelte"
import StatsHero from "@components/stats/StatsBlock.svelte"
import HubHero from "@components/hub/HubHero.svelte"
import JobHero from "@components/jobs/JobHero.svelte"
import HomePageHero from "@components/banners/heros/TopHero.svelte"

let heroInfo = {
    titleExcludingLastWord: "city of the",
    highlightedWord: "future",
    descriptionOne: "Welcome to Lumina, a social experiment with the goals of setting up a new innovative and environmentally sustainable city.",
    descriptionTwo: "Lumina proposes to set up a Autonomous Zone, which is a city with it's own government, laws, and currency.",
    buttonOneText: "Become a citizen",
    buttonOneLink: "/onboarding",
    img: "/images/hero-image.svg"
}

</script>

<div class="hero-wrapper">
    <div class="inner-hero main-hero-layout">
        <HomePageHero bind:heroInfo />
    </div>
    <div class="inner-hero">
        <StatsHero />
    </div>
    <DiscordBanner/>
</div>
<HubHero />
<div class="inner-hero jobs-hero">
    <JobHero />
</div>
<div class="extra-info-wrapper">
    <UpcomingEvents />
    <div class="knowledge-hub-wrapper">
        <KnowledgeHubCard />
    </div>
</div>
<NewsHero />

<style lang="stylus">
.hero-wrapper
    // min height instead of height because some devices have a smaller height
    // header is 80px, so we need to subtract that from the height
    min-height calc(100vh \- 80px)
    display flex
    flex-direction column
    .main-hero-layout
        flex 1 // take up all available space

.inner-hero
    max-width 1200px
    width 100%
    padding-left 20px
    padding-right 20px
    margin 0 auto

.main-hero-layout
    display grid
    grid-template-columns 1fr 1fr
    grid-gap 40px
    align-items center
    padding-top 60px
    padding-bottom 40px

.jobs-hero
    padding-top 80px
    padding-bottom 80px

.extra-info-wrapper
    display grid
    grid-template-columns 1.2fr 1fr
    width: 100vw

.knowledge-hub-wrapper
    padding 40px
    background-color #EBEBEA
    padding-right 160px

</style>