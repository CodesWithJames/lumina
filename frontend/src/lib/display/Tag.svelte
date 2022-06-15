<script lang="ts">
    export let color: 'red' | 'brand' | '' | 'blue' | 'white' = 'brand';
</script>

<span class="tag {color}">
    <slot />
</span>
<style lang="stylus">
@import 'variables'

.tag
    padding 2px 4px
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
        color $red
        background transparify($red, 16%)

    &.blue
        color $blue
        background transparify($blue, 16%)

    &.white
        color white
        background transparify(white, 16%)
</style>
