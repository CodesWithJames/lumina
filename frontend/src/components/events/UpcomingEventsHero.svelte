<script lang="ts">
import type { EventType } from "@components/events/event_types"
import EventCard from "@components/events/EventCard.svelte"
import ChevronRight from "svelte-material-icons/ChevronRight.svelte"

let events: EventType[] = [
    {
        link: "/",
        date: {
            time: {
                hour: "00",
                minutes: "00",
                pm: false
            },
            day: 31,
            month: "Aug",
            year: 2022
        },
        title: "Lumina Day",
        type: "Holiday",
        img: "static/images/lumina_flag.png",
        location: "Lumina"
    },
    {
        link: "/",
        date: {
            time: {
                hour: "00",
                minutes: "00",
                pm: false
            },
            day: 31,
            month: "Aug",
            year: 2022
        },
        title: "Lumina Day",
        type: "Holiday",
        img: "static/images/lumina_flag.png",
        location: "Lumina"
    },
]

</script>

<div class="main">
    <div class="inner-wrapper">
        <div class="header">
            <div class="title">
                Upcoming Events
            </div>
            <div class="view-events">
                <a href="/">
                    <div class="link-text">
                        View all events
                    </div>
                    <div class="chevron-right">
                        <svelte:component this={ChevronRight} />
                    </div>
                </a>
            </div>
        </div>
        <div class="events-wrapper">
            {#each events as event}
                <EventCard bind:event />
            {/each}
        </div>
    </div>
</div>

<style lang="stylus">
@import "variables"

.main
    padding 40px
    background-color #F3F3F2
    display flex

.inner-wrapper
    margin-left auto
    max-width 700px

.header
    display flex
    margin-bottom 40px

.events-wrapper
    display grid
    grid-auto-flow column
    grid-auto-columns minmax(0, 300px)
    grid-gap 40px

.title
    font-weight 600
    font-size 32px

.view-events
    margin auto 0
    margin-left auto
    font-weight 600

a
    color $brand
    text-decoration none
    display flex

.chevron-right
    margin auto 0
    font-size 20px
    height 20px

</style>