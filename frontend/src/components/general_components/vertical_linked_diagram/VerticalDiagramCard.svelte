<script lang="ts">
import type { VerticalDiagramCardType } from "@components/general_components/vertical_linked_diagram/vertical_diagram_types"
export let cardInfo: VerticalDiagramCardType
</script>

<div class="main">
    <div class="seperator-wrapper">
        <div class="icon">
            <svelte:component this={cardInfo.icon} />
        </div>
        <div class="seperator" />
    </div>
    <div class="content-wrapper">
        <div class="heading">
            {cardInfo.heading}
        </div>
        <div class="description">
            {cardInfo.description}
        </div>
    </div>
</div>