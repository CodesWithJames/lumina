<script>
export let translucent = false
</script>
<div class="hero" class:translucent>
    <slot/>
</div>
<style lang="stylus">
@import 'variables'
.hero
    padding 48px 20px
    &.translucent
        background transparify(white, 4%)
</style>