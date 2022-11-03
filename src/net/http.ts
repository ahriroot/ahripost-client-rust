import { invoke } from "@tauri-apps/api/tauri"
import { Request, Response } from "@/types/net/http"

const request = async (request: Request) => {
    let res = await invoke<Response>('request', { request })
    return res
}

const start_login_server = async () => {
    let res = await invoke('start_login_server')
    return res
}

const load_project = async (args: any) => {
    let res = await invoke('load_project', args)
    return res
}

interface Result {
    error?: string
    code?: number
    msg?: string
    data?: any
}

const opera = async (url: string, args: any) => {
    let res = await invoke<Result>(url, args)
    if (res?.error && typeof res.error === 'string') {
        window.$message.error(res.error)
    } else if (typeof res?.code == 'number' && (res.code >= 20000 || res.code < 10000)) {
        window.$message.error(res.msg)
    }
    return res
}

const sync_check = async (args: any) => {
    return await opera('sync_check', args)
}

const sync_data = async (args: any) => {
    return await opera('sync_data', args)
}

const sync_api = async (args: any) => {
    return await opera('sync_api', args)
}

const delete_api = async (args: any) => {
    return await opera('delete_api', args)
}

export { request, start_login_server, load_project, sync_check, sync_data, sync_api, delete_api }