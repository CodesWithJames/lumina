<script>
import InfoBox from "@components/small_components/InfoBox.svelte";

import Text from "@icons/Text.svelte"

</script>
<div class="main">
    <div class="header">
        <div class="header-icon">
            <svelte:component this={Text}/>
        </div>
        <h1>

        </h1>
    </div>
    <p>
        In Lumina, we strive to provide citizens with many rights, 
        as well duties they need to fulfill in order to maintain order in society.
    </p>
    <p>
        We strive to ensure that all rights for citizens are 
        actually provided to citizens, as some rights require active fulfillment (eg: right to education).
    </p>
    <InfoBox />
    <div class="examples-wrapper">
        <div class="left">
            <div class="example-heading">
                Positive right examples
            </div>
            <ul>
                <li>right to housing</li>
                <li>right to food</li>
                <li>right to free healthcare</li>
            </ul>
            <p>
                These types of rights require someone to actively provide them.
            </p>
        </div>
        <div class="right">
            <div class="example-heading">
                Negative right examples
            </div>
            <ul>
                <li>freedom of speech</li>
                <li>freedom from violent crime</li>
                <li>freedom of religion</li>
            </ul>
            <p>
                These types of rights requires others to not infringe citizens rights.
            </p>
        </div>
    </div>
</div>

<style lang="stylus">
h1
    margin 0
    padding 0

</style>