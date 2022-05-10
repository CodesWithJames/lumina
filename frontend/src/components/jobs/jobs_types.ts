export type InfoCardType = {
    icon: any,
    text: string,
}

export type JobType = {
    title: string,
    icon: any,
    attributes: InfoCardType[]
    link: string,
}

export type JobField = {
    title: string,
    jobs: JobType[],

}
