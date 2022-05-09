<script>
import Footer from '$components/banners/Footer.svelte';
import Header from '$components/banners/Header.svelte';
import Global from '$templates/Global.svelte';

</script>
<Global>
    <Header/>
    <slot/>
    <Footer/>
</Global>