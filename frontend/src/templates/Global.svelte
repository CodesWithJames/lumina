<script>
    import ActionBar from '$main/ActionBar.svelte'
    import PageLoaderBar from '$main/PageLoaderBar.svelte'
</script>
<style lang="stylus">
:global
    @import 'normalise'
    @import 'variables'

    html
        display flex
        min-height 100%
        flex 1
    h1, h2, h3, h4, h5, h6
        font-weight 500

    a
        text-decoration none
        color $brand_light
        &:hover
            color lighten($brand_light, 10%)

    body
        background $dark_app
        color white
        flex 1
        min-height 100%
        font-family  'Prompt'
        border-top-left-radius 15px
        overflow-x hidden

</style>
<svelte:head>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <link href="https://fonts.googleapis.com/css2?family=Prompt:wght@300;400;500;600;700&display=swap" rel="stylesheet" defer>
</svelte:head>
<ActionBar/>
<PageLoaderBar/>
<slot/>