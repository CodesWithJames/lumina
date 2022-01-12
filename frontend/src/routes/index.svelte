<script lang="ts">
import type { bind, SvelteComponentDev } from "svelte/internal"
import UpcomingEvents from "@components/events/UpcomingEventsHero.svelte"
import NewsHero from "@components/news/NewsHero.svelte"
import DiscordBanner from "@components/banners/DiscordBanner.svelte"
import KnowledgeHubCard from "@components/hubs/HubsBlock.svelte"
import StatsHero from "@components/stats/StatsBlock.svelte"
import HubHero from "@components/hub/HubHero.svelte"
import JobHero from "@components/jobs/JobHero.svelte"
import TopHero from "@components/banners/heros/TopHero.svelte"

import AccountGroup from "@icons/AccountGroup.svelte"
import Domain from "@icons/Domain.svelte"
import AccountOutline from "@icons/AccountOutline.svelte"
import Flash from "@icons/Flash.svelte"
import ScaleBalance from "@icons/ScaleBalance.svelte"
import Pillar from "@icons/Pillar.svelte"
import PassportBiometric from "@icons/PassportBiometric.svelte"
import Telescope from "@icons/Telescope.svelte"
import BadgeAccountHorizontal from "@icons/AccountBadgeHorizontal.svelte"
import CodeTags from "@icons/CodeTags.svelte"
import Clock from "@icons/Clock.svelte"
import Wifi from "@icons/Wifi.svelte"
import Brush from "@icons/Brush.svelte"
import Text from "@icons/Text.svelte"
import Hammer from "@icons/Hammer.svelte"
import Settlement from "@components/custom-icons/settlement.svelte"
import ChevronRight from "svelte-material-icons/ChevronRight.svelte";

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

let twoWideHubCards = [
    {
        link: "/rights-and-duties",
        tag: {
            text: "Info",
            color: "#2570B6",
            
        },
        title: {
            icon: ScaleBalance,
            text: "Rights & Duties",
        },
        description: "Learn about your rights and duties required for our society to function.",
        greyedOut: false
    },
    {
        link: "/principles-and-values",
        tag: {
            text: "Info",
            color: "#2570B6",
            
        },
        title: {
            icon: Pillar,
            text: "Principles & Values",
        },
        description: "Our mission, principles and values define the emotional, moral, and spiritual core of our society.",
        greyedOut: false
    },
    {
        link: "/",
        tag: {
            text: "Service",
            color: "#492C9C",
            
        },
        title: {
            icon: PassportBiometric,
            text: "Citizenship",
        },
        description: "Find out the benefits of becoming a citizen in Lumina. Get involved, along with thousands others today.",
        greyedOut: false
    },
    {
        link: "/",
        tag: {
            text: "Service",
            color: "#492C9C",
            
        },
        title: {
            icon: AccountOutline,
            text: "myLumina",
        },
        description: "Lumina's cloud government portal that lets citizens access government services.",
        greyedOut: false
    },
    {
        link: "/",
        tag: {
            text: "Info",
            color: "#2570B6",
            
        },
        title: {
            icon: Telescope,
            text: "Our Vision",
        },
        description: "Learn about your rights and duties required for our society to function.",
        greyedOut: false
    },
    {
        link: "/",
        tag: {
            text: "Service",
            color: "#2570B6",
            
        },
        title: {
            icon: BadgeAccountHorizontal,
            text: "Media & Press",
        },
        description: "Are you a member of the press looking to speak to a Luminar representative? Contact us here.",
        greyedOut: false
    }
]

