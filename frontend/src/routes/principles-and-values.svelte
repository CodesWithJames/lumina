<script lang="ts">
import TopHero from "@components/banners/heros/TopHero.svelte"
import SubHeading from "@components/general_components/SubHeading.svelte"
import VerticalLinkedDiagram from "@components/general_components/vertical_linked_diagram/VerticalLinkedDiagram.svelte"
import ChevronDown from "@icons/ChevronDown.svelte"
import ChevronRight from "@icons/ChevronRight.svelte"
import ScriptText from "@icons/ScriptText.svelte"
import RobotIndustrial from "@icons/RobotIndustrial.svelte"
import Bolt from "@icons/Bolt.svelte"
import Sync from "@icons/Sync.svelte"
import Brain from "@icons/Brain.svelte"
import ArmFlexOutline from "@icons/ArmFlexOutline.svelte"
import Leaf from "@icons/Leaf.svelte"
import Shape from "@icons/Shape.svelte"

import { tick } from "svelte"
import { loop_guard } from "svelte/internal"
import type { VerticalDiagramCardType } from "@components/general_components/vertical_linked_diagram/vertical_diagram_types";
import InfoBox from "@components/general_components/InfoBox.svelte"
import type { InfoBoxType } from "@components/general_components/general_components_types"
import Telescope from "svelte-material-icons/Telescope.svelte"
import RightsDutiesDisplayBlock from "@components/rights_and_duties/RightsDutiesDisplayBlock.svelte";

let heroInfo = {
    subtitle: "A Core Difference",
    titleExcludingLastWord: "principles",
    highlightedWord: "and values",
    descriptionParagraphs: [
        "Our mission, principles and values define the emotional, moral, and spiritual core of our society."
    ],
    buttons: [
        {
            text: "Read your rights", 
            link: "/",
            branded: true,
            rightIcon: ChevronDown
        },
        {
            text: "Become a citizen", 
            link: "/",
            rightIcon: ChevronRight
        },
    ],
    img: "/images/astronaut.png"
}

let subHeadingInfoFirstSection = {
    icon: ScriptText,
    heading: "Our Principles",
    paragraphs: [
        "Principles are fundamental, universal truths that transcend time, geography, culture and context.",
        "They define the foundation for our society, and provide a base for our chain of reasoning."
    ]
}

let diagramSections: VerticalDiagramCardType[] = [
    {
        icon: RobotIndustrial,
        heading: "Pragmatism",
        description: "The right to free education in subjects: which do not cause division in society  and: lead to employment; or further scientific knowledge; or create technological advancement; or philosophy;",
        active: false,
        separatorActive: false
    },
    {
        icon: Leaf,
        heading: "Sustainability",
        description: "The right to free education in subjects: which do not cause division in society  and: lead to employment; or further scientific knowledge; or create technological advancement; or philosophy;",
        active: false,
        separatorActive: false
    },
    {
        icon: Sync,
        heading: "Iterate and improve",
        description: "Failure is part of the processs of iteration. Embrace and learn from failure.",
        active: false,
        separatorActive: false
    },
    {
        icon: Brain,
        heading: "Never stop learning",
        description: "The right to free education in subjects: which do not cause division in society  and: lead to employment; or further scientific knowledge; or create technological advancement; or philosophy;",
        active: false,
        separatorActive: false
    },
    {
        icon: ArmFlexOutline,
        heading: "Be bold",
        description: "The right to free education in subjects: which do not cause division in society  and: lead to employment; or further scientific knowledge; or create technological advancement; or philosophy;",
        active: false,
        separatorActive: false
    }
]

let subHeadingInfoSecondSection = {
    icon: Shape,
    heading: "Our Values",
    paragraphs: [
        "Principles are fundamental, universal truths that transcend time, geography, culture and context.",
        "They define the foundation for our society, and provide a base for our chain of reasoning."
    ]
}

