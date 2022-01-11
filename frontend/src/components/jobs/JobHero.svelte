<script lang="ts">
import JobSection from "@components/jobs/JobSection.svelte"
import type { JobField } from "@components/jobs/jobs_types"

import CodeTags from "@icons/CodeTags.svelte"
import Clock from "@icons/Clock.svelte"
import Wifi from "@icons/Wifi.svelte"
import Brush from "@icons/Brush.svelte"
import Text from "@icons/Text.svelte"
import Hammer from "@icons/Hammer.svelte"
import Settlement from "@components/custom-icons/settlement.svelte"

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
</script>

<div class="job-info">
    <div class="header">
        <h2 class="job-heading">
            Open positions
        </h2>
        <p class="job-description">
            We are a technology driven society comprised of principled and disciplined people bonded by deeply held mission: to create a modern solution to governance.
        </p>
    </div>
    <div class="jobs-wrapper">
        {#each jobFields as jobField}
            <JobSection bind:jobField />
        {/each}
    </div>
</div>

<style lang="stylus">
.job-info
    display flex
    flex-direction column
    align-items center
    max-width 1000px
    width 100%
    margin 0 auto

.header
    display flex
    flex-direction column
    align-items center
    max-width 600px
    width 100%

.job-heading
    font-size 40px
    font-weight 600
    margin 0
    padding 0

.job-description
    opacity 0.6
    text-align center
    line-height 28px

.jobs-wrapper
    display flex
    flex-direction column
    gap 40px
    padding-top 40px

</style>