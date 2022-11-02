<script setup lang="ts">
import { h, ref, shallowRef, onMounted, onBeforeMount, computed, watch } from 'vue'
import {
    NLayout, NH2, NInputGroup, NButton, NInput, useDialog, NSpin, NIcon,
    NSelect, NTabs, NTabPane, NDataTable, SelectOption, DataTableColumns,
    NRadioGroup, NSpace, NRadio, NUpload, NPopover
} from 'naive-ui'
import { FolderOutline, Add, Remove } from '@vicons/ionicons5'
import AInput from '@/components/AInput.vue'
import ACheckbox from '@/components/ACheckbox.vue'
import Item from '@/models/Item'
import { useMessage } from 'naive-ui'
import { useI18n } from 'vue-i18n'
import { useIndexStore } from '@/store'
import { request, sync_api } from '@/net/http'
import { Request, Response } from '@/types/net/http'
import Editor from '@/components/Editor.vue'
import Project from '@/models/Project'

window.$message = useMessage()
const store = useIndexStore()
const data = ref<any>({
    request: {
        body: {
            type: 'form',
            form: [],
            json: ''
        },
    },
    response: {
        body: {
            type: 'form',
            html: '',
            json: ''
        }
    },
})

const props = defineProps<{
    item: any
}>()
const emits = defineEmits<{
    // (e: 'handle', id: any): void
    (e: 'handleCloseTab', ev: null, id: string): void
}>()

onBeforeMount(async () => {
    height.value = store.config.apiAreaHeight

    let res = await Item.where({ key: props.item }).get()
    data.value = res

    data.value.request.body.form = data.value.request.body.form.map((item: any) => {
        item.file = null
        return item
    })
})

const options = shallowRef<SelectOption[]>([
    {
        label: 'GET',
        value: 'GET',
    },
    {
        label: 'POST',
        value: 'POST',
    },
    {
        label: 'PUT',
        value: 'PUT',
    },
    {
        label: 'DELETE',
        value: 'DELETE',
    },
    {
        label: 'PATCH',
        value: 'PATCH',
    }
])