let threeWideHubCards = [
    {
        link: "/",
        tag: {
            text: "Coming Soon",
            color: "#B4B4B4",
            
        },
        title: {
            icon: ScaleBalance,
            text: "Legislation",
        },
        description: "Learn about your rights and duties required for our society to function.",
        greyedOut: true
    },
    {
        link: "/",
        tag: {
            text: "Coming Soon",
            color: "#B4B4B4",
            
        },
        title: {
            icon: ScaleBalance,
            text: "Legislation",
        },
        description: "Learn about your rights and duties required for our society to function.",
        greyedOut: true
    },
    {
        link: "/",
        tag: {
            text: "Coming Soon",
            color: "#B4B4B4",
            
        },
        title: {
            icon: ScaleBalance,
            text: "Legislation",
        },
        description: "Learn about your rights and duties required for our society to function.",
        greyedOut: true
    },
    {
        link: "/",
        tag: {
            text: "Coming Soon",
            color: "#B4B4B4",
            
        },
        title: {
            icon: ScaleBalance,
            text: "Legislation",
        },
        description: "Learn about your rights and duties required for our society to function.",
        greyedOut: true
    },
    {
        link: "/",
        tag: {
            text: "Coming Soon",
            color: "#B4B4B4",
            
        },
        title: {
            icon: ScaleBalance,
            text: "Legislation",
        },
        description: "Learn about your rights and duties required for our society to function.",
        greyedOut: true
    },
    {
        link: "/",
        tag: {
            text: "Coming Soon",
            color: "#B4B4B4",
            
        },
        title: {
            icon: ScaleBalance,
            text: "Legislation",
        },
        description: "Learn about your rights and duties required for our society to function.",
        greyedOut: true
    },
    {
        link: "/",
        tag: {
            text: "Coming Soon",
            color: "#B4B4B4",
            
        },
        title: {
            icon: ScaleBalance,
            text: "Legislation",
        },
        description: "Learn about your rights and duties required for our society to function.",
        greyedOut: true
    },
    {
        link: "/",
        tag: {
            text: "Coming Soon",
            color: "#B4B4B4",
            
        },
        title: {
            icon: ScaleBalance,
            text: "Legislation",
        },
        description: "Learn about your rights and duties required for our society to function.",
        greyedOut: true
    },
    {
        link: "/",
        tag: {
            text: "Coming Soon",
            color: "#B4B4B4",
            
        },
        title: {
            icon: ScaleBalance,
            text: "Legislation",
        },
        description: "Learn about your rights and duties required for our society to function.",
        greyedOut: true
    },
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
                        color: "#492C9C",
                        opacity: "1",
                    },
                    {
                        icon: Wifi,
                        text: "Remote",
                        color: "#492C9C",
                        opacity: "1",
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
                        color: "#492C9C",
                        opacity: "1",
                    },
                    {
                        icon: Wifi,
                        text: "Remote",
                        color: "#492C9C",
                        opacity: "1",
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
                        color: "#492C9C",
                        opacity: "1",
                    },
                    {
                        icon: Wifi,
                        text: "Remote",
                        color: "#492C9C",
                        opacity: "1",
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
                        color: "#492C9C",
                        opacity: "1",
                    },
                    {
                        icon: Wifi,
                        text: "Remote",
                        color: "#492C9C",
                        opacity: "1",
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
                        color: "#492C9C",
                        opacity: "1",
                    },
                    {
                        icon: Wifi,
                        text: "Remote",
                        color: "#492C9C",
                        opacity: "1",
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
                        color: "#492C9C",
                        opacity: "1",
                    },
                    {
                        icon: Wifi,
                        text: "Remote",
                        color: "#492C9C",
                        opacity: "1",
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
                        color: "#000000",
                        opacity: "0.3",
                    },
                    {
                        icon: Settlement,
                        text: "Settlement",
                        color: "#492C9C",
                        opacity: "1",
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
                        color: "#000000",
                        opacity: "0.3",
                    },
                    {
                        icon: Settlement,
                        text: "Settlement",
                        color: "#492C9C",
                        opacity: "1",
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
                        color: "#000000",
                        opacity: "0.3",
                    },
                    {
                        icon: Settlement,
                        text: "Settlement",
                        color: "#492C9C",
                        opacity: "1",
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
                        color: "#000000",
                        opacity: "0.3",
                    },
                    {
                        icon: Settlement,
                        text: "Settlement",
                        color: "#492C9C",
                        opacity: "1",
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
                        color: "#000000",
                        opacity: "0.3",
                    },
                    {
                        icon: Settlement,
                        text: "Settlement",
                        color: "#492C9C",
                        opacity: "1",
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
                        color: "#000000",
                        opacity: "0.3",
                    },
                    {
                        icon: Settlement,
                        text: "Settlement",
                        color: "#492C9C",
                        opacity: "1",
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
            img: "static/images/albert_dp.png"
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
            img: "static/images/albert_dp.png"
        }
    }
]

</script>

<div class="hero-wrapper">
    <div class="inner-hero main-hero-layout">
        <TopHero bind:heroInfo />
    </div>
    <div class="inner-hero">
        <StatsHero bind:stats />
    </div>
    <DiscordBanner/>
</div>
<HubHero bind:twoWideHubCards bind:threeWideHubCards />
<div class="inner-hero jobs-hero">
    <JobHero bind:jobFields />
</div>
<div class="extra-info-wrapper">
    <UpcomingEvents bind:events />
    <div class="knowledge-hub-wrapper">
        <KnowledgeHubCard bind:knowledgeInfo bind:areas />
    </div>
</div>
<NewsHero bind:featuredStories />

<style lang="stylus">
.hero-wrapper
    // min height instead of height because some devices have a smaller height
    // header is 80px, so we need to subtract that from the height
    min-height calc(100vh \- 80px)
    display flex
    flex-direction column
    .main-hero-layout
        flex 1 // take up all available space

.inner-hero
    max-width 1200px
    width 100%
    padding-left 20px
    padding-right 20px
    margin 0 auto

.main-hero-layout
    display grid
    grid-template-columns 1fr 1fr
    grid-gap 40px
    align-items center
    padding-top 60px
    padding-bottom 40px

.jobs-hero
    padding-top 80px
    padding-bottom 80px

.extra-info-wrapper
    display grid
    grid-template-columns 1.2fr 1fr
    width: 100vw

.knowledge-hub-wrapper
    padding 40px
    background-color #EBEBEA
    padding-right 160px

</style>