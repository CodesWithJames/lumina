<script lang="ts">
import type { bind, SvelteComponentDev } from "svelte/internal"
import JobSection from "@components/jobs/JobSection.svelte"
import type { JobField } from "@components/jobs/jobs_types"
import HubCard from "@components/hub/HubCard.svelte"

import AccountGroup from "@icons/AccountGroup.svelte"
import Domain from "@icons/Domain.svelte"
import AccountOutline from "@icons/AccountOutline.svelte"
import Flash from "@icons/Flash.svelte"
import DiscordBanner from "@components/banners/DiscordBanner.svelte"
import MainButton from "@components/inputs/MainButton.svelte"
import ChevronRight from "@icons/ChevronRight.svelte"
import CodeTags from "@icons/CodeTags.svelte"
import Clock from "@icons/Clock.svelte"
import Wifi from "@icons/Wifi.svelte"
import Brush from "@icons/Brush.svelte"
import Text from "@icons/Text.svelte"
import Hammer from "@icons/Hammer.svelte"
import Settlement from "@components/custom-icons/settlement.svelte"
import Hub from "@components/custom-icons/Hub.svelte"
import ScaleBalance from "@icons/ScaleBalance.svelte"
import PassportBiometric from "@icons/PassportBiometric.svelte"
import Pillar from "@icons/Pillar.svelte"
import Telescope from "@icons/Telescope.svelte"
import BadgeAccountHorizontal from "@icons/AccountBadgeHorizontal.svelte"

type Stat = {
    value: string,
    name: string,
    icon: typeof SvelteComponentDev,
    color: string
}

let stats: Stat[] = [
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


let jobFields: JobField[] = [
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

let twoWideHubCards = [
    {
        tag: {
            text: "Info",
            color: "#2570B6",
            
        },
        title: {
            icon: ScaleBalance,
            text: "Rights & Duties",
        },
        description: "Learn about your rights and duties required for our society to function."
    },
    {
        tag: {
            text: "Info",
            color: "#2570B6",
            
        },
        title: {
            icon: Pillar,
            text: "Principles & Values",
        },
        description: "Our mission, principles and values define the emotional, moral, and spiritual core of our society."
    },
    {
        tag: {
            text: "Service",
            color: "#492C9C",
            
        },
        title: {
            icon: PassportBiometric,
            text: "Citizenship",
        },
        description: "Find out the benefits of becoming a citizen in Lumina. Get involved, along with thousands others today."
    },
    {
        tag: {
            text: "Service",
            color: "#492C9C",
            
        },
        title: {
            icon: AccountOutline,
            text: "myLumina",
        },
        description: "Lumina's cloud government portal that lets citizens access government services."
    },
    {
        tag: {
            text: "Info",
            color: "#2570B6",
            
        },
        title: {
            icon: Telescope,
            text: "Our Vision",
        },
        description: "Learn about your rights and duties required for our society to function."
    },
    {
        tag: {
            text: "Service",
            color: "#2570B6",
            
        },
        title: {
            icon: BadgeAccountHorizontal,
            text: "Media & Press",
        },
        description: "Are you a member of the press looking to speak to a Luminar representative? Contact us here."
    }
]


</script>
<div class="hero-wrapper">
    <div class="inner-hero main-hero-layout">
        <div>
            <h3 class="subtitle">Start a new city</h3>
            <h1>
                <div>
                    city of the
                </div>
                <div>
                    <span class="gradient-text">future</span>
                </div>
            </h1>
            <p>
                Welcome to Lumina, a social experiment with the goals of setting up 
                a new innovative and environmentally sustainable city.
            </p>
            <p>
                Lumina proposes to set up a Autonomous Zone, 
                which is a city with it's own government, laws, and currency.
            </p>
            <div>
                <MainButton
                    href="/onboarding"
                    right_icon={ChevronRight}
                    inline={true}
                    gradient={true}
                >
                    Become a citizen
                </MainButton>
            </div>

        </div>
        <div class="hero-image-wrapper">
            <img class="hero-image" src="/images/hero-image.svg" alt="a city skyline">
        </div>
    </div>
    <div class="inner-hero">
        <div class="stats">
            {#each stats as stat}
                <div class="stat">
                    <div class="stat-icon" style="color: {stat.color};">
                        <svelte:component this={stat.icon} />
                    </div>
                    <div class="stat-value">
                        <span>{stat.value}</span>
                    </div>
                    <div class="stat-name">
                        <span>{stat.name}</span>
                    </div>
                </div>
            {/each}
        </div>
    </div>
    <DiscordBanner/>
</div>

<div class="inner-hero">
    <div class="title hub">
        <div class="hub-icon">
            <svelte:component this={Hub} />
        </div>
        <h2 class="hub-heading">
            Lumina Hub
        </h2>
    </div>
    <div class="hub-cards-wrapper">
        <div class="hub-cards-two-wide-wrapper">
            {#each twoWideHubCards as info}
                <HubCard bind:info/>
            {/each}
        </div>
        <div class="hub-cards-three-wide-wrapper">

        </div>
    </div>
</div>

<div class="inner-hero jobs-hero">
    <div class="job-info">
        <h2 class="job-heading">
            Open positions
        </h2>
        <p class="job-description">
            We are a technology driven society comprised of principled and disciplined people bonded by deeply held mission: to create a modern solution to governance.
        </p>
    </div>
    <div class="jobs-wrapper">
        {#each jobFields as jobField}
            <JobSection
                bind:jobField
            />
        {/each}
    </div>
</div>

<style lang="stylus">


// STATS SECTION
.stats
    display grid
    grid-auto-flow column
    grid-auto-columns 1fr
    grid-gap 20px
    padding 40px 0
.stat
    display flex
    flex-direction column
    align-items center
.stat-icon
    font-size 48px
.stat-value
    font-size 36px
    font-weight 600
.stat-name
    font-size 18px
    opacity 0.5

// HERO MAIN

.hero-wrapper
    // min height instead of height because some devices have a smaller height
    // header is 80px, so we need to subtract that from the height
    min-height calc(100vh \- 80px)
    display flex
    flex-direction column
    .main-hero-layout
        flex 1 // take up all available space

.gradient-text
    background linear-gradient(90deg, #5c32d4, #32c0c0);
    -webkit-background-clip text
    -webkit-text-fill-color transparent

.subtitle
    font-size 20px
    text-transform uppercase
    font-weight 600
    opacity 0.3
    margin 0

h1
    font-size 80px
    font-weight 600
    margin 0

.inner-hero
    max-width 1200px
    width 100%
    padding-left 20px
    padding-right 20px
    margin 0 auto

.main-hero-layout
    display grid
    grid-template-columns 1fr 1fr
    grid-gap 20px
    align-items center
    padding-top 60px
    padding-bottom 40px

.hero-image-wrapper
    position relative
    display flex
    align-items center

.hero-image
    position absolute
    left 0

.job-info
    display flex
    flex-direction column
    align-items center
    max-width 600px
    margin 0 auto

.job-heading
    font-size 40px
    font-weight 600
    margin 0
    padding 0

.job-description
    opacity 0.6
    text-align center
    line-height 28px

.jobs-hero
    padding-top 80px
    padding-bottom 80px

.jobs-wrapper
    display flex
    flex-direction column
    gap 40px
    padding-top 40px

.title.hub
    display flex
    margin-bottom 20px

.hub-icon
    font-size 46px
    margin-right 10px

.hub-heading
    margin 0
    padding 0
    font-size 40px
    margin auto 0
</style>