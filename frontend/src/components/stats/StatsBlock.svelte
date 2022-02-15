<script lang="ts">
import type { StatType } from "@components/stats/stats_types"

export let stats: StatType[]

</script>

<div class="stats">
    {#each stats as stat}
        <div class="stat">
            <div class="stat-icon" style="color: {stat.color};">
                <svelte:component this={stat.icon} />
            </div>
            <div class="stat-value">
                <span>{stat.value}</span>
            </div>
            <div class="stat-name">
                <span>{stat.name}</span>
            </div>
        </div>
    {/each}
</div>

<style lang="stylus">
.stats
    display grid
    grid-auto-flow column
    grid-auto-columns 1fr
    grid-gap 20px
    padding 40px 0

.stat
    display flex
    flex-direction column
    align-items center

.stat-icon
    font-size 48px

.stat-value
    font-size 36px
    font-weight 600

.stat-name
    font-size 18px
    opacity 0.5

</style>