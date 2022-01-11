<script lang="ts">
import MainButton from "@components/inputs/MainButton.svelte"
import ChevronRight from "@icons/ChevronRight.svelte"

</script>

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
    <div class="hero-description">
        <p>
            Welcome to Lumina, a social experiment with the goals of setting up 
            a new innovative and environmentally sustainable city.
        </p>
        <p>
            Lumina proposes to set up a Autonomous Zone, 
            which is a city with it's own government, laws, and currency.
        </p>
    </div>
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

<style lang="stylus">
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

.hero-description
    font-size 20px

.hero-image-wrapper
    position relative
    display flex
    align-items center

.hero-image
    position absolute
    left 0
</style>