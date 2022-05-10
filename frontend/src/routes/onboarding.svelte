<script lang="ts">
import Logo from '$lib/icons/Logo.svelte'
import AccountCreate from '$components/pages/onboarding/AccountCreate.svelte';
import AccountType from '$components/pages/onboarding/AccountType.svelte'
import ProgressBar from '$lib/display/ProgressBar.svelte';
import Tag from '$lib/display/Tag.svelte';
import Global from '$templates/Global.svelte';

let page = "account-type";

let percentages = {
    "account-type": 25,
    "create-account": 75,
}

$: width = percentages[page]

</script>
<Global>
    <div class="page">
        <div class="top-part">
            <Logo/>
            <ProgressBar bind:width/>
            <Tag color="brand">Onboarding</Tag>
        </div>
        {#if page == "account-type"}
            <AccountType on:next={() => page = "create-account"}/>
        {:else if page == "create-account"}
            <AccountCreate/>
        {/if}
    </div>
</Global>
<style lang="stylus">

.top-part
    display: flex
    flex-direction: column
    align-items: center
    max-width: 500px
    width: 100%
    gap: 20px


.page
    padding: 80px 20px
    display: flex
    flex-direction: column
    gap: 40px
    align-items: center

</style>