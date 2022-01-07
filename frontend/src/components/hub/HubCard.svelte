
<script lang="ts">
import type {HubType} from "@components/hub/hub_type"
import ArrowRight from "svelte-material-icons/ArrowRight.svelte"
export let info: HubType
</script>

<a href="/" class="main">
    <div class="header">
        <div class="tag" style="background-color: {info.tag.color};">
            {info.tag.text}
        </div>
        <div class="arrow">
            <svelte:component this={ArrowRight}/>

        </div>
        <div class="title">
            <div class="icon">
                <svelte:component this={info.title.icon}/>
            </div>
            <div class="heading">
                {info.title.text}
            </div>
        </div>
        <p>
            {info.description}
        </p>
    </div>
</a>
