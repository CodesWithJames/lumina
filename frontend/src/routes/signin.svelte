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
import Plus from '$icons/Plus.svelte';
import Lock from '$icons/Lock.svelte';
import Segment from '$lib/buttons/Segment.svelte';
import Account from '$icons/Account.svelte';
import Swap from '$icons/SwapHorizontal.svelte';
import ExitToApp from '$icons/ExitToApp.svelte';

enum Page {
    Email,
    Password
};

let page = Page.Email;

let user = {
    email: '',
    password: '',
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
        <Card padding="24px" max_width="550px">
            <Box max_width="360px" gap="20px">
                {#if page === Page.Email}
                        <Input
                            type="email"
                            placeholder="Enter your email"
                            left_icon={Email}
                            name="email"
                            bind:value={user.email}/>
                        <Button
                            disabled={!user.email}
                            on:click={() => page = Page.Password}
                            right_icon={ChevronRight}>
                            Enter password
                        </Button>
                        <Button
                            style="translucent"
                            right_icon={Plus}>
                            Create Account
                        </Button>
                {/if}
                {#if page === Page.Password}
                    <Segment
                        on:click={() => page = Page.Email}
                        left_icon={Account}
                        right_icon={Swap}>
                        {user.email}
                    </Segment>
                    <Input
                        type="password"
                        placeholder="Enter your password"
                        left_icon={Lock}
                        name="password"
                        bind:value={user.password}/>
                    <Button
                        disabled={!user.password}
                        right_icon={ExitToApp}>
                        Sign In
                    </Button>
                {/if}
            </Box>
        </Card>

        <!-- {#if page == 'email'}
            <EmailPage on:next={() => (page = 'create-account')} />
        {:else if page == 'create-account'}
            <PasswordPage/>
        {/if} -->
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
