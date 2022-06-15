<script lang="ts">
import IconButton from '$lib/buttons/IconButton.svelte';
import Apps from '$icons/Apps.svelte';
import Lumina from '$lib/icons/Lumina.svelte';
import BellOutline from '$icons/BellOutline.svelte';
import ProfileButton from '$lib/buttons/ProfileButton.svelte';
import NotificationsPopout from '$components/app/NotificationsPopout.svelte';
import AccountPopout from '$components/app/AccountPopout.svelte';
import { click_outside } from '$utils/click_outside';

enum Dropdown {
    Notifications,
    Account,
}

let dropdown: Dropdown | null = null;

let dropdowns = {
    [Dropdown.Notifications]: NotificationsPopout,
    [Dropdown.Account]: AccountPopout
}

function toggle(toggling: Dropdown) {
    if (toggling === dropdown) {
        dropdown = null;
    } else {
        dropdown = toggling;
    }
}

</script>
<div class="sidebar">
    <div class="section">
        <IconButton href="/" icon={Lumina} opacity={false}/>
        <div class="div"/>
        <IconButton href="/dashboard" icon={Apps}/>
    </div>
    <div class="section">
        <IconButton
            icon={BellOutline}
            on:click={() => toggle(Dropdown.Notifications)}/>
        <div class="div"/>
        <ProfileButton
            on:click={() => toggle(Dropdown.Account)}/>
    </div>
    {#if dropdown !== null}
        <div class="out-of-sidebar" use:click_outside={() => dropdown = null}>
            <svelte:component this={dropdowns[dropdown]}/>
        </div>
    {/if}
</div>
<style lang="stylus">
@import 'variables'

.sidebar
    display flex
    background transparify(white, 12%)
    width 60px
    flex-direction column
    padding 8px 0
    justify-content space-between
    position relative

.div
    height 1px
    width 50%
    background transparify(white, 12%)

.section
    display flex
    justify-content center
    flex-direction column
    gap 8px
    align-items center

.out-of-sidebar
    max-width 300px
    width 100vw
    position absolute
    bottom 16px
    left calc(100% \+ 16px)

</style>