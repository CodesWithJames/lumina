<script lang="ts">
import type { StatType } from "@components/stats/stats_types"
import AccountGroup from "@icons/AccountGroup.svelte"
import Domain from "@icons/Domain.svelte"
import AccountOutline from "@icons/AccountOutline.svelte"
import Flash from "@icons/Flash.svelte"

let stats: StatType[] = [
    {
        icon: AccountGroup,
        name: "Citizens registered",
        value: "2251",
        color: "#5D35D5"
    },
    {
        icon: Domain,
        name: "Government Ministries",
        value: "15",
        color: "#4F63CE"
    },
    {
        icon: AccountOutline,
        name: "Government Staff",
        value: "36",
        color: "#4488C9"
    },
    {
        icon: Flash,
        name: "Renewable",
        value: "100%",
        color: "#34BFC1"
    }
]

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