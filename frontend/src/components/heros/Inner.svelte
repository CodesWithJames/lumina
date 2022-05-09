<script lang="ts">
export let max_width = 1200;
</script>
<div class="inner" style="
    --max-width: {max_width}px;
">
    <slot/>
</div>
<style lang="stylus">

.inner
    max-width var(--max-width)
    margin 0 auto
</style>