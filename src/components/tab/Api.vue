<script setup lang="ts">
import { h, ref, shallowRef, onMounted, onBeforeMount, computed, watch } from 'vue'
import {
    NLayout, NH2, NInputGroup, NButton, NInput, useDialog, NSpin, NIcon,
    NSelect, NTabs, NTabPane, NDataTable, SelectOption, DataTableColumns,
    NRadioGroup, NSpace, NRadio, NUpload, NUploadFileList, NPopover
} from 'naive-ui'
import { FolderOutline, Add, Remove } from '@vicons/ionicons5'
import { nanoid } from 'nanoid'
import AInput from '@/components/AInput.vue'
import ACheckbox from '@/components/ACheckbox.vue'
import Item from '@/models/Item'
import { useMessage } from 'naive-ui'
import { useI18n } from 'vue-i18n'
import { useIndexStore } from '@/store'
import { get, post, put, del } from '@/net/http'
import { Request, Response } from '@/types/net/http'
import Editor from '@/components/Editor.vue'

window.$message = useMessage()
const store = useIndexStore()
const dialog = useDialog()
const data = ref<any>({
    detail: {
        body: {
            type: 'form',
            form: [],
            json: ''
        }
    }
})

const props = defineProps<{
    item: any
}>()
const emits = defineEmits<{
    // (e: 'handle', id: any): void
}>()

onBeforeMount(async () => {
    height.value = store.config.apiAreaHeight

    let res = await Item.where({ id: props.item }).get()
    data.value = res

    data.value.detail.body.form = data.value.detail.body.form.map((item: any) => {
        item.file = null
        return item
    })
})

