<script lang="ts">
import NewsCard from "$components/pages/news/NewsCard.svelte"
import type { NewsCardType } from "$components/pages/news/news_types"
import Button from "$lib/buttons/Button.svelte"
import ChevronRight from "$icons/ChevronRight.svelte"

export let featuredStories: NewsCardType[]

</script>

<div class="main">
    <div class="inner-wrapper">
        <div class="left">
            <h2 class="heading">
                Recent News
            </h2>
            <div class="subtitle">
                The latest official news, updates and advice from the Lumina government.
            </div>
            <div class="button">
                <Button right_icon={ChevronRight} style="translucent">
                    More News
                </Button>
            </div>
        </div>
        <div class="right">
            {#each featuredStories as newsInfo, i}
                <NewsCard bind:newsInfo />
            {/each}
        </div>
    </div>
</div>

<style lang="stylus">
.main
    display flex

.inner-wrapper
    display flex
    margin 0 auto
    padding 100px 0
    max-width 1200px
    width 100%

.left
    width 100%
    max-width 430px
    margin-right 190px

.heading
    font-size 40px
    line-height 60px
    font-weight 600
    padding 0
    margin 0
    margin-top 10px

.subtitle
    font-size 20px
    line-height 30px
    font-weight 400
    margin-bottom 12px

.button
    max-width 170px
    width 100%

.right
    max-width 580px
    width 100%
    gap 16px
    display flex
    flex-direction column


</style>