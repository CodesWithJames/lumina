<script lang="ts">
    export let color: 'red' | 'brand' | '' | 'blue' = 'brand';
</script>

<span class="tag {color}">
    <slot />
</span>
<style lang="stylus">
@import 'variables'

.tag
    padding 4px 8px
    line-height 1
    font-weight 600
    display inline-flex
    border-radius: 4px

    color white
    background transparify(white, 16%)

    &.brand
        color $brand
        background transparify($brand, 16%)

    &.red
        color $alert_red
        background transparify($alert_red, 16%)

    &.blue
        color $alert_blue
        background transparify($alert_blue, 16%)
</style>
