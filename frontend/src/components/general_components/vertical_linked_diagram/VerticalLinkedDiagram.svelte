<script lang="ts">
import VerticalDiagramCard from "$components/general_components/vertical_linked_diagram/VerticalDiagramCard.svelte"
import type { VerticalDiagramCardType } from "$components/general_components/vertical_linked_diagram/vertical_diagram_types"
export let diagramSections: VerticalDiagramCardType[]

</script>

<div class="wrapper">
    {#each diagramSections as cardInfo, i}
        <VerticalDiagramCard bind:cardInfo isLastCard={ (i === diagramSections.length - 1) ? true : false  }  />
    {/each}
</div>

<style lang="stylus">
.wrapper
    margin 0 auto
    margin-top 60px
    max-width 600px

</style>