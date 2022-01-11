export type HeroType = {
    titleExcludingLastWord: string,
    highlightedWord: string,
    descriptionParagraphs: string[],
    buttons: { text: string, link: string }[],
    img: string
}