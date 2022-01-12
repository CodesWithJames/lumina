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

import { tick } from "svelte"
import { loop_guard } from "svelte/internal"

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

let subHeadingInfo = {
    icon: ScriptText,
    heading: "Our principles",
    paragraphs: [
        "Principles are fundamental, universal truths that transcend time, geography, culture and context.",
        "They define the foundation for our society, and provide a base for our chain of reasoning."
    ]
}

let diagramSections = [
    {
        icon: RobotIndustrial,
        heading: "Pragmatism",
        description: "The right to free education in subjects: which do not cause division in society  and: lead to employment; or further scientific knowledge; or create technological advancement; or philosophy;",
        passedOnScroll: false
    },
    {
        icon: Bolt,
        heading: "Sustainability",
        description: "The right to free education in subjects: which do not cause division in society  and: lead to employment; or further scientific knowledge; or create technological advancement; or philosophy;",
        passedOnScroll: false
    },
    {
        icon: Sync,
        heading: "Iterate and improve",
        description: "Failure is part of the processs of iteration. Embrace and learn from failure.",
        passedOnScroll: false
    },
    {
        icon: Brain,
        heading: "Never stop learning",
        description: "The right to free education in subjects: which do not cause division in society  and: lead to employment; or further scientific knowledge; or create technological advancement; or philosophy;",
        passedOnScroll: false
    },
    {
        icon: ArmFlexOutline,
        heading: "Be bold",
        description: "The right to free education in subjects: which do not cause division in society  and: lead to employment; or further scientific knowledge; or create technological advancement; or philosophy;",
        passedOnScroll: false
    }
]



let scrollCycle: number = 0
function getScrollCycle(){
    scrollCycle = window.scrollY
    if (scrollCycle > 30) {
        makeDiagramSectionChangeColour()
        scrollCycle = 0
    }
}

function makeDiagramSectionChangeColour(){
    let i = 0
    for (let card in diagramSections) {
        let elem = document.getElementById(`diagramCard ${i}`)
        let bottomOfElem = elem.getBoundingClientRect().bottom
        if (bottomOfElem < 600) {
            diagramSections[i].passedOnScroll = true
        } else {
            diagramSections[i].passedOnScroll = false
        }
        i++
    }
}

</script>

<svelte:window on:scroll={getScrollCycle} />

<div class="hero-wrapper">
    <div class="inner-hero main-hero-layout">
        <TopHero bind:heroInfo />
    </div>
</div>
<div class="section-wrapper">
    <div class="inner-hero general">
        <SubHeading headerInfo={subHeadingInfo} />
        <VerticalLinkedDiagram bind:diagramSections />
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
    border-top 1px solid #E6E6E6
    border-bottom 1px solid #E6E6E6

.general
    max-width 800px
    
</style>