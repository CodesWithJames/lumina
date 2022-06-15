<script lang="ts">
import UpcomingEvents from "$components/events/UpcomingEventsHero.svelte"
import NewsHero from "$components/pages/news/NewsHero.svelte"
import DiscordBanner from "$components/banners/DiscordBanner.svelte"
import KnowledgeHubCard from "$components/hubs/HubsBlock.svelte"
import StatsHero from "$components/stats/StatsBlock.svelte"
import JobHero from "$components/jobs/JobHero.svelte"

import AccountGroup from "$icons/AccountGroup.svelte"
import Domain from "$icons/Domain.svelte"
import AccountOutline from "$icons/AccountOutline.svelte"
import Flash from "$icons/Flash.svelte"
import CodeTags from "$icons/CodeTags.svelte"
import Clock from "$icons/Clock.svelte"
import Wifi from "$icons/Wifi.svelte"
import Brush from "$icons/Brush.svelte"
import Text from "$icons/Text.svelte"
import Hammer from "$icons/Hammer.svelte"
import ChevronRight from "svelte-material-icons/ChevronRight.svelte";
import Settlement from "$lib/icons/Settlement.svelte"
import Site from '$templates/Site.svelte'
import Hero20 from '$components/heros/Hero20.svelte';
import Hero48 from '$components/heros/Hero48.svelte'
import Button from '$lib/buttons/Button.svelte'
import Inner from '$components/heros/Inner.svelte'
import Split from '$layouts/Split.svelte'
import HubArea from '$components/hub/HubArea.svelte';
import Coloured from '$lib/display/Coloured.svelte'

let heroInfo = {
    titleExcludingLastWord: "city of the",
    highlightedWord: "future",
    subtitle: "Start a new city",
    descriptionParagraphs: [
        "Welcome to Lumina, a social experiment with the goals of setting up a new innovative and environmentally sustainable city.",
        "Lumina proposes to set up a Autonomous Zone, which is a city with it's own government, laws, and currency.",
    ],
    buttons: [
        {
            text: "Become a citizen",
            link: "/onboarding",
            gradient: true,
            rightIcon: ChevronRight
        },
    ],
    img: "/images/hero-image.svg"
}

let stats = [
    {
        icon: AccountGroup,
        name: "Citizens registered",
        value: "2251",
        color: "#5D35D5"
    },
    {
        icon: Domain,
        name: "Government Ministries",
        value: "15",
        color: "#4F63CE"
    },
    {
        icon: AccountOutline,
        name: "Government Staff",
        value: "36",
        color: "#4488C9"
    },
    {
        icon: Flash,
        name: "Renewable",
        value: "100%",
        color: "#34BFC1"
    }
]

let jobFields = [
    {
        title: "Software",
        jobs: [
            {
                title: "Backend Engineer",
                icon: CodeTags,
                attributes: [
                    {
                        icon: Clock,
                        text: "Volunteer",
                    },
                    {
                        icon: Wifi,
                        text: "Remote",
                    }
                ]
            },
            {
                title: "Frontend Engineer",
                icon: CodeTags,
                attributes: [
                    {
                        icon: Clock,
                        text: "Volunteer",
                    },
                    {
                        icon: Wifi,
                        text: "Remote",
                    }
                ]
            },
            {
                title: "Deployment Engineer",
                icon: CodeTags,
                attributes: [
                    {
                        icon: Clock,
                        text: "Volunteer",
                    },
                    {
                        icon: Wifi,
                        text: "Remote",
                    }
                ]
            },

        ]
    },
    {
        title: "Design",
        jobs: [
            {
                title: "UX Design",
                icon: Brush,
                attributes: [
                    {
                        icon: Clock,
                        text: "Volunteer",
                    },
                    {
                        icon: Wifi,
                        text: "Remote",
                    }
                ]
            },
            {
                title: "Video Editor",
                icon: Brush,
                attributes: [
                    {
                        icon: Clock,
                        text: "Volunteer",
                    },
                    {
                        icon: Wifi,
                        text: "Remote",
                    }
                ]
            },
            {
                title: "Content Writer",
                icon: Text,
                attributes: [
                    {
                        icon: Clock,
                        text: "Volunteer",
                    },
                    {
                        icon: Wifi,
                        text: "Remote",
                    }
                ]
            }

        ]
    },
    {
        title: "Settlement",
        jobs: [
            {
                title: "Welder",
                icon: Hammer,
                attributes: [
                    {
                        icon: Clock,
                        text: "Future",
                    },
                    {
                        icon: Settlement,
                        text: "Settlement",
                    }
                ]
            },
            {
                title: "Plumber",
                icon: Hammer,
                attributes: [
                    {
                        icon: Clock,
                        text: "Future",
                    },
                    {
                        icon: Settlement,
                        text: "Settlement",
                    }
                ]
            },
            {
                title: "Concreter",
                icon: Hammer,
                attributes: [
                    {
                        icon: Clock,
                        text: "Future",
                    },
                    {
                        icon: Settlement,
                        text: "Settlement",
                    }
                ]
            },
            {
                title: "Power Engineer",
                icon: Hammer,
                attributes: [
                    {
                        icon: Clock,
                        text: "Future",
                    },
                    {
                        icon: Settlement,
                        text: "Settlement",
                    }
                ]
            },
            {
                title: "Civil Engineer",
                icon: Hammer,
                attributes: [
                    {
                        icon: Clock,
                        text: "Future",
                    },
                    {
                        icon: Settlement,
                        text: "Settlement",
                    }
                ]
            },
            {
                title: "Electrical Engineer",
                icon: Hammer,
                attributes: [
                    {
                        icon: Clock,
                        text: "Future",
                    },
                    {
                        icon: Settlement,
                        text: "Settlement",
                    }
                ]
            }
        ]
    }
]

