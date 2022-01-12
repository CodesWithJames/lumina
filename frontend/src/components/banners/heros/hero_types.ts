export type HeroType = {
    titleExcludingLastWord: string,
    subtitle: string,
    highlightedWord: string,
    descriptionParagraphs: string[],
    buttons: { text: string, link: string }[],
    img: string
}