<script>
import Card from '$lib/cards/Card.svelte';
import InformationOutline from '$icons/InformationOutline.svelte';

</script>
<Card
    padding="24px"
    reset_bg={true}>
    <div class="description">
        <InformationOutline/>
        No new notifications
    </div>
</Card>
<style lang="stylus">
@import 'variables'

.description
    opacity 0.4
</style>