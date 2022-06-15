<script lang="ts">
    import Icon from '$lib/display/Icon.svelte';

    import ChevronRight from 'svelte-material-icons/ChevronRight.svelte';
    import Card from './Card.svelte';
    export let icon;
    export let title: string
</script>

<Card gap="12px" padding="24px" direction="horizontal">
    <Icon {icon} size={32} />
    <div class="box">
        <div class="top-part">
            <div class="title">{ title }</div>
            <slot name="tag"/>
        </div>
        <div class="description">
            <slot/>
        </div>
    </div>
    <ChevronRight size="24" />
</Card>

<style lang="stylus">
@import 'variables'

.box
    display flex
    flex-direction column
    align-items flex-start
    justify-content center
    flex 1

.description
    opacity 0.5

.title
    font-size 18px
    font-weight 700

.top-part
    display flex
    align-items center
    gap 8px

</style>
