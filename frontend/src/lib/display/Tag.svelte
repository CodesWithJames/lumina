<script lang="ts">
    export let color: string = '';
</script>

<div class="tag {color}">
    <slot />
</div>
<style lang="stylus">
@import 'variables'

.tag
    padding 4px 8px
    background transparify(white, 10%)
    line-height 1
    font-weight 600
    display: inline-flex
    border-radius: 4px

.theme
    color $brand_light
    background transparify($brand_light, 10%)

</style>