const columns = ref<DataTableColumns<any>>([
    {
        type: 'selection',
    },
    {
        title: 'Key',
        key: 'key',
        render(row: any, index: number) {
            return h('div',
                {
                    class: 'input'
                },
                [
                    h(AInput, {
                        value: row.field,
                        placeholder: 'Key',
                        onUpdateValue: (val: any) => {
                            row.field = val
                        }
                    })
                ]
            )
        }
    },
    {
        title: 'Value',
        key: 'value',
        render(row: any, index: number) {
            return h('div',
                {
                    class: 'input'
                },
                [
                    h(AInput, {
                        value: row.value,
                        placeholder: 'Value',
                        onUpdateValue: (val: any) => {
                            row.value = val
                        }
                    })
                ]
            )
        }
    },
    {
        title: 'Default',
        key: 'default',
        render(row: any, index: number) {
            return h('div',
                {
                    class: 'input'
                },
                [
                    h(AInput, {
                        value: row.default,
                        placeholder: 'Default',
                        onUpdateValue: (val: any) => {
                            row.default = val
                        }
                    })
                ]
            )
        }
    },
    {
        title: 'Describe',
        key: 'describe',
        render(row: any, index: number) {
            return h('div',
                {
                    class: 'input'
                },
                [
                    h(AInput, {
                        value: row.describe,
                        placeholder: 'Describe',
                        onUpdateValue: (val: any) => {
                            row.describe = val
                        }
                    })
                ]
            )
        }
    },
    {
        title: 'Must',
        key: 'must',
        align: 'center',
        width: 60,
        render(row: any, index: number) {
            return h('div',
                {
                    class: 'input',
                },
                [
                    h(ACheckbox, {
                        value: row.must,
                        onUpdateValue: (val: any) => {
                            row.must = val
                        }
                    })
                ]
            )
        }
    },
    {
        key: 'title',
        align: 'center',
        width: 34,
        render(row: any, index: number) {
            return h('div',
                {
                    class: 'input',
                },
                [
                    h(NButton, {
                        size: 'small',
                        quaternary: true,
                        onClick: () => {
                            data.value.request.body.form.push({
                                key: window.crypto.randomUUID(),
                                checked: true,
                                field: '',
                                value: '',
                                describe: '',
                                default: '',
                                must: true
                            })
                        }
                    }, {
                        default: () => h(
                            NIcon,
                            {},
                            {
                                default: () => h(Remove)
                            }
                        )
                    })
                ]
            )
        },
        title() {
            return h('div',
                {
                    class: 'input',
                },
                [
                    h(NButton, {
                        size: 'small',
                        quaternary: true,
                        onClick: () => {
                            if (data.value.request.tab === 'param') {
                                data.value.request.params.push({
                                    key: window.crypto.randomUUID(),
                                    checked: true,
                                    field: '',
                                    value: '',
                                    describe: '',
                                    default: '',
                                    must: true
                                })
                            } else if (data.value.request.tab === 'header') {
                                data.value.request.headers.push({
                                    key: window.crypto.randomUUID(),
                                    checked: true,
                                    field: '',
                                    value: '',
                                    describe: '',
                                    default: '',
                                    must: true
                                })
                            } else if (data.value.request.tab === 'body' && data.value.request.body.type === 'form') {
                                data.value.request.body.form.push({
                                    key: window.crypto.randomUUID(),
                                    checked: true,
                                    field: '',
                                    value: '',
                                    describe: '',
                                    default: '',
                                    must: true
                                })
                            }
                        }
                    }, {
                        default: () => h(
                            NIcon,
                            {},
                            {
                                default: () => h(Add)
                            }
                        )
                    })
                ]
            )
        }
    }
])
const columnsResponseHeaders = ref<DataTableColumns<any>>([
    {
        title: 'Key',
        key: 'field',
        render(row: any, index: number) {
            return h('div',
                {
                    class: 'input'
                },
                [
                    h(AInput, {
                        value: row.field,
                        placeholder: 'Key',
                        clearable: false,
                        onUpdateValue: (val: any) => {
                            row.field = val
                        }
                    })
                ]
            )
        }
    },
    {
        title: 'Value',
        key: 'value',
        render(row: any, index: number) {
            return h('div',
                {
                    class: 'input'
                },
                [
                    h(AInput, {
                        value: row.value,
                        placeholder: 'Value',
                        clearable: false,
                        onUpdateValue: (val: any) => {
                            row.field = val
                        }
                    })
                ]
            )
        }
    }
])
const columnsForm = ref<DataTableColumns<any>>([
    {
        type: 'selection',
    },
    {
        title: 'Key',
        key: 'key',
        render(row: any, index: number) {
            return h('div',
                {
                    class: 'input'
                },
                [
                    h(AInput, {
                        value: row.field,
                        placeholder: 'Key',
                        onUpdateValue: (val: any) => {
                            row.field = val
                        }
                    })
                ]
            )
        }
    },
    {
        title: 'Value',
        key: 'value',
        minWidth: 300,
        render(row: any, index: number) {
            let node
            if (row.type == 'text') {
                node = h(AInput, {
                    value: row.value,
                    placeholder: 'Value',
                    onUpdateValue: (val: any) => {
                        row.value = val
                    }
                })
            } else if (row.type == 'file') {
                node = h(NUpload, {
                    multiple: true,
                    size: 'small',
                    defaultUpload: false,
                    showFileList: false,
                    onUpdateFileList: (val: any) => {
                        row.file = val
                    }
                }, {
                    default: () => h(NPopover, {
                    }, {
                        trigger: () => h(
                            NButton,
                            {
                                size: 'small',
                                tertiary: true,
                            },
                            {
                                default: () => {
                                    if (row.file) {
                                        return row.file[0].name
                                    }
                                    return 'Upload'
                                }
                            }
                        ),
                        default: () => {
                            if (row.file) {
                                let filelist = row.file.map((item: any) => {
                                    return h('div', {}, item.file.name)
                                })
                                return filelist
                            }
                            return '[no file]'
                        }
                    })
                })
            }
            return h('div',
                {
                    class: 'input',
                    style: {
                        display: 'flex'
                    }
                },
                [
                    h(NSelect, {
                        value: row.type,
                        size: 'small',
                        style: {
                            width: '100px'
                        },
                        options: [{
                            label: "Text",
                            value: 'text'
                        }, {
                            label: "File",
                            value: 'file'
                        }],
                        onUpdateValue: (val: any) => {
                            row.type = val
                        }
                    }),
                    node
                ]
            )
        }
    },
    {
        title: 'Default',
        key: 'default',
        render(row: any, index: number) {
            return h('div',
                {
                    class: 'input'
                },
                [
                    h(AInput, {
                        value: row.default,
                        placeholder: 'Default',
                        onUpdateValue: (val: any) => {
                            row.default = val
                        }
                    })
                ]
            )
        }
    },
    {
        title: 'Describe',
        key: 'describe',
        render(row: any, index: number) {
            return h('div',
                {
                    class: 'input'
                },
                [
                    h(AInput, {
                        value: row.describe,
                        placeholder: 'Describe',
                        onUpdateValue: (val: any) => {
                            row.describe = val
                        }
                    })
                ]
            )
        }
    },
    {
        title: 'Must',
        key: 'must',
        align: 'center',
        width: 60,
        render(row: any, index: number) {
            return h('div',
                {
                    class: 'input',
                },
                [
                    h(ACheckbox, {
                        value: row.must,
                        onUpdateValue: (val: any) => {
                            row.must = val
                        }
                    })
                ]
            )
        }
    },
    {
        key: 'title',
        align: 'center',
        width: 34,
        render(row: any, index: number) {
            return h('div',
                {
                    class: 'input',
                },
                [
                    h(NButton, {
                        size: 'small',
                        quaternary: true,
                        onClick: () => {
                            data.value.request.body.form.splice(index, 1)
                        }
                    }, {
                        default: () => h(
                            NIcon,
                            {},
                            {
                                default: () => h(Remove)
                            }
                        )
                    })
                ]
            )
        },
        title() {
            return h('div',
                {
                    class: 'input',
                },
                [
                    h(NButton, {
                        size: 'small',
                        quaternary: true,
                        onClick: () => {
                            let key = window.crypto.randomUUID()
                            if (data.value.request.tab === 'param') {
                                data.value.request.params.push({
                                    key: key,
                                    checked: true,
                                    field: '',
                                    value: '',
                                    describe: '',
                                    default: '',
                                    must: true
                                })
                                data.value.request.params_keys.push(key)
                            } else if (data.value.request.tab === 'header') {
                                data.value.request.headers.push({
                                    key: key,
                                    checked: true,
                                    field: '',
                                    value: '',
                                    describe: '',
                                    default: '',
                                    must: true
                                })
                                data.value.request.headers_keys.push(key)
                            } else if (data.value.request.tab === 'body' && data.value.request.body.type === 'form') {
                                data.value.request.body.form.push({
                                    key: key,
                                    checked: true,
                                    field: '',
                                    value: '',
                                    type: 'text',
                                    file: null,
                                    describe: '',
                                    default: '',
                                    must: true,
                                })
                                data.value.request.body.form_keys.push(key)
                            }
                        }
                    }, {
                        default: () => h(
                            NIcon,
                            {},
                            {
                                default: () => h(Add)
                            }
                        )
                    })
                ]
            )
        }
    }
])

