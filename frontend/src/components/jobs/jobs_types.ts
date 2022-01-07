export type InfoCard = {
    icon: any,
    text: string,
    color: string,
    opacity: string,
}

export type Job = {
        title: string,
        icon: any,
        attributes: InfoCard[]
}

export type JobField = {
    title: string,
    jobs: Job[],

}
