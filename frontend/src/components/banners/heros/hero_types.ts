

export type HeroType = {
    subtitle: string,
    titleExcludingLastWord: string,
    highlightedWord: string,
    descriptionParagraphs: string[],
    buttons: { 
        text: string, 
        link: string, 
        rightIcon?: any,
        gradient?: boolean,
        branded?: boolean,
    }[],
    img: string
}