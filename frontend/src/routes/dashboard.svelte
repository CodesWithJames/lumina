<script lang="ts">
import App from '$templates/App.svelte';
import Settlement from '$lib/icons/Settlement.svelte';
import Coloured from '$lib/display/Coloured.svelte';
import Logo from '$lib/icons/Logo.svelte';
import Button from '$lib/buttons/Button.svelte';
import Copy from '$icons/ContentCopy.svelte';
import Info from '$icons/InformationOutline.svelte';
import Icon from '$lib/display/Icon.svelte';
import Box from '$lib/cards/Box.svelte';
import { page, session } from '$app/stores'
import { MessageType } from '$types/message';
import AppCard from '$lib/cards/AppCard.svelte';
import PassportBiometric from 'svelte-material-icons/PassportBiometric.svelte';
import Tag from '$lib/display/Tag.svelte';

async function copy_referral_link() {
    let text = "this is supposed to be an invite link"
    await navigator.clipboard.writeText(text);
    $page.stuff.alerts.create_alert(MessageType.Success, "Invite link copied")
}


</script>
<App>
    <div class="area">
        <Logo/>
        <h1>
            Hi, { $session.user.first_name } <Coloured><Settlement/></Coloured>
        </h1>
        <div class="referral">
            <h3>User Referrals</h3>
            <p>Earn 50Ł per citizen referred (Lumina's currency, Luna)</p>
            <h2>10 Users</h2>
            <Box padding={false} max_width="300px">
                <Button on:click={copy_referral_link}
                    left_icon={Copy}
                    style="translucent">Copy referral link</Button>
            </Box>
        </div>
        <Box gap="16px">
            <AppCard icon={PassportBiometric} title="Become a citizen">
                <span slot="tag">
                    <Tag>
                        RECOMMENDED
                    </Tag>
                </span>
                Sign up to become a citizen of Lumina here.
            </AppCard>
            <AppCard icon={PassportBiometric} title="Join the Discord">
                <span slot="tag">
                    <Tag>
                        RECOMMENDED
                    </Tag>
                </span>
                If you've got questions or are interested in contributing, come chat on our community Discord server.
            </AppCard>
            <AppCard icon={PassportBiometric} title="Register a business">
                <span slot="tag">
                    <Tag color="white">
                        COMING SOON
                    </Tag>
                </span>
                Create an organisation in Lumina online in a few minutes..
            </AppCard>
            <AppCard icon={PassportBiometric} title="Job board">
                <span slot="tag">
                    <Tag color="white">
                        COMING SOON
                    </Tag>
                </span>
                Find a job. Get involved with helping at Lumina.
            </AppCard>
            <AppCard icon={PassportBiometric} title="Create a bank account">
                <span slot="tag">
                    <Tag color="white">
                        COMING SOON
                    </Tag>
                </span>
                Set up a new bank account to start getting paid Luna.
            </AppCard>
        </Box>
    </div>
    <div class="area light">
        <h2>
            <Coloured><Icon icon={Info}/></Coloured> Information Hub
        </h2>
        <Box gap="16px">
            <AppCard icon={PassportBiometric} title="Read the Lumina Accords">
                <span slot="tag">
                    <Tag>
                        COMING SOON
                    </Tag>
                </span>
                Hello
            </AppCard>
        </Box>
    </div>
</App>
<style lang="stylus">
@import 'variables'

h1
    font-size 36px
    font-weight 700
    display flex
    align-items center

h2
    font-weight 700
    margin 0
    padding-bottom 10px
    display flex
    gap 8px
    align-items center

.referral
    h3
        margin 0
        font-weight 700
        text-tranform uppercase

    p
        opacity 0.5

.area
    padding 40px
    gap 32px
    display flex
    flex-direction column
    flex 1
    &.light
        background transparify(white, 4%)
</style>