<script lang="ts">
import Lumina from "@components/custom-icons/LuminaEdge.svelte"
import Hub from '@components/custom-icons/Hub.svelte'
import Shop from '@icons/TagOutline.svelte'
import ShieldLock from '@icons/ShieldLock.svelte'
import MainButton from "@components/inputs/MainButton.svelte"
import ChevronRight from "svelte-material-icons/ChevronRight.svelte"
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
                <a href="/" class="link">
                    <div class="icon">
                        <Hub/>
                    </div>
                    <div class="name">
                        Hub
                    </div>
                </a>

                <a href="/shop" class="link">
                    <div class="icon">
                        <Shop/>
                    </div>
                    <div class="name">
                        Shop
                    </div>
                </a>
            </nav>
        </div>
        <div class="right">
            <nav>
                <a href="/login" class="link">
                    <div class="icon">
                        <ShieldLock/>
                    </div>
                    <div class="name">
                        Log In
                    </div>
                </a>
                <MainButton
                    href="/onboarding"
                    gradient={true}
                    right_icon={ChevronRight}
                >
                    Start
                </MainButton>
            </nav>
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
    gap 40px

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

.link
    display: flex;
    align-items: center;
    gap 8px
    text-decoration none
    color $dark
    
.symbol
    font-size 40px
    display inline-flex

.brand
    font-size 22px
    font-weight 600

.icon
    font-size 24px
    display inline-flex

.name
    font-size 18px
    font-weight 600

nav
    display: flex;
    align-items: center;
    gap 15px
</style>