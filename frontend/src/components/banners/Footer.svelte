<script lang="ts">
import ArrowRight from "@icons/ArrowRight.svelte"
import Lumina from "@components/custom-icons/LuminaEdge.svelte"
import MainButton from "@components/inputs/MainButton.svelte"
import Youtube from "@icons/Youtube.svelte"
import Twitter from "@icons/Twitter.svelte"
import Discord from "@icons/Discord.svelte"
import Facebook from "@icons/Facebook.svelte"
import ChevronRight from "svelte-material-icons/ChevronRight.svelte";

</script>

<footer>
    <div class="container top">
        <div class="inner-wrapper first">
            <div class="call-to-action one">
                Join the early adopters building capabilities
            </div>
            <div class="call-to-action two">
                <div class="inner-call-to-action-two">
                    and help shape our future.
                </div>
                <a href="" class="cta-link">
                    <div class="cta-anchor-text">
                        Get involved.
                    </div>
                    <div class="cta-anchor-icon">
                        <ArrowRight />
                    </div>

                </a>
            </div>
        </div>
    </div>
    <div class="divider" />
    <div class="container middle">
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
                    <a href="" class="link">Home</a>
                    <a href="" class="link">Hub</a>
                    <a href="" class="link">myLumina</a>
                    <a href="" class="link">Knowledge Center</a>
                    <a href="" class="link">Discord</a>
                    <a href="" class="link">Media</a>
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
                    branded={true}>
                    Get involved
                </MainButton>
            </div>
        </div>
    </div>
    <div class="container bottom">
        <div class="inner-wrapper last">
            <div class="authorisation">
                Authorised by the lumina government
            </div>
            <div class="social-media-icons">
                <a href="" class="social-media-icon"><Youtube/></a>
                <a href="" class="social-media-icon"><Twitter/></a>
                <a href="" class="social-media-icon"><Discord/></a>
                <a href="" class="social-media-icon"><Facebook/></a>
            </div>
        </div>
    </div>
</footer>

<style lang="stylus">
@import "variables"

footer
    background-color #EDEDEC

.container
    padding 0 156px

a 
    display flex

.inner-wrapper
    padding 80px 60px

.inner-wrapper.first

.call-to-action
    font-size 40px
    font-weight 600
    text-align center

.call-to-action.two
    display flex
    justify-content center

.cta-link
    color $brand

.cta-anchor-text
    margin-right 5px

.inner-call-to-action-two
    text-align center
    margin-right 20px

.divider
    z-index 3
    background-color $dark
    opacity 0.15
    height 1px
    width 100%

.inner-wrapper.second
    padding 60px 20px
    display flex
    justify-content space-between

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

.citizen-description
    font-weight 500
    margin 8px 0
    max-width 238px

.container.bottom
    background-color #E5E5E5
    padding 19px 156px

.inner-wrapper.last
    display flex
    padding 0
    justify-content space-between

.authorisation
    margin auto 0
    opacity 0.2
    text-transform uppercase
    font-weight 600
    font-size 16px

.social-media-icons
    display flex

.social-media-icon
    color $dark
    margin-right 15px
    font-size 28px
</style>

