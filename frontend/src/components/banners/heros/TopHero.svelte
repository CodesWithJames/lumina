<script lang="ts">
import MainButton from "@components/inputs/MainButton.svelte"
import ChevronRight from "@icons/ChevronRight.svelte"
import type { HeroType } from "@components/banners/heros/hero_types"

export let heroInfo: HeroType

</script>

<div>
    <h3 class="subtitle">{heroInfo.subtitle}</h3>
    <h1>
        <div>
            {heroInfo.titleExcludingLastWord}
        </div>
        <div>
            <span class="gradient-text">{heroInfo.highlightedWord}</span>
        </div>
    </h1>
    <div class="hero-description">
        {#each heroInfo.descriptionParagraphs as paragraph}
            <p>{paragraph}</p>
        {/each}
    </div>
    <div>
        <div class="buttons-wrapper">
            {#each heroInfo.buttons as button}
                <div class="button">
                    <MainButton
                        href={button.link}
                        right_icon={button.rightIcon}
                        inline={true}
                        gradient={button.gradient}
                        branded={button.branded}
                    >
                    {button.text}
                    </MainButton>
                </div>
            {/each}
        </div>
    </div>
</div>
<div class="hero-image-wrapper">
    <img class="hero-image" src={heroInfo.img} alt="a city skyline">
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

.buttons-wrapper
    display flex

.button
    margin auto 0
    margin-right 20px
</style>