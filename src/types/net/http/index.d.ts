import { InvokeArgs } from "@tauri-apps/api/tauri"


export interface Request extends InvokeArgs {
    protocol: string
    method: string
    host: string
    port: string
}

export interface Response {
}
