export type InfoBoxType = {
    heading: string,
    paragraphs: string[],
}

export type IAmAPickerType = {
    heading: string,
    choices: {text: string, icon: any}[]
}

export type ScrollableDisplayCardType = {
    type: string,
    icon: any,
    title: string,
    description: string,
    tags: { question: string, answerText: string, answerIcon: any, color: string }[]
}

export type HeaderInfoType = {
    heading: string,
    icon: any,
    paragraphs: string[]
}