let values: InfoBoxType[] = [
    {
        icon: {
            type: Telescope,
            size: 28,
            color: "rgb(45, 55, 68)"
        },
        heading: "Optimism",
        paragraphs: ["Learn more about your rights and duties as a citizen of Lumina"]
    },
    {
        icon: {
            type: Telescope,
            size: 28,
            color: "rgb(45, 55, 68)"
        },
        heading: "Teamwork",
        paragraphs: ["Learn more about your rights and duties as a citizen of Lumina"]
    },
    {
        icon: {
            type: Telescope,
            size: 28,
            color: "rgb(45, 55, 68)"
        },
        heading: "Responsibility",
        paragraphs: ["Learn more about your rights and duties as a citizen of Lumina"]
    },
    {
        icon: {
            type: Telescope,
            size: 28,
            color: "rgb(45, 55, 68)"
        },
        heading: "Self-Sufficiency",
        paragraphs: ["Learn more about your rights and duties as a citizen of Lumina"]
    },
    {
        icon: {
            type: Telescope,
            size: 28,
            color: "rgb(45, 55, 68)"
        },
        heading: "Humour",
        paragraphs: ["Learn more about your rights and duties as a citizen of Lumina"]
    },
    {
        icon: {
            type: Telescope,
            size: 28,
            color: "rgb(45, 55, 68)"
        },
        heading: "Optimism",
        paragraphs: ["Learn more about your rights and duties as a citizen of Lumina"]
    },
]

import Information from "@icons/Information.svelte";
let valueBoxes: InfoBoxType = {
    icon: {
        type: Information,
        size: 28,
        color: "rgb(45, 55, 68)"
    },
    heading: "Optimism",
    paragraphs: ["Learn more about your rights and duties as a citizen of Lumina"]
}


let last_scroll_promise
let next_active_item_index = 0

async function scrolled(){
    if (last_scroll_promise) {
        return
    }
    last_scroll_promise = update_diagram_sections()
    await last_scroll_promise
    last_scroll_promise = undefined
}

async function activate_next_card() {

}

async function update_diagram_sections () {
    let card = diagramSections[next_active_item_index]
    let client_height = window.innerHeight

    if (card) {
        let distanceFromTop = card.el.getBoundingClientRect().top
        if (distanceFromTop < client_height * 0.6) { // if the card is half way up the screen (should be active)
            let prev_card = diagramSections[next_active_item_index - 1]
            if (prev_card) {
                // let the previous item know that this item is now active, and separator should be shown
                prev_card.separatorActive = true
                diagramSections = diagramSections
            }
            await new Promise(resolve => setTimeout(resolve, 500)) 
            card.active = true
            diagramSections = diagramSections
            next_active_item_index++
            // this item may have been passed on scroll, so check again
            return await update_diagram_sections()
        }
    }

    // We need functionality to inactivate items which should not be active at the current scroll level
    card = diagramSections[next_active_item_index - 1]

    if (card) {
        let distanceFromTop = card.el.getBoundingClientRect().top
        if (distanceFromTop > client_height * 0.6) { // if the card is half way up the screen (should be active)
            let prev_card = diagramSections[next_active_item_index - 2]
            if (prev_card) {
                // let the previous item know that this item is now inactive, and separator should be hidden
                prev_card.separatorActive = false
            }

            card.active = false
            diagramSections = diagramSections
            await new Promise(resolve => setTimeout(resolve, 500)) 
            if (next_active_item_index > 0) {
                next_active_item_index--
                // this item may have been passed on scroll, so check again
                await update_diagram_sections()
            }
        }
    }
}

</script>

<svelte:window on:scroll={scrolled} />

<div class="hero-wrapper">
    <div class="inner-hero main-hero-layout">
        <TopHero bind:heroInfo />
    </div>
</div>
<div class="section-wrapper">
    <div class="inner-hero general">
        <SubHeading headerInfo={subHeadingInfoFirstSection} />
        <VerticalLinkedDiagram bind:diagramSections />
    </div>
    <SubHeading headerInfo={subHeadingInfoSecondSection} />
    <div class="inner-hero grid">
        {#each values as value}
            <InfoBox infoBox={value} />
        {/each}
    </div>
</div>

<style lang="stylus">
@import "variables"

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
    padding 40px 20px
    margin 0 auto
    display flex
    flex-direction column

.main-hero-layout
    display grid
    grid-template-columns 1fr 1fr
    grid-gap 40px
    align-items center
    padding-top 60px
    padding-bottom 40px

.section-wrapper
    display flex
    flex-direction column
    border-top 1px solid #E6E6E6
    border-bottom 1px solid #E6E6E6

.general
    max-width 800px

.inner-hero.grid
    display grid
    grid-template-columns 1fr 1fr 1fr
    grid-gap 20px
    align-items center

</style>