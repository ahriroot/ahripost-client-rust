export interface OpenTabMesagae<T> {
    id: string
    title: string
    type: string
    item: T
}

export interface Config {
    deleteNoConfirm: boolean
    showSideBar: boolean
    sideBarWidth: number
    apiAreaHeight: number
    pageSize: number
    lang: string
    token: string
    host: string
    client: string
}