let events = [
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
        img: "/images/lumina_flag.png",
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
        img: "images/lumina_flag.png",
        location: "Lumina"
    },
]

let knowledgeInfo = {
    emoji: "📕",
    title: "Knowledge Hub",
    description: "Learn about how Lumina plans to become a reality. Get information on funding, planning, settlement & more."
}

let areas = [
    {
        emoji: "🏦",
        title: "Banking & Monetary System"
    },
    {
        emoji: "💰",
        title: "Taxation System"
    }
]

let featuredStories = [
    {
        tag: {
            text: "Article",
            color: "#492C9C"
        },
        date: {
            dayAsNumber: 4,
            month: "Jan",
            year: 2020
        },
        title: "Lumina hits 2000 citizens",
        author: {
            name: "Albert",
            img: "/images/albert_dp.png"
        }
    },
    {
        tag: {
            text: "Article",
            color: "#492C9C"
        },
        date: {
            dayAsNumber: 3,
            month: "Jan",
            year: 2020
        },
        title: "Lumina hits 1000 citizens",
        author: {
            name: "Albert",
            img: "/images/albert_dp.png"
        }
    }
]

</script>
<Site>
    <Hero48>
        <Inner>
            <Split>
                <div>
                    <h1>city of the <Coloured>future</Coloured></h1>
                    <p>
                        Welcome to <Coloured>Lumina</Coloured>, a social experiment with the goals of setting up a new innovative and environmentally sustainable city.
                    </p>
                    <p>
                        Lumina proposes to set up a <Coloured>Autonomous Zone</Coloured>, which is a city with it's own government, laws, and currency.
                    </p>
                    <Button href="onboarding" right_icon={Settlement} hug={true}>
                        Become a citizen
                    </Button>
                </div>
                <div class="image-wrapper">
                    <img class="hero-image" src={heroInfo.img} alt="a city skyline">
                </div>
            </Split>
            <StatsHero bind:stats/>
        </Inner>
    </Hero48>
    <Hero20>
        <DiscordBanner/>
    </Hero20>
    <Hero48 translucent={true}>
        <Inner max_width={1000}>
            <HubArea/>
        </Inner>
    </Hero48>
    <Hero48>
        <Inner>
            <JobHero bind:jobFields />
        </Inner>
    </Hero48>
    <div class="extra-info-wrapper">
        <UpcomingEvents bind:events />
        <div class="knowledge-hub-wrapper">
            <KnowledgeHubCard bind:knowledgeInfo bind:areas />
        </div>
    </div>

    <NewsHero bind:featuredStories />
</Site>
<svelte:head>
    <title>Lumina - the city of the future</title>
</svelte:head>

<style lang="stylus">
@import 'variables'

h1
    font-size 80px
    font-weight 600
    margin 0

.image-wrapper
    position relative
    display flex
    align-items center

.hero-image
    position: relative;
    left: 0;

.extra-info-wrapper
    display grid
    grid-template-columns 1.2fr 1fr
    background transparify(white, 4%)
    width: 100vw

.knowledge-hub-wrapper
    padding 40px
    background transparify(white, 4%)
    padding-right 160px

</style>