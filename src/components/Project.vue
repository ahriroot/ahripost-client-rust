<script setup lang="ts">
import { h, ref, VNodeChild, onBeforeMount } from 'vue'
import { NTree, NIcon, TreeOption, NDropdown, NSpin, useDialog } from 'naive-ui'
import { FolderOutline, ChevronForward, CodeWorkingOutline } from '@vicons/ionicons5'

import { TreeDropInfo, TreeRenderProps } from 'naive-ui/es/tree/src/interface'
import AInputFocus from './AInputFocus.vue'
import Item from '@/models/Item'
import { useMessage } from 'naive-ui'
import { useI18n } from 'vue-i18n'
import { useIndexStore } from '@/store'
import { OpenTabMesagae } from '@/types/store'
import Project from '@/models/Project'
import { sync_check, sync_data } from '@/net/http'

window.$message = useMessage()
const store = useIndexStore()
const dialog = useDialog()

const props = defineProps<{
    project: any
}>()
const emits = defineEmits<{
    (e: 'handleOpenTab', val: OpenTabMesagae<any>): void
    (e: 'handleCloseTab', ev: null, id: string): void
    (e: 'handleDeleteProject', id: number): void
}>()


const data = ref<TreeOption[]>([])

const { t } = useI18n()

const loading = ref(false)

const data2tree = async (data: any[], parent: string) => {
    let tmp: any[] = []
    for (let index = 0; index < data.length; index++) {
        const element = data[index];
        if (element.parent === parent) {
            let tmp_item: any = {
                key: element.key,
                label: element.label,
                value: element.key,
                type: element.type,
                edit: false,
            }
            if (element.type == 'folder') {
                tmp_item.prefix = () => h(NIcon, null, { default: () => h(FolderOutline) })
                let res = await data2tree(data, element.key)
                tmp_item.children = res
            } else {
                tmp_item.prefix = () => h(NIcon, null, { default: () => h(CodeWorkingOutline) })
            }
            tmp.push(tmp_item)
        }
    }
    // 根据 type 排序
    tmp.sort((a, b) => (a.type > b.type) ? -1 : ((b.type > a.type) ? 1 : 0))
    return tmp
}

const handleLoadProject = async () => {
    loading.value = true
    let items = (await Item.where({ 'project': props.project.key }).order({ id: 1 }).all()) as any[]
    let res = await data2tree(items, '')

    data.value = [{
        key: `project:${props.project.key}`,
        label: props.project.name,
        value: '',
        type: 'project',
        edit: false,
        children: res,
    }]
    loading.value = false
}

onBeforeMount(async () => {
    await handleLoadProject()
    let eks = localStorage.getItem(`expandedKeys:${props.project.key}`)
    if (eks) {
        expandedKeys.value = JSON.parse(eks)
    }
})

const renderSwitcherIcon = () => {
    return h(NIcon, null, { default: () => h(ChevronForward) })
}

const renderLabel = ({ option, checked, selected }: TreeRenderProps): VNodeChild => {
    if (option.edit) {
        return h(AInputFocus, {
            value: option.label as string,
            onUpdateValue: (val: any) => {
                option.label = val
            },
            onBlur: async () => {
                let tmp = (await Item.where({ key: option.value }).obj() as Item)
                if (tmp && tmp.key) {
                    tmp.label = option.label
                    tmp.last_update = new Date().getTime()
                    await tmp.save()
                }
                option.edit = false
            }
        })
    } else {
        return h('span', {}, option.label)
    }
}

const expandedKeys = ref<string[]>([])

