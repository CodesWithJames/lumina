<script lang="ts">
import type { ScrollableDisplayCardType } from "@components/small_components/small_components_types"
import TagOutline from "svelte-material-icons/TagOutline.svelte"
export let displayInfo: ScrollableDisplayCardType[]

</script>

<div class="main">
    {#each displayInfo as card}
        <div class="card">
            <div class="left">
                <div class="header-icon">
                    <svelte:component this={card.icon}/>
                </div>
                <div class="type">
                    {card.type}
                </div>
                <div class="title">
                    {card.title}
                </div>
                <div class="description">
                    {card.description}
                </div>
            </div>
            <div class="right">
                {#each card.tags as tag}
                    <div class="tag">
                        <div class="question">
                            {tag.question}
                        </div>
                        <div class="answer" style="background-color: {tag.color};">
                            <div class="answer-icon">
                                <svelte:component this={tag.answerIcon}/>
                            </div>
                            <div class="answer-text">
                                {tag.answerText}
                            </div>
                        </div>
                    </div>
                {/each}
            </div>
        </div>
    {/each}
</div>

<style lang="stylus">
@import "variables"

.main
    border-radius 8px
    padding 40px
    background-color #F2F2F1

.header-icon
    font-size 40px
    height 40px
    color $brand

</style>