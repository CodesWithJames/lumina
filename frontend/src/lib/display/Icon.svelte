<script lang="ts">
export let icon: any;
export let size: number = undefined;
</script>
<div class="icon">
    <svelte:component this={icon} size={size}/>
</div>
<style lang="stylus">

.icon
    display inline-flex
</style>