const showContextmenu = ref(false)
const optionsContextmenu = ref<any[]>([])
const xPos = ref(0)
const yPos = ref(0)
const nodeProps = ({ option }: { option: any }): any => {
    return {
        onClick() {

        },
        onDblclick() {
            if (option.type === 'api') {
                emits('handleOpenTab', {
                    id: option.key,
                    title: option.label,
                    type: 'api',
                    item: option.value
                })
                showContextmenu.value = false
            }
        },
        onContextmenu(e: MouseEvent): void {
            e.preventDefault()
            e.stopPropagation()
            if (option.type == 'folder' || option.type == 'project') {
                optionsContextmenu.value = [
                    {
                        label: t('itemTree.createApi'),
                        key: 'create_api',
                        props: {
                            onClick: async () => {
                                if (!Array.isArray(option.children)) {
                                    option.children = []
                                }

                                let key = window.crypto.randomUUID()
                                let label = 'New Api'

                                let item = new Item()
                                item.key = key
                                item.label = label
                                item.type = 'api'
                                item.project = props.project.key
                                item.parent = option.type == 'project' ? '' : option.value
                                item.last_update = new Date().getTime()
                                item.tag = false
                                item.client = store.config.client
                                await item.save()

                                if (!expandedKeys.value.includes(option.key as string)) {
                                    expandedKeys.value.push(option.key)
                                    setTimeout(() => {
                                        option.children.push({
                                            key: key,
                                            label: label,
                                            type: 'api',
                                            edit: true,
                                            value: key,
                                            prefix: () => h(NIcon, null, { default: () => h(CodeWorkingOutline) })
                                        })
                                    }, 100)
                                } else {
                                    option.children.push({
                                        key: key,
                                        label: label,
                                        type: 'api',
                                        edit: true,
                                        value: key,
                                        prefix: () => h(NIcon, null, { default: () => h(CodeWorkingOutline) })
                                    })
                                }
                                showContextmenu.value = false
                            }
                        }
                    },
                    {
                        label: t('itemTree.createFolder'),
                        key: 'create_folder',
                        props: {
                            onClick: async () => {
                                if (!Array.isArray(option.children)) {
                                    option.children = []
                                }

                                let key = window.crypto.randomUUID()
                                let label = 'New Folder'

                                let item = new Item()
                                item.key = key
                                item.label = label
                                item.type = 'folder'
                                item.project = props.project.key
                                item.parent = option.type == 'project' ? '' : option.value
                                item.last_update = new Date().getTime()
                                item.tag = false
                                item.client = store.config.client
                                item.request = ''
                                item.response = ''
                                await item.save()

                                if (expandedKeys.value.indexOf(option.key as string) == -1) {
                                    expandedKeys.value.push(option.key)
                                    setTimeout(() => {
                                        option.children.push({
                                            key: key,
                                            label: label,
                                            type: 'folder',
                                            edit: true,
                                            value: key,
                                            prefix: () => h(NIcon, null, { default: () => h(FolderOutline) }),
                                            children: []
                                        })
                                    }, 100)
                                } else {
                                    option.children.push({
                                        key: key,
                                        label: label,
                                        type: 'folder',
                                        edit: true,
                                        value: key,
                                        prefix: () => h(NIcon, null, { default: () => h(FolderOutline) }),
                                        children: []
                                    })
                                }
                                showContextmenu.value = false
                            }
                        }
                    },
                    {
                        label: t('itemTree.rename'),
                        key: 'rename',
                        props: {
                            onClick: () => {
                                option.edit = true
                                showContextmenu.value = false
                            }
                        }
                    },
                    {
                        label: t('common.delete'),
                        key: 'delete',
                        props: {
                            onClick: async () => {
                                if (store.config?.deleteNoConfirm) {
                                    if (option.type == 'project') {
                                        emits('handleDeleteProject', props.project.key)
                                    } else {
                                        let obj: any = await Item.where({ key: option.value }).obj()
                                        if (obj.last_sync) {
                                            // TODO: 删除远程
                                        }
                                        await Item.where({ key: option.value }).delete()
                                    }
                                    if (expandedKeys.value.includes(option.key as string)) {
                                        expandedKeys.value.splice(expandedKeys.value.indexOf(option.key as string), 1)
                                    }
                                    showContextmenu.value = false
                                    await handleLoadProject()
                                } else {
                                    dialog.warning({
                                        title: t('common.delete'),
                                        content: `${t('copywriting.deleteFolder')} ${option.label} ?`,
                                        positiveText: t('common.delete'),
                                        onPositiveClick: async () => {
                                            if (option.type == 'project') {
                                                emits('handleDeleteProject', props.project.key)
                                            } else {
                                                let obj: any = await Item.where({ key: option.value }).obj()
                                                if (obj.last_sync) {
                                                    // TODO: 删除远程
                                                }
                                                await Item.where({ key: option.value }).delete()
                                            }
                                            if (expandedKeys.value.includes(option.key as string)) {
                                                expandedKeys.value.splice(expandedKeys.value.indexOf(option.key as string), 1)
                                            }
                                            showContextmenu.value = false
                                            await handleLoadProject()
                                        }
                                    })
                                }
                            }
                        }
                    }
                ]
                if (option.type == 'project') {
                    optionsContextmenu.value.unshift({
                        label: t('common.sync'),
                        key: 'syncs',
                        props: {
                            onClick: async () => {
                                if (!store.config?.token) {
                                    window.$message.error("No Token")
                                    return
                                }
                                loading.value = true
                                showContextmenu.value = false
                                try {
                                    let host = store.config.host
                                    if (!host) {
                                        host = 'https://ahripost.ahriknow.com'
                                        store.updateConfig({
                                            ...store.config,
                                            host
                                        })
                                    }
                                    let formApis: { key: any; project: any; last_update: any; last_sync: any }[] = []
                                    let items: any[] = (await Item.where({ project: props.project.key }).all()) as any[]
                                    items.forEach(async (item: any) => {
                                        formApis.push({
                                            key: item.key,
                                            project: item.project,
                                            last_update: item.last_update,
                                            last_sync: item.last_sync,
                                        })
                                    })
                                    let tmpProject: any = await Project.where({ key: props.project.key }).get()
                                    let formProject = {
                                        key: tmpProject.key,
                                        name: tmpProject.name,
                                    }
                                    let sc: any = await sync_check({
                                        data: { apis: formApis, project: formProject },
                                        server: host,
                                        token: store.config.token || ''
                                    })
                                    if (sc.error && typeof sc.error == 'string') {
                                        window.$message.error(sc.error)
                                    }
                                    let items_upload: any[] = []
                                    for (let i = 0; i < sc.data.items_upload.length; i++) {
                                        let api: any = await Item.where({ key: sc.data.items_upload[i] }).get()
                                        if (api && api.key) {
                                            api.request = JSON.stringify(api.request)
                                            api.response = JSON.stringify(api.response)
                                            items_upload.push(api)
                                        }
                                    }
                                    let sd: any = await sync_data({
                                        data: {
                                            items_upload: items_upload,
                                            items_download: sc.data.items_download,
                                            project: formProject
                                        },
                                        server: host,
                                        token: store.config.token || ''
                                    })
                                    if (sd.error && typeof sd.error == 'string') {
                                        window.$message.error(sd.error)
                                    }
                                    for (let i = 0; i < sd.data.length; i++) {
                                        let api = (await Item.where({ key: sd.data[i].key }).obj() as Item)
                                        if (api && api.key) {
                                            api.request = sd.data[i].request ? JSON.parse(sd.data[i].request) : null
                                            api.response = sd.data[i].response ? JSON.parse(sd.data[i].response) : null
                                            api.label = sd.data[i].label
                                            api.type = sd.data[i].type
                                            api.from = sd.data[i].from
                                            api.project = sd.data[i].project_id
                                            api.parent = sd.data[i].parent
                                            api.last_sync = sd.data[i].last_sync
                                            api.last_update = sd.data[i].last_update
                                            api.tag = false
                                            api.client = store.config.client
                                            await api.save()
                                        } else {
                                            let api = new Item()
                                            api.key = sd.data[i].key
                                            api.request = sd.data[i].request ? JSON.parse(sd.data[i].request) : null
                                            api.response = sd.data[i].response ? JSON.parse(sd.data[i].response) : null
                                            api.label = sd.data[i].label
                                            api.type = sd.data[i].type
                                            api.from = sd.data[i].from
                                            api.project = sd.data[i].project_id
                                            api.parent = sd.data[i].parent
                                            api.last_sync = sd.data[i].last_sync
                                            api.last_update = sd.data[i].last_update
                                            api.tag = false
                                            api.client = store.config.client
                                            await api.save()
                                        }
                                    }
                                    await handleLoadProject()
                                } catch { }
                                loading.value = false
                            }
                        }
                    })
                }
                xPos.value = e.clientX
                yPos.value = e.clientY
                showContextmenu.value = true
            } else if (option.type == 'api') {
                optionsContextmenu.value = [
                    {
                        label: t('common.open'),
                        key: 'open',
                        props: {
                            onClick: () => {
                                if (option.type === 'api') {
                                    emits('handleOpenTab', {
                                        id: option.key,
                                        title: option.label,
                                        type: 'api',
                                        item: option.value
                                    })
                                    showContextmenu.value = false
                                }
                                showContextmenu.value = false
                            }
                        }
                    },
                    {
                        label: t('common.delete'),
                        key: 'delete',
                        props: {
                            onClick: async () => {
                                if (store.config?.deleteNoConfirm) {
                                    let obj: any = await Item.where({ key: option.value }).obj()
                                    if (obj.last_sync) {
                                        // TODO: 删除远程
                                    }
                                    await Item.where({ key: option.value }).delete()
                                    emits('handleCloseTab', null, option.key)
                                    if (expandedKeys.value.includes(option.key as string)) {
                                        expandedKeys.value.splice(expandedKeys.value.indexOf(option.key as string), 1)
                                    }
                                    showContextmenu.value = false
                                    await handleLoadProject()
                                } else {
                                    dialog.warning({
                                        title: t('common.delete'),
                                        content: `${t('copywriting.deleteApi')} ${option.label} ?`,
                                        positiveText: t('common.delete'),
                                        onPositiveClick: async () => {
                                            let obj: any = await Item.where({ key: option.value }).obj()
                                            if (obj.last_sync) {
                                                // TODO: 删除远程
                                            }
                                            await Item.where({ key: option.value }).delete()
                                            emits('handleCloseTab', null, option.key)
                                            if (expandedKeys.value.includes(option.key as string)) {
                                                expandedKeys.value.splice(expandedKeys.value.indexOf(option.key as string), 1)
                                            }
                                            showContextmenu.value = false
                                            await handleLoadProject()
                                        }
                                    })
                                }
                            }
                        }
                    }
                ]
                xPos.value = e.clientX
                yPos.value = e.clientY
                showContextmenu.value = true
            }
        }
    }
}

