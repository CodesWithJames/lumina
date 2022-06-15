<script lang="ts">
import Card from '$lib/cards/Card.svelte';
import Settings from '$icons/Settings.svelte';
import Icon from '$lib/display/Icon.svelte';
import ExitToApp from '$icons/ExitToApp.svelte';
import Profile from '$lib/display/Profile.svelte';
import { session } from '$app/stores';

$: name = $session.user.first_name + ' ' + $session.user.last_name;
$: email = $session.user.email;

</script>
<Card
    reset_bg={true}>
    <div class="account">
        <Profile size=48px/>
        <div class="data">
            <span class="name">
                {name}
            </span>
            <span class="email">
                {email}
            </span>
        </div>
    </div>
    <div class="links">
        <div class="link">
            <Icon icon={Settings} size={20}/>
            <span>
                Account Settings
            </span>
        </div>
        <div class="link red">
            <Icon icon={ExitToApp} size={20}/>
            <span>
                Log Out
            </span>
        </div>
    </div>
</Card>
<style lang="stylus">
@import 'variables'

.account
    border-bottom 1px solid transparify(white, 12%)
    padding 16px
    width 100%
    display flex
    align-items center
    gap 8px
    .data
        display flex
        flex-direction column
        gap 8px
        line-height 1
        .name
            font-size 18px
            font-weight 500
        .email
            font-size 14px
            font-weight 500
            opacity 0.5

.links
    width 100%
    display flex
    flex-direction column
    padding 8px
    gap 0px

.link
    display flex
    align-items center
    gap 8px
    padding 8px
    width 100%
    border-radius 4px
    cursor pointer
    font-size 16px
    font-weight 600
    opacity 0.5
    &:hover
        opacity 1
        background transparify(white, 8%)
    &.red
        color $red
        opacity 1
        &:hover
            color lighten($red, 20%)
</style>