import type { HubType } from './hub_type'

import AccountGroup from "$icons/AccountGroup.svelte"
import Domain from "$icons/Domain.svelte"
import AccountOutline from "$icons/AccountOutline.svelte"
import Flash from "$icons/Flash.svelte"
import ScaleBalance from "$icons/ScaleBalance.svelte"
import Pillar from "$icons/Pillar.svelte"
import PassportBiometric from "$icons/PassportBiometric.svelte"
import Telescope from "$icons/Telescope.svelte"
import BadgeAccountHorizontal from "$icons/AccountBadgeHorizontal.svelte"
import CodeTags from "$icons/CodeTags.svelte"
import Clock from "$icons/Clock.svelte"
import Wifi from "$icons/Wifi.svelte"
import Brush from "$icons/Brush.svelte"
import Text from "$icons/Text.svelte"
import Hammer from "$icons/Hammer.svelte"
import ChevronRight from "$icons/ChevronRight.svelte"
import Settlement from "$lib/icons/Settlement.svelte"

export let two_wide: HubType[] = [
    {
        link: "/rights-and-duties",
        tag: {
            text: "Info",
            color: "blue",
        },
        title: {
            icon: ScaleBalance,
            text: "Rights & Duties",
        },
        description: "Learn about your rights and duties required for our society to function.",
    },
    {
        link: "/principles-and-values",
        tag: {
            text: "Info",
            color: "blue",
        },
        title: {
            icon: Pillar,
            text: "Principles & Values",
        },
        description: "Our mission, principles and values define the emotional, moral, and spiritual core of our society.",
    },
    {
        link: "/onboarding",
        tag: {
            text: "Services",
            color: "brand",
        },
        title: {
            icon: PassportBiometric,
            text: "Citizenship",
        },
        description: "Find out the benefits of becoming a citizen in Lumina. Get involved, along with thousands others today.",
    },
    {
        link: "/login",
        tag: {
            text: "Service",
            color: "brand",
        },
        title: {
            icon: AccountOutline,
            text: "myLumina",
        },
        description: "Lumina's cloud government portal that lets citizens access government services.",
    },
    {
        link: "",
        tag: {
            text: "Info",
            color: "blue",
        },
        title: {
            icon: Telescope,
            text: "Our Vision",
        },
        description: "Learn about your rights and duties required for our society to function.",
    },
    {
        link: "",
        tag: {
            text: "Info",
            color: "blue",
        },
        title: {
            icon: BadgeAccountHorizontal,
            text: "Media & Press",
        },
        description: "Are you a member of the press looking to speak to a Luminar representative? Contact us here.",
    }
]

export let three_wide: HubType[] = [
    {
        link: "",
        tag: {
            text: "Coming Soon",
            color: "",
        },
        title: {
            icon: ScaleBalance,
            text: "Legislation",
        },
        description: "Learn about your rights and duties required for our society to function.",
    },
    {
        link: "",
        tag: {
            text: "Coming Soon",
            color: "",
        },
        title: {
            icon: ScaleBalance,
            text: "Legislation",
        },
        description: "Learn about your rights and duties required for our society to function.",
    },
    {
        link: "",
        tag: {
            text: "Coming Soon",
            color: "",
        },
        title: {
            icon: ScaleBalance,
            text: "Legislation",
        },
        description: "Learn about your rights and duties required for our society to function.",
    },
    {
        link: "",
        tag: {
            text: "Coming Soon",
            color: "",
        },
        title: {
            icon: ScaleBalance,
            text: "Legislation",
        },
        description: "Learn about your rights and duties required for our society to function.",
    },
    {
        link: "",
        tag: {
            text: "Coming Soon",
            color: "",
        },
        title: {
            icon: ScaleBalance,
            text: "Legislation",
        },
        description: "Learn about your rights and duties required for our society to function.",
    },
    {
        link: "",
        tag: {
            text: "Coming Soon",
            color: "",
        },
        title: {
            icon: ScaleBalance,
            text: "Legislation",
        },
        description: "Learn about your rights and duties required for our society to function.",
    },
    {
        link: "",
        tag: {
            text: "Coming Soon",
            color: "",
        },
        title: {
            icon: ScaleBalance,
            text: "Legislation",
        },
        description: "Learn about your rights and duties required for our society to function.",
    },
    {
        link: "",
        tag: {
            text: "Coming Soon",
            color: "",
        },
        title: {
            icon: ScaleBalance,
            text: "Legislation",
        },
        description: "Learn about your rights and duties required for our society to function.",
    },
    {
        link: "",
        tag: {
            text: "Coming Soon",
            color: "",
        },
        title: {
            icon: ScaleBalance,
            text: "Legislation",
        },
        description: "Learn about your rights and duties required for our society to function.",
    },
]