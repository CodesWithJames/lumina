<script lang="ts">
import Sidebar from '$main/app/Sidebar.svelte';
import Global from "$templates/Global.svelte"
</script>
<Global>
    <div class="app">
        <Sidebar/>
        <div class="content">
            <slot/>
        </div>
    </div>
</Global>
<style lang="stylus">
.app
    display flex
    height 100%

.content
    flex 1
    overflow-y scroll
    height 100%
    display flex

</style>