<script lang="ts">
import VerticalDiagramCard from "@components/general_components/vertical_linked_diagram/VerticalDiagramCard.svelte"
import type { VerticalDiagramCardType } from "@components/general_components/vertical_linked_diagram/vertical_diagram_types"
export let diagramSections: VerticalDiagramCardType[]

</script>

{#each diagramSections as cardInfo}
    <VerticalDiagramCard bind:cardInfo  />
{/each}