const showLoading = ref<boolean>(false)
const href = computed({
    get() {
        return data.value.request?.path || ''
    },
    set(val: string) {
        data.value.request.path = val
    }
})

// watch(() => data.value.request?.params, async (_) => {

// }, {
//     immediate: false,
//     deep: true,
// })
const handleSend = async () => {
    let url = new URL(data.value.request.path)
    let search = data.value.request.params.filter((item: any) => data.value.request.params_keys.includes(item.key)).map((item: any) => {
        return `${item.field}=${item.value}`
    }).join('&')
    let params = data.value.request.params.filter((item: any) => data.value.request.params_keys.includes(item.key)).map((item: any) => {
        return {
            field: item.field,
            value: item.value
        }
    })
    let headers = data.value.request.headers.filter((item: any) => data.value.request.headers_keys.includes(item.key)).map((item: any) => {
        return {
            field: item.field,
            value: item.value
        }
    })
    if (!headers.some((item: any) => item.field.trim().toLowerCase() === 'content-type')) {
        if (data.value.request.body.type === 'form') {
            headers.push({
                field: 'Content-Type',
                value: 'application/x-www-form-urlencoded'
            })
        } else if (data.value.request.body.type === 'json') {
            headers.push({
                field: 'Content-Type',
                value: 'application/json'
            })
        }
    }
    let form = data.value.request.body.form.filter((item: any) => data.value.request.body.form_keys.includes(item.key)).map((item: any) => {
        let file = []
        if (item.file) {
            file = item.file.map((file: any) => {
                return file.file
            })
        }
        return {
            field: item.field,
            value: item.value,
            value_type: item.type,
            file: file
        }
    })
    let args: Request = {
        protocol: url.protocol,
        method: data.value.request.method,
        host: url.hostname,
        port: url.port,
        path: url.pathname,
        params: params,
        headers: headers,
        body_type: data.value.request.body.type,
        form: form,
        json: data.value.request.body.json,
    }
    showLoading.value = true
    let response = await request(args)
    showLoading.value = false

    data.value.response.status = response.status
    data.value.response.statusText = response.canonical_reason
    data.value.response.headers = response.headers
    data.value.response.tab = 'body'

    let contentType = response.headers.find((item: any) => item.field.toLowerCase() === 'content-type')
    if (contentType) {
        if (contentType.value.includes('application/json')) {
            data.value.response.body.type = 'pretty'
            data.value.response.body.json = response.body
        } else if (contentType.value.includes('text/html')) {
            data.value.response.body.type = 'preview'
            data.value.response.body.html = response.body
        } else {
            data.value.response.body.type = 'raw'
            data.value.response.body.text = response.body
        }
    } else {
        data.value.response.body.type = 'raw'
        data.value.response.body.text = response.body
    }

    if (responseRef.value && data.value.response?.body) {
        responseRef.value.setValue(response.body || '')
    }

    let obj: any = await Item.where({ key: props.item }).obj()
    obj.last_update = new Date().getTime()
    obj.request = JSON.parse(JSON.stringify(data.value.request))
    obj.response = JSON.parse(JSON.stringify(data.value.response))
    obj.tag = false
    obj.client = store.config.client
    obj.save()
}
const handleChangeType = (_: string) => {
    requestRef.value.setValue(data.value.request.body.json || '')
}

