import type { InfoBoxType } from '@components/small_components/small_components_types'

export type RightsDutiesIntro = {
    heading: string,
    paragraphs: string[],
    positiveRightsHeading: string,
    positiveRights: string[],
    positiveExplanation: string,
    negativeRightsHeading: string,
    negativeRights: string[],
    negativeExplanation: string,
    infoBox: InfoBoxType
}

export type HeaderInfoType = {
    heading: string,
    icon: any,
    subtitle: string
}