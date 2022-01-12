<script lang="ts">
import InfoCard from "@components/jobs/InfoCard.svelte"
import InfoBox from "@components/small_components/InfoBox.svelte"
import ScrollableDisplay from "@components/small_components/ScrollableDisplay.svelte"
import type { HeaderInfoType } from "@components/rights_and_duties/rights_duties_types"
import type { ScrollableDisplayCardType } from "@components/small_components/small_components_types"

export let headerInfo: HeaderInfoType
export let displayInfo: ScrollableDisplayCardType[]

</script>

<div class="main">
    <div class="header">
        <div class="header-icon">
            <svelte:component this={headerInfo.icon} />
        </div>
        <h2>
            {headerInfo.heading}
        </h2>
    </div>
    <div class="subtitle">
        {headerInfo.subtitle}
    </div>
    <ScrollableDisplay bind:displayInfo />
</div>

<style lang="stylus">
@import "variables"

.main
    display flex
    flex-direction column

.header
    display flex
    margin 0 auto

.header-icon
    font-size 40px
    height 40px
    width 40px
    color $brand
    margin auto 0
    margin-right 10px

h2
    margin 0
    padding 0
    font-size 30px
    font-weight 600
    margin auto 0

.subtitle
    font-size 18px
    opacity 0.3
    margin 0 auto
    margin-top 10px
    margin-bottom 60px

</style>