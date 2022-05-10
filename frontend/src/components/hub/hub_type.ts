export type HubType = {
    link: string,
    tag: {
        text: string,
        color: 'red' | 'brand' | '' | 'blue',
    },
    title: {
        icon: any,
        text: string,
    },
    description: string,
}