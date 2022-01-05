<script lang="ts">
import Discord from "@icons/Discord.svelte"
import MainButton from '@components/inputs/MainButton.svelte'
import RightChevron from '@icons/ChevronRight.svelte'
</script>

<div class="banner">
    <div class="banner-inner">
        <div class="left">
            <div class="icon">
                <Discord />
            </div>
            <div class="text-wrapper">
                <div class="title">
                    Got questions? Join the discussion
                </div>
                <div class="subtitle">
                    Join our Discord server.
                </div>
            </div>
        </div>

        <MainButton
            left_icon={Discord}
            dark={true}
            right_icon={RightChevron}>
            Join Server
        </MainButton>

    </div>
</div>

<style lang="stylus">

.banner
    background #384192
    padding 40px 20px
    color #fff

.banner-inner
    max-width 800px
    width 100%
    margin 0 auto
    display flex
    justify-content space-between

.left
    display flex
    flex-direction row
    align-items center
    gap 20px

.icon
    font-size 32px

.title
    font-size 18px
    font-weight 600

.subtitle
    font-size 16px
    opacity 0.5
</style>