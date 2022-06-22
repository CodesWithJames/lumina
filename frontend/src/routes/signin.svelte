<script lang="ts">
import Logo from '$lib/icons/Logo.svelte'
import Tag from '$lib/display/Tag.svelte'
import Global from '$templates/Global.svelte'
import Settlement from '$lib/icons/Settlement.svelte';
import Card from '$lib/cards/Card.svelte'
import Input from '$lib/inputs/Input.svelte'
import Email from '$icons/Email.svelte';
import Box from '$lib/cards/Box.svelte';
import Button from '$lib/buttons/Button.svelte'
import ChevronRight from '$icons/ChevronRight.svelte';
import EyeOffOutline from '$icons/EyeOffOutline.svelte';
import EyeOutline from 'svelte-material-icons/EyeOutline.svelte';
import Plus from '$icons/Plus.svelte';
import Lock from '$icons/Lock.svelte';
import Segment from '$lib/buttons/Segment.svelte';
import Account from '$icons/Account.svelte';
import Swap from '$icons/SwapHorizontal.svelte';
import ExitToApp from '$icons/ExitToApp.svelte';
import { MessageType } from '$types/message';
import { page, session } from '$app/stores';
import { set_cookie } from '$utils/cookie';
import OverlayLoading from '$lib/loading/OverlayLoading.svelte';
import future from '$utils/future';
import { goto } from '$app/navigation';

enum DisplayPage {
    Email,
    Password
};

let display = DisplayPage.Email;
let password_visible = false;
let loading = false;

let user = {
    email: '',
    password: '',
}

async function signin () {
    let { data, errors } = await $page.stuff.graph.req`message {
        login(${user})
    }`

    if (errors.length > 0) {
        errors.map(error => $page.stuff.alerts.create_alert(MessageType.Error, error))
        return
    }

    let { login: token } = data
    if(!token) return $page.stuff.alerts.create_alert(MessageType.Error, 'Invalid Login')

    set_cookie('token', token)
    $session.auth_token = token

    $page.stuff.alerts.create_alert(MessageType.Success, 'Login Successful')

    await goto('/dashboard')
}


</script>

<Global>
    <div class="page">
        <div class="top-part">
            <Logo />
            <Tag color="brand">Sign In</Tag>
            <h1>
                One account. All of Lumina.
                <span class="icon h1-icon">
                    <Settlement/>
                </span>
            </h1>
        </div>
        {#if loading}
            <OverlayLoading/>
        {/if}
        <Card padding="24px" max_width="550px">
            <Box max_width="360px" gap="20px">
                {#if display === DisplayPage.Email}
                        <Input
                            type="email"
                            placeholder="Enter your email"
                            left_icon={Email}
                            name="email"
                            focus_on_mount={true}
                            bind:value={user.email}
                            on:keypress={e => { if (e.key === "Enter" && user.email) display = DisplayPage.Password}}/>
                        <Button
                            disabled={!user.email}
                            on:click={() => display = DisplayPage.Password}
                            right_icon={ChevronRight}>
                            Enter password
                        </Button>
                        <Button
                            style="translucent"
                            right_icon={Plus}>
                            Create Account
                        </Button>
                {/if}
                {#if display === DisplayPage.Password}
                    <Segment
                        on:click={() => display = DisplayPage.Email}
                        left_icon={Account}
                        right_icon={Swap}>
                        {user.email}
                    </Segment>
                    <Input
                        type={password_visible ? 'text' : 'password'}
                        placeholder="Enter your password"
                        left_icon={Lock}
                        right_icon={password_visible ? EyeOutline : EyeOffOutline}
                        focus_on_mount={true}
                        right_icon_handler={() => password_visible = !password_visible}
                        on:keypress={e => { if (e.key === "Enter" && user.password) future(signin(), is_loading => loading = is_loading)}}
                        name="password"
                        bind:value={user.password}/>
                    <Button
                        disabled={!user.password}
                        on:click={() => future(signin(), is_loading => loading = is_loading)}
                        right_icon={ExitToApp}>
                        Sign In
                    </Button>
                {/if}
            </Box>
        </Card>
    </div>
</Global>

<style lang="stylus">
@import 'variables'

.top-part
    display flex
    flex-direction column
    align-items center
    max-width 500px
    width: 100%
    gap 20px

h1
    margin 0
    display flex
    gap 10px
    align-items center
    font-weight 600

.icon
    display inline-flex

.h1-icon
    color $brand

.page
    padding 80px 20px
    display flex
    flex-direction column
    gap 40px
    align-items center

</style>
