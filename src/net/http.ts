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
    let res = await invoke('sync_api', args)
    return res
}

const load_project = async (args: any) => {
    let res = await invoke('load_project', args)
    return res
}

const sync_check = async (args: any) => {
    let res = await invoke('sync_check', args)
    return res
}

const sync_data = async (args: any) => {
    let res = await invoke('sync_data', args)
    return res
}

export { request, start_login_server, sync_api, load_project, sync_check, sync_data }