const method = ref<string>('GET')
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
                        onUpdateValue: (val: any) => {
                            row.value = val
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
                        onUpdateValue: (val: any) => {
                            row.describe = val
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
                        onUpdateValue: (val: any) => {
                            row.default = val
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
                            data.value.detail.body.form.push({
                                key: nanoid(),
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
                            if (data.value.detail.tab === 'param') {
                                data.value.detail.params.push({
                                    key: nanoid(),
                                    checked: true,
                                    field: '',
                                    value: '',
                                    describe: '',
                                    default: '',
                                    must: true
                                })
                            } else if (data.value.detail.tab === 'header') {
                                data.value.detail.headers.push({
                                    key: nanoid(),
                                    checked: true,
                                    field: '',
                                    value: '',
                                    describe: '',
                                    default: '',
                                    must: true
                                })
                            } else if (data.value.detail.tab === 'body' && data.value.detail.body.type === 'form') {
                                data.value.detail.body.form.push({
                                    key: nanoid(),
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
                        console.log(data.value.detail.body.form)
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
                            console.log(val)
                            row.type = val
                        }
                    }),
                    node
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
                        onUpdateValue: (val: any) => {
                            row.describe = val
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
                        onUpdateValue: (val: any) => {
                            row.default = val
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
                            data.value.detail.body.form.push({
                                key: nanoid(),
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
                            if (data.value.detail.tab === 'param') {
                                data.value.detail.params.push({
                                    key: nanoid(),
                                    checked: true,
                                    field: '',
                                    value: '',
                                    describe: '',
                                    default: '',
                                    must: true
                                })
                            } else if (data.value.detail.tab === 'header') {
                                data.value.detail.headers.push({
                                    key: nanoid(),
                                    checked: true,
                                    field: '',
                                    value: '',
                                    describe: '',
                                    default: '',
                                    must: true
                                })
                            } else if (data.value.detail.tab === 'body' && data.value.detail.body.type === 'form') {
                                data.value.detail.body.form.push({
                                    key: nanoid(),
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

const showLoading = ref<boolean>(false)
const href = computed({
    get() {
        return data.value.detail?.path || ''
    },
    set(val: string) {
        data.value.detail.path = val
    }
})

// watch(() => data.value.detail?.params, async (_) => {

// }, {
//     immediate: false,
//     deep: true,
// })
const handleSend = async () => {
    showLoading.value = true
    let response = await get({
        protocol: 'http:',
        method: data.value.detail.method,
        host: 'localhost',
        port: '3000',
    })
    showLoading.value = false
    console.log(response)
}
const handleChangeType = (val: string) => {
    editorRef.value.setValue(data.value.detail.body.json || '')
}

const tabApiRef = shallowRef<HTMLElement | null>(null)
const topRef = shallowRef<HTMLElement | null>(null)
const bottomRef = shallowRef<HTMLElement | null>(null)
const resizeable = ref<boolean>(false)
const height = ref(store.config.apiAreaHeight)
const oldHeight = ref(250)
const cursor = ref('default')
const currentMoveY = ref(0)
const editorRef = shallowRef<any>(null)
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
        if (editorRef.value && data.value.detail?.body) {
            editorRef.value.setValue(data.value.detail.body.json || '')
        }
    }, 1000)

    tabApiRef.value?.addEventListener('keydown', async (ev) => {
        if (ev.ctrlKey && ev.key == 's') {
            ev.preventDefault()
            let obj: any = await Item.where({ id: props.item }).obj()
            data.value.detail.updated = new Date().getTime()
            obj.detail = JSON.parse(JSON.stringify(data.value.detail))
            obj.save()
        }
    })
})
</script>

<template>
    <div class="tab-api" ref="tabApiRef" tabindex="1">
        <div ref="topRef" class="top" :style="`height: ${height - 2}px; cursor: ${resizeable ? 'ns-resize' : cursor}`">
            <div class="title" style="display: flex; justify-content: space-between; align-items: center;">
                <span style="padding-left: 10px">{{ data.label }}</span>
                <span>env</span>
            </div>
            <div class="location">
                <n-input-group>
                    <n-select v-model:value="method" :options="options" style="width: 150px" />
                    <n-input v-model:value="href" placeholder="Location" />
                    <n-button secondary @click="handleSend">SEND</n-button>
                </n-input-group>
            </div>
            <n-tabs style="top: 72px; bottom: 6px" v-model:value="data.detail.tab">
                <n-tab-pane name="header" display-directive="show">
                    <template #tab>
                        <div style="padding: 0 10px">
                            <span>Header</span>
                        </div>
                    </template>
                    <n-layout position="absolute" style="top: 0; bottom: 0; background: #21252b"
                        :native-scrollbar="false">
                        <n-data-table v-if="data.detail?.headers" size="small" :columns="columns"
                            v-model:checked-row-keys="data.detail.headers_keys" :data="data.detail.headers"
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
                        <n-data-table v-if="data.detail?.params" size="small" :columns="columns"
                            v-model:checked-row-keys="data.detail.params_keys" :data="data.detail.params"
                            :single-line="false" :bordered="false" />
                    </n-layout>
                </n-tab-pane>
                <n-tab-pane name="body" display-directive="show">
                    <template #tab>
                        <div style="padding: 0 10px">
                            <span>Body</span>
                        </div>
                    </template>
                    <n-radio-group v-model:value="data.detail.body.type" style="position: absolute; top: 6px; left: 4px"
                        name="radiogroup" @update:value="handleChangeType">
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
                    <div v-show="data.detail.body.type == 'none'"></div>
                    <div v-show="data.detail.body.type == 'json'"
                        style="position: absolute; top: 30px; left: 0; bottom: 0; right: 0">
                        <Editor ref="editorRef" @change="(val) => data.detail.body.json = val"
                            :value="data.detail.body.json" />
                    </div>
                    <n-layout v-show="data.detail.body.type == 'form'" position="absolute"
                        style="top: 30px; bottom: 0; background: #21252b" :native-scrollbar="false">
                        <n-data-table v-if="data.detail?.params" size="small" :columns="columnsForm"
                            v-model:checked-row-keys="data.detail.body.form_keys" :data="data.detail.body.form"
                            :single-line="false" :bordered="false" />
                    </n-layout>
                </n-tab-pane>
            </n-tabs>
        </div>
        <div ref="bottomRef" class="bottom" :style="`top: ${height}px`">
            <n-spin :show="showLoading">
                <n-layout position="absolute" style="top: 0; bottom: 0; background: #21252b" :native-scrollbar="false">
                    <div class="result">
                        {{ data }}
                    </div>
                    <n-h2>12</n-h2>
                    <n-h2>12</n-h2>
                    <n-h2>12</n-h2>
                    <n-h2>12</n-h2>
                    <n-h2>12</n-h2>
                    <n-h2>12</n-h2>
                    <n-h2>12</n-h2>
                    <n-h2>12</n-h2>
                    <n-h2>12</n-h2>
                    <n-h2>12</n-h2>
                    <n-h2>12</n-h2>
                    <iframe border="0" style="border: none">
                        <p>123</p>
                    </iframe>
                </n-layout>
            </n-spin>
        </div>
    </div>
</template>
<!-- https://127.0.0.1:123?k1=123322222234222 -->
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
