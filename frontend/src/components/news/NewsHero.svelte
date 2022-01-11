<script lang="ts">
import NewsCard from "@components/news/NewsCard.svelte"
import type { NewsCardType } from "@components/news/news_types"
import MainButton from "@components/inputs/MainButton.svelte"
import ChevronRight from "@icons/ChevronRight.svelte"

let featuredStories: NewsCardType[]  = [
    {
        tag: {
            text: "Article",
            color: "#492C9C"
        },
        date: {
            dayAsNumber: 4,
            month: "Jan",
            year: 2020
        },
        title: "Lumina hits 2000 citizens",
        author: {
            name: "Albert",
            img: "static/images/albert_dp.png"
        }
    },
    {
        tag: {
            text: "Article",
            color: "#492C9C"
        },
        date: {
            dayAsNumber: 3,
            month: "Jan",
            year: 2020
        },
        title: "Lumina hits 1000 citizens",
        author: {
            name: "Albert",
            img: "static/images/albert_dp.png"
        }
    }
]

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
                <MainButton right_icon={ChevronRight} gradient={true} >
                    More News
                </MainButton>
            </div>
        </div>
        <div class="right">
            {#each featuredStories as newsInfo, i}
                <NewsCard bind:newsInfo />
                {#if i !== featuredStories.length - 1}
                    <div class="divider" />
                {/if}
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

.divider
    margin 20px 0
    height 0px
    border-top 1px solid rgba(45, 55, 68, 0.1)

</style>