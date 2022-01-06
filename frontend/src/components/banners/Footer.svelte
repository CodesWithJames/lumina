<script lang="ts">
import ArrowRight from "@icons/ArrowRight.svelte"
import Lumina from "@components/custom-icons/LuminaEdge.svelte"
import MainButton from "@components/inputs/MainButton.svelte"
import Youtube from "@icons/Youtube.svelte"
import Twitter from "@icons/Twitter.svelte"
import Discord from "@icons/Discord.svelte"
import Facebook from "@icons/Facebook.svelte"
import ChevronRight from "svelte-material-icons/ChevronRight.svelte";
import TikTok from '@components/custom-icons/TikTok.svelte';

</script>

<footer>
    <div class="inner-wrapper call-to-action">
        <div class="cta-text">
            Join the early adopters building capabilities and help shape our future.
            <a href="/call-to-action" class="cta-link">
                <span class="cta-anchor-text">
                    Get involved.
                </span>
                <span class="cta-anchor-icon">
                    <ArrowRight />
                </span>
            </a>
        </div>
    </div>
    <div class="divider" />
    <div class="inner-wrapper second">
        <div class="lumina">
            <div class="lumina-heading">
                <div class="lumina-icon">
                    <Lumina />
                </div>
                <div class="lumina-text">
                    Lumina
                </div>
            </div>
            <div class="block-heading">
                The modern solution to governance
            </div>
        </div>
        <div class="links">
            <div class="block-heading">
                Quick Links
            </div>
            <div class="links-wrapper">
                <a href="/" class="link">Home</a>
                <a href="/hub" class="link">Hub</a>
                <a href="/my-lumina" class="link">myLumina</a>
                <a href="knowledge-center" class="link">Knowledge Center</a>
                <a href="discord" class="link">Discord</a>
                <a href="media" class="link">Media</a>
            </div>
        </div>
        <div class="become-citizen">
            <div class="block-heading">
                Become a citizen
            </div>
            <div class="citizen-description">
                Become a citizen of Lumina today!
            </div>
            <MainButton
                right_icon={ChevronRight}
                inline={true}
                gradient={true}>
                Get involved
            </MainButton>
        </div>
    </div>
    <div class="bottom">
        <div class="inner-wrapper">
            <div class="authorisation">
                Authorised by the Lumina Autonomous Zone
            </div>
            <nav class="social-media-icons">
                <a href="https://www.twitter.com/LuminaGov/" class="social-media-icon"><Twitter/></a>
                <a href="https://discord.gg/r4vNcUKktT" class="social-media-icon"><Discord/></a>
                <a href="https://www.tiktok.com/@luminagov/" class="social-media-icon"><TikTok/></a>
                <a href="https://www.facebook.com/luminagov" class="social-media-icon"><Facebook/></a>
            </nav>
        </div>
    </div>
</footer>

<style lang="stylus">
@import "variables"

footer
    background-color #EDEDEC

.inner-wrapper
    padding 60px 20px
    max-width 1200px
    width 100%
    margin 0 auto

.call-to-action
    font-size 40px
    font-weight 600
    text-align center
    display inline-flex
    align-items center

.cta-link
    color $brand

.cta-anchor-text
    margin-right 5px
    font-weight 1000

.divider
    background-color $dark
    opacity 0.15
    height 1px
    width 100%

.inner-wrapper.second
    display grid
    grid-auto-columns 1fr
    grid-auto-flow column

.lumina-heading
    display flex
    margin-bottom 16px

.lumina-icon
    font-size 48px
    height 48px
    margin auto 0

.lumina-text
    font-size 22px
    font-weight 600
    margin auto 0
    margin-left 18px

.block-heading
    max-width 225px
    text-transform uppercase
    font-weight 600
    opacity 0.35

.links-wrapper
    display grid
    grid-template-columns 1fr 1fr
    grid-gap 8px 40px
    margin-top 8px

.link
    color $dark
    text-decoration none
    font-weight 600
    display inline-block

.citizen-description
    font-weight 500
    margin 8px 0
    max-width 238px

.bottom
    background-color #E5E5E5
    padding 20px
    .inner-wrapper
        padding 8px 20px
        display flex
        justify-content space-between

.authorisation
    margin auto 0
    opacity 0.3
    text-transform uppercase
    font-weight 600
    font-size 16px

.social-media-icons
    display flex
    gap 10px

.social-media-icon
    color $dark
    padding 4px
    font-size 28px
    &:hover
        color $brand
</style>

