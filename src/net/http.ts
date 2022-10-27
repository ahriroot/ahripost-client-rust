import { invoke } from "@tauri-apps/api/tauri"
import { Request, Response } from "@/types/net/http"

const get = async (request: Request) => {
    let res = await invoke<Response>('get', { request })
    return res
}

const post = async (request: Request) => {
    let res = await invoke<Response>('post', { request })
    return res
}

const put = async (request: Request) => {
    let res = await invoke<Response>('put', { request })
    return res
}

const del = async (request: Request) => {
    let res = await invoke<Response>('delete', { request })
    return res
}

export { get, post, put, del }