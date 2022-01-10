<script lang="ts">
import type { NewsCardType } from "@components/news/news_types"
import ChevronRight from "@icons/ChevronRight.svelte"

export let newsInfo: NewsCardType

let dateSuffix = () => {
    if (newsInfo.date.dayAsNumber === 1){
        return "st"
    } else if (newsInfo.date.dayAsNumber === 2){
        return "nd"
    } else if (newsInfo.date.dayAsNumber === 3){
        return "rd"
    } else {
        return "th"
    }
}
</script>

<div class="main">
    <div class="left">
        <div class="header">
            <div class="tag" style="color: {newsInfo.tag.color}">
                {newsInfo.tag.text}
            </div>
            <div class="date">
                {newsInfo.date.dayAsNumber}{dateSuffix} {newsInfo.date.month} {newsInfo.date.year}
            </div>
        </div>
        <div class="title">
            {newsInfo.title}
        </div>
        <div class="author">
            <div class="by">
                By
            </div>
            <img class="author-img" src={newsInfo.author.img}>
            <div class="name">
                {newsInfo.author.name}
            </div>
        </div>
    </div>
    <div class="chevron-right">
        <svelte:component this={ChevronRight} />
    </div>
</div>