export type InfoCardType = {
    icon: any,
    text: string,
    color: string,
    opacity: string,
}

export type JobType = {
        title: string,
        icon: any,
        attributes: InfoCardType[]
}

export type JobField = {
    title: string,
    jobs: JobType[],

}