const tabApiRef = shallowRef<HTMLElement | null>(null)
const topRef = shallowRef<HTMLElement | null>(null)
const bottomRef = shallowRef<HTMLElement | null>(null)
const resizeable = ref<boolean>(false)
const height = ref(store.config.apiAreaHeight)
const oldHeight = ref(250)
const cursor = ref('default')
const currentMoveY = ref(0)
const requestRef = shallowRef<any>(null)
const responseRef = shallowRef<any>(null)
onMounted(async () => {
    if (topRef.value && bottomRef.value) {
        topRef.value.addEventListener('mousedown', (ev) => {
            if (cursor.value == 'ns-resize') {
                resizeable.value = true
                currentMoveY.value = ev.clientY
                oldHeight.value = height.value
            }
        })
        bottomRef.value.addEventListener('mousedown', (ev) => {
            if (cursor.value == 'ns-resize') {
                resizeable.value = true
                currentMoveY.value = ev.clientY
                oldHeight.value = height.value
            }
        })
        document.body.addEventListener('mousemove', (ev) => {
            if (topRef.value && (height.value - 6 < ev.offsetY && ev.offsetY < height.value + 4)) {
                cursor.value = 'ns-resize'
            } else {
                cursor.value = 'default'
            }
            if (resizeable.value) {
                const tmp = oldHeight.value + ev.clientY - currentMoveY.value
                if (tmp < 150) {
                    height.value = 150
                } else if (tmp > document.body.offsetHeight - 100) {
                    height.value = document.body.offsetHeight - 100
                } else {
                    height.value = tmp
                }
            }
        })
        document.body.addEventListener('mouseup', (_) => {
            resizeable.value = false
            store.updateConfig({
                ...store.config,
                apiAreaHeight: height.value
            })
        })
    }
    setTimeout(() => {
        if (requestRef.value && data.value.request?.body) {
            requestRef.value.setValue(data.value.request.body.json || '')
        }
        if (responseRef.value && data.value.response?.body) {
            responseRef.value.setValue(data.value.response.body.json || '')
        }
    }, 1000)

    tabApiRef.value?.addEventListener('keydown', async (ev) => {
        if (ev.ctrlKey && ev.key == 's') {
            ev.preventDefault()
            let obj: any = await Item.where({ key: props.item }).obj()
            obj.last_update = new Date().getTime()
            obj.request = JSON.parse(JSON.stringify(data.value.request))
            obj.response = JSON.parse(JSON.stringify(data.value.response))
            obj.tag = false
            obj.client = store.config.client
            obj.save()
        }
    })
})

