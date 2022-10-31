import { invoke } from "@tauri-apps/api/tauri"
import { Request, Response } from "@/types/net/http"

const request = async (request: Request) => {
    console.log('get', request)
    let res = await invoke<Response>('request', { request })
    return res
}

const start_login_server = async () => {
    let res = await invoke('start_login_server')
    return res
}

const sync_api = async (args: any) => {
    console.log(args)
    let res = await invoke('sync_api', args)
    return res
}

export { request, start_login_server, sync_api }