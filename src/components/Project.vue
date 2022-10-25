<script setup lang="ts">
import { h, ref, VNodeChild, onBeforeMount } from 'vue'
import { NTree, NIcon, TreeOption, NDropdown, useDialog } from 'naive-ui'
import { FolderOutline, ChevronForward, CodeWorkingOutline } from '@vicons/ionicons5'
import { nanoid } from 'nanoid'

import { TreeDropInfo, TreeRenderProps } from 'naive-ui/es/tree/src/interface'
import AInput from './AInput.vue'
import Item from '@/models/Item'
import { useMessage } from 'naive-ui'
import { useI18n } from 'vue-i18n'
import { useIndexStore } from '@/store'
import { OpenTabMesagae } from '@/types/store'
import { emit } from 'process'

window.$message = useMessage()
const store = useIndexStore()
const dialog = useDialog()

const props = defineProps<{
    project: any
}>()
const emits = defineEmits<{
    (e: 'handleOpenTab', val: OpenTabMesagae<any>): void
    (e: 'handleCloseTab', ev: null, id: string): void
}>()


const data = ref<TreeOption[]>([])

const { t } = useI18n()

const data2tree = async (data: any[], parent: number) => {
    let tmp: any[] = []
    for (let index = 0; index < data.length; index++) {
        const element = data[index];
        if (element.parent === parent) {
            let tmp_item: any = {
                key: element.key,
                label: element.label,
                value: element.id,
                type: element.type,
                edit: false,
            }
            if (element.type == 'folder') {
                tmp_item.prefix = () => h(NIcon, null, { default: () => h(FolderOutline) })
                let res = await data2tree(data, element.id)
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
    let items = (await Item.where({ 'project': props.project.id }).order({ id: 1 }).all()) as any[]
    let res = await data2tree(items, 0)

    data.value = [{
        key: `project:${props.project.id}`,
        label: props.project.name,
        value: 0,
        type: 'project',
        edit: false,
        children: res,
    }]
}

onBeforeMount(async () => {
    await handleLoadProject()
    let eks = localStorage.getItem(`expandedKeys:${props.project.id}`)
    if (eks) {
        expandedKeys.value = JSON.parse(eks)
    }
})

const renderSwitcherIcon = () => {
    return h(NIcon, null, { default: () => h(ChevronForward) })
}

const renderLabel = ({ option, checked, selected }: TreeRenderProps): VNodeChild => {
    if (option.edit) {
        return h(AInput, {
            value: option.label as string,
            onUpdateValue: (val: any) => {
                option.label = val
            },
            onBlur: async () => {
                let tmp = (await Item.where({ id: option.value }).obj() as Item)
                if (tmp && tmp.id) {
                    tmp.label = option.label
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

                                let key = nanoid()
                                let label = 'New Api'

                                let item = new Item()
                                item.key = key
                                item.label = label
                                item.type = 'api'
                                item.project = props.project.id
                                item.parent = option.type == 'project' ? 0 : option.value
                                let id = await item.save()

                                if (!expandedKeys.value.includes(option.key as string)) {
                                    expandedKeys.value.push(option.key)
                                    setTimeout(() => {
                                        option.children.push({
                                            key: key,
                                            label: label,
                                            type: 'api',
                                            edit: true,
                                            value: id,
                                            prefix: () => h(NIcon, null, { default: () => h(CodeWorkingOutline) })
                                        })
                                    }, 100)
                                } else {
                                    option.children.push({
                                        key: key,
                                        label: label,
                                        type: 'api',
                                        edit: true,
                                        value: id,
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

                                let key = nanoid()
                                let label = 'New Folder'

                                let item = new Item()
                                item.key = key
                                item.label = label
                                item.type = 'folder'
                                item.project = props.project.id
                                item.parent = option.type == 'project' ? 0 : option.value
                                let id = await item.save()

                                if (expandedKeys.value.indexOf(option.key as string) == -1) {
                                    expandedKeys.value.push(option.key)
                                    setTimeout(() => {
                                        option.children.push({
                                            key: key,
                                            label: label,
                                            type: 'folder',
                                            edit: true,
                                            value: id,
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
                                        value: id,
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
                                    let obj = (await Item.where({ id: option.value }).delete() as Item)
                                    if (expandedKeys.value.includes(option.key as string)) {
                                        expandedKeys.value.splice(expandedKeys.value.indexOf(option.key as string), 1)
                                    }
                                    await handleLoadProject()
                                    showContextmenu.value = false
                                } else {
                                    dialog.warning({
                                        title: t('common.delete'),
                                        content: `${t('copywriting.deleteFolder')} ${option.label} ?`,
                                        positiveText: t('common.delete'),
                                        onPositiveClick: async () => {
                                            let obj = (await Item.where({ id: option.value }).delete() as Item)
                                            if (expandedKeys.value.includes(option.key as string)) {
                                                expandedKeys.value.splice(expandedKeys.value.indexOf(option.key as string), 1)
                                            }
                                            await handleLoadProject()
                                            showContextmenu.value = false
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
                                    await Item.where({ id: option.value }).delete()
                                    emits('handleCloseTab', null, option.key)
                                    if (expandedKeys.value.includes(option.key as string)) {
                                        expandedKeys.value.splice(expandedKeys.value.indexOf(option.key as string), 1)
                                    }
                                    await handleLoadProject()
                                    showContextmenu.value = false
                                } else {
                                    dialog.warning({
                                        title: t('common.delete'),
                                        content: `${t('copywriting.deleteApi')} ${option.label} ?`,
                                        positiveText: t('common.delete'),
                                        onPositiveClick: async () => {
                                            await Item.where({ id: option.value }).delete()
                                            emits('handleCloseTab', null, option.key)
                                            if (expandedKeys.value.includes(option.key as string)) {
                                                expandedKeys.value.splice(expandedKeys.value.indexOf(option.key as string), 1)
                                            }
                                            await handleLoadProject()
                                            showContextmenu.value = false
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
    localStorage.setItem(`expandedKeys:${props.project.id}`, JSON.stringify(key))
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
    <div>
        <n-dropdown trigger="manual" size="small" placement="bottom-start" :show="showContextmenu"
            :options="(optionsContextmenu as any)" :x="xPos" :y="yPos" @clickoutside="showContextmenu = false" />
        <n-tree :data="data" :selectable="false" @update:expanded-keys="handleExpand" :node-props="nodeProps"
            expand-on-click :render-switcher-icon="renderSwitcherIcon" :default-expanded-keys="expandedKeys"
            :render-label="renderLabel" draggable @drop="handleDrop" />
    </div>
</template>

<style scoped>

</style>
