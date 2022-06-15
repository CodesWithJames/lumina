<script lang="ts">
import Hub from '$lib/icons/Hub.svelte'
import Shop from '$icons/TagOutline.svelte'
import ShieldLock from '$icons/ShieldLock.svelte'
import ChevronRight from "svelte-material-icons/ChevronRight.svelte"
import Button from '$lib/buttons/Button.svelte';
import Logo from '$lib/icons/Logo.svelte'
</script>

<header>
    <div class="inner-header">
        <div class="left">
            <Logo/>
            <nav>
                <Button style="transparent" href="/" right_icon={Hub}>
                    Hub
                </Button>
                <Button style="transparent" href="/shop" right_icon={Shop}>
                    Shop
                </Button>
        </div>
        <nav class="right">
            <Button style="transparent" href="/signin" right_icon={ShieldLock}>
                Log In
            </Button>
            <Button href="/onboarding" right_icon={ChevronRight}>
                Start
            </Button>
        </nav>
    </div>
</header>

<style lang="stylus">
@import 'variables'

.inner-header
    max-width 1200px
    width 100%
    display flex
    flex-direction row
    justify-content space-between
    align-items center
    padding: 0px 20px
    height 64px
    margin 0 auto

.left
    display: flex;
    flex-direction: row;
    align-items: center;
    gap 40px

nav
    display: flex;
    align-items: center;
    gap 12px
</style>