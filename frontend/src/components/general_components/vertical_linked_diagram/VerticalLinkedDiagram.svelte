<script lang="ts">
import VerticalDiagramCard from "@components/general_components/vertical_linked_diagram/VerticalDiagramCard.svelte"
import type { VerticalDiagramCardType } from "@components/general_components/vertical_linked_diagram/vertical_diagram_types"
export let diagramSections: VerticalDiagramCardType[]

</script>

<div class="wrapper">
    {#each diagramSections as cardInfo, i}
        <div id="diagramCard {i}" class="card-wrapper">
            <VerticalDiagramCard bind:cardInfo  />
        </div>
    {/each}
</div>

<style lang="stylus">
.wrapper
    margin 0 auto
    max-width 485px

</style>