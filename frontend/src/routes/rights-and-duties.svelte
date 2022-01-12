<script lang="ts">
import TopHero from "@components/banners/heros/TopHero.svelte"
import RightsIntroBlock from "@components/rights_and_duties/RightsIntroBlock.svelte"

let heroInfo = {
    titleExcludingLastWord: "rights and",
    highlightedWord: "duties",
    subtitle: "A core difference",
    descriptionParagraphs: [
        "Most governments give their citizens rights, but no duties.",
        "For example, Lumina believe that all citizens should have the right to vote on laws.",
        "However, for this to work, citizens have the duty of being an intelligent and independent free-thinker.",
        "For our society to function, it is your duty to fulfill your duties."
    ],
    buttons: [
        {
            text: "Read your rights", 
            link: "/" 
        },
        {
            text: "Become a citizen", 
            link: "/" 
        },
    ],
    img: "/images/tank_man.png"
}

</script>

<div class="hero-wrapper">
    <div class="inner-hero main-hero-layout">
        <TopHero bind:heroInfo />
    </div>
    <div class="section-wrapper">
        <RightsIntroBlock />
    </div>
</div>

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

.section-wrapper
    display flex
    align-items center
    border-top 1px solid #E6E6E6
    border-bottom 1px solid #E6E6E6
</style>