
<script>
import Header from '@components/banners/Header.svelte'
import Footer from '@components/banners/Footer.svelte'
</script>

<svelte:head>
    <link
        href="https://fonts.googleapis.com/css2?family=Prompt:wght@300;400;500;600;700&display=swap"
        rel="stylesheet"
        defer
    />
</svelte:head>


<main>
    <Header/>
    <slot />
    <Footer/>
</main>

<style lang="stylus">

:global
    @import 'normalise'
    @import 'variables'

    html, body, #svelte
        display flex
        flex-direction row
        min-height 100%
        flex 1
        color $dark
        font-family 'Prompt'

    h1, h2, h3, h4, h5, h6
        font-weight 500

    html, body
        overflow-x hidden

    main
        width 100%
</style>
