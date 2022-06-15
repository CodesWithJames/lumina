<script>
import ShieldAccount from '$icons/ShieldAccount.svelte';
import Email from '$icons/Email.svelte';
import Input from '$lib/inputs/Input.svelte';
import ProfilePicker from '$lib/inputs/ProfilePicker.svelte';
import Button from '$lib/buttons/Button.svelte';

let user = {
    email: '',
    password: '',
    first_name: '',
    last_name: '',
};
</script>

<div class="page">
    <h1>
        <span class="icon h1-icon">
            <ShieldAccount />
        </span>
        Create Account
    </h1>
    <div class="card">
        <div class="card-inner">
            <div class="center">
                <ProfilePicker />
            </div>
            <Input
                name="Email"
                placeholder="eg: john@example.com"
                bind:value={user.email}
                left_icon={Email}
            />
            <div class="split">
                <Input
                    name="First Name"
                    placeholder="eg: John"
                    bind:value={user.first_name}
                />
                <Input
                    name="Last Name"
                    placeholder="eg: Smith"
                    bind:value={user.last_name}
                />
            </div>
            <Input
                type="password"
                name="Password"
                placeholder="Use a secure password"
                bind:value={user.password}
                left_icon={Email}
            />
            <Button on:click={() => {}} right_icon={ShieldAccount}>
                Create Account
            </Button>
        </div>
    </div>
</div>

<style lang="stylus">
@import 'variables'

.page
    display: flex
    flex-direction: column
    align-items: center
    width: 100%
    gap: 40px

.center
    display: flex
    justify-content: center

.split
    display: grid
    grid-auto-flow: column
    grid-auto-columns: 1fr
    gap: 20px

.card
    padding: 60px 20px
    background: rgba(255,255,255,0.05)
    display: flex
    width: 100%
    max-width: 600px
    justify-content: center
    border-radius: 8px

.card-inner
    gap: 20px
    max-width: 400px
    flex-direction: column
    width: 100%
    display flex

h1
    margin: 0
    display: flex
    gap 10px
    align-items center
    font-weight 600

.icon
    display inline-flex

.h1-icon
    color $brand

</style>
