import { InvokeArgs } from "@tauri-apps/api/tauri"


export interface Request extends InvokeArgs {
    protocol: string
    method: string
    host: string
    port: string
    path: string
    params: {
        field: string
        value: string
    }[]
    headers: {
        field: string
        value: string
    }[]
    body_type: string
    form: {
        field: string
        value: string
        value_type: string
        file: string | string[]
    }[]
    json: string
}

export interface Response {
    body: string
    canonical_reason: string
    headers: any[]
    status: number
    version: string
}
