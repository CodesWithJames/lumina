<script lang="ts">
import Lumina from "@components/custom-icons/lumina-edge.svelte"
import Hub from '@components/custom-icons/hub.svelte'
import Shop from '@icons/TagOutline.svelte'
</script>

<header>
    <div class="inner-header">
        <div class="left">
            <a href="/" class="logo">
                <div class="symbol">
                    <Lumina/>
                </div>
                <div class="brand">
                    Lumina
                </div>
            </a>
            <nav>
                <a href="/">
                    <div class="icon">
                        <Hub/>
                    </div>
                    <div class="name">
                        Hub
                    </div>
                </a>

                <a href="/shop">
                    <div class="icon">

                    </div>
                    <div class="name">

                    </div>
                </a>
            </nav>
        </div>
        <div class="right">
                Login Start
        </div>
    </div>
</header>

<style lang="stylus">
@import 'variables'

.inner-header
    max-width 1200px
    display: flex;
    flex-direction: row;
    justify-content: space-between;
    align-items: center;
    padding: 0px 20px;
    width: 1200px;
    height: 80px;
    margin 0 auto;


.left
    display: flex;
    flex-direction: row;
    align-items: center;


.right
    display: flex;
    flex-direction: row;
    align-items: center;
.logo
    display: flex;
    align-items: center;
    gap 14px
    text-decoration none
    color $dark

.symbol
    font-size 40px
    display inline-flex

.brand
    font-size 22px
    font-weight 600

</style>