const handleSync = async () => {
    let obj: any = await Item.where({ key: props.item }).obj()
    obj.last_sync = new Date().getTime()
    obj.request = JSON.parse(JSON.stringify(data.value.request))
    obj.response = JSON.parse(JSON.stringify(data.value.response))
    obj.tag = false
    obj.client = store.config.client
    obj.save()

    let ids: string[] = []
    let item: any = await Item.where({ key: props.item }).get()
    ids.unshift(item.key)

    while (item.parent) {
        item = await Item.where({ key: item.parent }).get()
        ids.unshift(item.key)
    }
    let apis: any[] = []
    for (let i = 0; i < ids.length; i++) {
        let api: any = await Item.where({ key: ids[i] }).get()
        api.request = JSON.stringify(api.request)
        api.response = JSON.stringify(api.response)
        apis.push(api)
    }
    if (apis.length > 0) {
        let project: any = await Project.where({ id: apis[0].project }).get()
        let res: any = await sync_api({
            data: { apis: apis, project: project },
            server: 'http://127.0.0.1:8080',
            token: store.config.token || ''
        })
        if (res.data.items_update && res.data.items_update.length > 0) {
            for (let i = 0; i < res.data.items_update.length; i++) {
                let item: any = await Item.where({ key: res.data.items_update[i].key }).obj()
                item.last_sync = res.data.items_update[i].last_sync
                item.save()
            }
        }
        if (res.data.items_sync && res.data.items_sync.length > 0) {
            for (let i = 0; i < res.data.items_sync.length; i++) {
                let item: any = await Item.where({ key: res.data.items_sync[i].key }).obj()
                item.label = res.data.items_sync[i].label
                item.last_sync = res.data.items_sync[i].last_sync
                item.request = JSON.parse(res.data.items_sync[i].request)
                item.response = JSON.parse(res.data.items_sync[i].response)
                item.save()
            }
        }
    } else {
        window.$message.warning('no api need to sync')
    }
}
</script>

