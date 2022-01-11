<script lang="ts">
import type { NewsCardType } from "@components/news/news_types"
import ArrowRight from "svelte-material-icons/ArrowRight.svelte"

export let newsInfo: NewsCardType

</script>

<div class="main">
    <a href="/">
        <div class="inner-wrapper">
            <div class="left">
                <div class="header">
                    <div class="tag" style="background-color: {newsInfo.tag.color}">
                        {newsInfo.tag.text}
                    </div>
                    <div class="text-with-opacity">
                        {newsInfo.date.dayAsNumber}{#if newsInfo.date.dayAsNumber === 1}st{:else if newsInfo.date.dayAsNumber === 2}nd{:else if newsInfo.date.dayAsNumber === 3}rd{:else}th{/if} 
                        {newsInfo.date.month} {newsInfo.date.year}
                    </div>
                </div>
                <div class="title">
                    {newsInfo.title}
                </div>
                <div class="author">
                    <div class="text-with-opacity">
                        By
                    </div>
                    <img class="author-img" src={newsInfo.author.img} alt="Author's DP">
                    <div class="text-with-opacity">
                        {newsInfo.author.name}
                    </div>
                </div>
            </div>
            <div class="arrow-right">
                <svelte:component this={ArrowRight} />
            </div>
        </div>
    </a>
</div>

<style lang="stylus">
@import "variables"

.main
    border-radius 5px
    padding 10px

.main:hover
    background-color rgba(45, 55, 68, 0.06)

a
    color $dark
    text-decoration none

.inner-wrapper
    display flex

.header
    display flex

.tag
    color white
    padding 4px
    border-radius 5px
    font-weight 500
    margin-right 15px

.text-with-opacity
    color $dark
    opacity 0.3
    margin auto 0
    font-weight 500

.title
    font-size 24px
    line-height 36px
    font-weight 700
    margin 10px 0

.author
    display flex

.author-img
    opacity 1
    height 27px
    width 27px
    margin 0 10px

.arrow-right
    margin auto 0
    margin-left auto
    font-size 30px
    height 30px
    color $brand

</style>