const handleExpand = (key: string[]) => {
    expandedKeys.value = key
    localStorage.setItem(`expandedKeys:${props.project.key}`, JSON.stringify(key))
}

const findSiblingsAndIndex = (
    node: TreeOption,
    nodes?: TreeOption[]
): [TreeOption[], number] | [null, null] => {
    if (!nodes) return [null, null]
    for (let i = 0; i < nodes.length; ++i) {
        const siblingNode = nodes[i]
        if (siblingNode.key === node.key) return [nodes, i]
        const [siblings, index] = findSiblingsAndIndex(node, siblingNode.children)
        if (siblings && index !== null) return [siblings, index]
    }
    return [null, null]
}

const handleDrop = async ({ node, dragNode, dropPosition }: TreeDropInfo) => {
    const [dragNodeSiblings, dragNodeIndex] = findSiblingsAndIndex(
        dragNode,
        data.value
    )
    if (dragNodeSiblings === null || dragNodeIndex === null) return
    dragNodeSiblings.splice(dragNodeIndex, 1)
    if (dropPosition === 'inside') {
        if (node.children) {
            node.children.unshift(dragNode)
        } else {
            node.children = [dragNode]
        }
    } else if (dropPosition === 'before') {
        const [nodeSiblings, nodeIndex] = findSiblingsAndIndex(
            node,
            data.value
        )
        if (nodeSiblings === null || nodeIndex === null) return
        nodeSiblings.splice(nodeIndex, 0, dragNode)
    } else if (dropPosition === 'after') {
        const [nodeSiblings, nodeIndex] = findSiblingsAndIndex(
            node,
            data.value
        )
        if (nodeSiblings === null || nodeIndex === null) return
        nodeSiblings.splice(nodeIndex + 1, 0, dragNode)
    }
    data.value = Array.from(data.value)
}
</script>

<template>
    <n-spin :show="loading">
        <n-dropdown trigger="manual" size="small" placement="bottom-start" :show="showContextmenu"
            :options="(optionsContextmenu as any)" :x="xPos" :y="yPos" @clickoutside="showContextmenu = false" />
        <n-tree :data="data" :selectable="false" @update:expanded-keys="handleExpand" :node-props="nodeProps"
            expand-on-click :render-switcher-icon="renderSwitcherIcon" :default-expanded-keys="expandedKeys"
            :render-label="renderLabel" draggable @drop="handleDrop" />
    </n-spin>
</template>

<style scoped>

</style>