<template>
    <div class="tab-api" ref="tabApiRef" tabindex="1">
        <div ref="topRef" class="top" :style="`height: ${height - 2}px; cursor: ${resizeable ? 'ns-resize' : cursor}`">
            <div class="title" style="display: flex; justify-content: space-between; align-items: center;">
                <n-input-group>
                    <n-input v-model:value="data.label" placeholder="Label" />
                    <n-input v-model:value="data.request.describe" placeholder="Describe" />
                    <span>
                        <n-button secondary @click="handleSync">SYNC</n-button>
                    </span>
                </n-input-group>
            </div>
            <div class="location">
                <n-input-group>
                    <n-select v-model:value="data.request.method" :options="options" style="width: 150px" />
                    <n-input v-model:value="href" placeholder="Location" />
                    <n-button secondary @click="handleSend">SEND</n-button>
                </n-input-group>
            </div>
            <n-tabs style="top: 72px; bottom: 0" v-model:value="data.request.tab">
                <n-tab-pane name="header" display-directive="show">
                    <template #tab>
                        <div style="padding: 0 10px">
                            <span>Header</span>
                        </div>
                    </template>
                    <n-layout position="absolute" style="top: 0; bottom: 0; background: #21252b"
                        :native-scrollbar="false">
                        <n-data-table v-if="data.request?.headers" size="small" :columns="columns"
                            v-model:checked-row-keys="data.request.headers_keys" :data="data.request.headers"
                            :single-line="false" :bordered="false" />
                    </n-layout>
                </n-tab-pane>
                <n-tab-pane name="param" display-directive="show">
                    <template #tab>
                        <div style="padding: 0 10px">
                            <span>Param</span>
                        </div>
                    </template>
                    <n-layout position="absolute" style="top: 0; bottom: 0; background: #21252b"
                        :native-scrollbar="false">
                        <n-data-table v-if="data.request?.params" size="small" :columns="columns"
                            v-model:checked-row-keys="data.request.params_keys" :data="data.request.params"
                            :single-line="false" :bordered="false" />
                    </n-layout>
                </n-tab-pane>
                <n-tab-pane name="body" display-directive="show">
                    <template #tab>
                        <div style="padding: 0 10px">
                            <span>Body</span>
                        </div>
                    </template>
                    <n-radio-group v-model:value="data.request.body.type"
                        style="position: absolute; top: 6px; left: 4px" name="radiogroup"
                        @update:value="handleChangeType">
                        <n-space>
                            <n-radio v-for="song in [
                                { label: 'None', value: 'none' },
                                { label: 'JSON', value: 'json' },
                                { label: 'Form', value: 'form' },
                                { label: 'Text', value: 'text' },
                                { label: 'XML', value: 'xml' },
                                { label: 'Binary', value: 'binary' },
                            ]" :key="song.value" :value="song.value">
                                {{ song.label }}
                            </n-radio>
                        </n-space>
                    </n-radio-group>
                    <div v-show="data.request.body.type == 'none'"></div>
                    <div v-show="data.request.body.type == 'json'"
                        style="position: absolute; top: 30px; left: 0; bottom: 0; right: 0">
                        <Editor ref="requestRef" @change="(val) => data.request.body.json = val"
                            :value="data.request.body.json" />
                    </div>
                    <n-layout v-show="data.request.body.type == 'form'" position="absolute"
                        style="top: 30px; bottom: 0; background: #21252b" :native-scrollbar="false">
                        <n-data-table v-if="data.request?.params" size="small" :columns="columnsForm"
                            v-model:checked-row-keys="data.request.body.form_keys" :data="data.request.body.form"
                            :single-line="false" :bordered="false" />
                    </n-layout>
                </n-tab-pane>
            </n-tabs>
        </div>
        <div ref="bottomRef" class="bottom" :style="`top: ${height}px`">
            <n-spin :show="showLoading">
                <n-layout position="absolute" style="top: 0; bottom: 0; background: #21252b" :native-scrollbar="false">
                    <n-tabs style="top: 0; bottom: 0" v-model:value="data.response.tab">
                        <n-tab-pane name="header" display-directive="show">
                            <template #tab>
                                <div style="padding: 0 10px">
                                    <span>Header</span>
                                </div>
                            </template>
                            <n-layout position="absolute" style="top: 0; bottom: 0; background: #21252b"
                                :native-scrollbar="false">
                                <n-data-table v-if="data.response?.headers" size="small"
                                    :columns="columnsResponseHeaders" :data="data.response.headers" :single-line="false"
                                    :bordered="false" />
                            </n-layout>
                        </n-tab-pane>
                        <n-tab-pane name="body" display-directive="show">
                            <template #tab>
                                <div style="padding: 0 10px">
                                    <span>Body</span>
                                </div>
                            </template>
                            <n-radio-group v-model:value="data.response.body.type"
                                style="position: absolute; top: 6px; left: 4px" name="radiogroup"
                                @update:value="handleChangeType">
                                <n-space>
                                    <n-radio v-for="song in [
                                        { label: 'Pretty', value: 'pretty' },
                                        { label: 'Raw', value: 'raw' },
                                        { label: 'Preview', value: 'preview' },
                                    ]" :key="song.value" :value="song.value">
                                        {{ song.label }}
                                    </n-radio>
                                </n-space>
                            </n-radio-group>
                            <div v-show="data.response.body.type == 'pretty'"
                                style="position: absolute; top: 30px; left: 0; bottom: 0; right: 0">
                                <Editor ref="responseRef" @change="(val) => data.response.body.json = val"
                                    :value="data.response.body.json" />
                            </div>
                            <div v-show="data.response.body.type == 'raw'"
                                style="position: absolute; top: 30px; left: 0; bottom: 0; right: 0">
                                <pre>{{ data.response.body.text }}</pre>
                            </div>
                            <div v-show="data.response.body.type == 'preview'"
                                style="position: absolute; top: 30px; left: 0; bottom: 0; right: 0">
                                <pre>{{ data.response.body.html }}</pre>
                            </div>
                        </n-tab-pane>
                        <template #suffix>
                            {{ data.response.status }}&nbsp;{{ data.response.statusText }}&nbsp;&nbsp;
                        </template>
                    </n-tabs>
                </n-layout>
            </n-spin>
        </div>
    </div>
</template>

<style scoped>
.tab-api {
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
}

.tab-api .top {
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    background: #21252b;
    border-bottom: 4px solid #333842;
}

.tab-api .bottom {
    position: absolute;
    left: 0;
    right: 0;
    bottom: 0;
    background: #21252b;
}

.tab-api .top {
    height: 100%;
}

.tab-api .top .title {
    height: 36px;
}

.tab-api .top .location {
    height: 36px;
}
</style>
