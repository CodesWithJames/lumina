<script lang="ts">
    import Lumina from "$components/icons/LuminaEdge.svelte"
</script>
<a href="/" class="logo">
    <div class="symbol">
        <Lumina/>
    </div>
    <div class="brand">
        LUMINA
    </div>
</a>
<style lang="stylus">
@import 'variables'

.logo
    display flex
    align-items: center
    gap 14px
    text-decoration none
    color white

.symbol
    font-size 40px
    display inline-flex
    color $brand_light

.brand
    font-size 22px
    font-weight 600
</style>