<script setup lang="ts">
import { h, ref, VNodeChild } from 'vue'
import { NTree, NIcon, TreeOption, NDropdown } from 'naive-ui'
import { FolderOutline, ChevronForward, CodeWorkingOutline } from '@vicons/ionicons5'
import { nanoid } from 'nanoid'

import { TreeRenderProps } from 'naive-ui/es/tree/src/interface'
import AInput from './AInput.vue'
import Item from '@/models/Item'

const props = defineProps<{
    project: any
}>()
const emits = defineEmits<{
    (e: 'handleRemoveProject', id: string): void
}>()

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
            onBlur: () => {
                option.edit = false
            }
        })
    } else {
        return h('span', {}, option.label)
    }
}

const defaultExpandedKeys = ref<string[]>([])

const showContextmenu = ref(false)
const optionsContextmenu = ref<any[]>([])
const xPos = ref(0)
const yPos = ref(0)
const nodeProps = ({ option }: { option: any }): any => {
    return {
        onClick() {
            if (option.children == undefined || option.children == null) {
                // emits('handleOpenTab', {
                //     id: nanoid(), conn: props.conn, tab_type: 'db', data: {
                //         title: `${option.label}@${props.conn.info.name}`,
                //         table: option.label
                //     }
                // })
            }
        },
        onContextmenu(e: MouseEvent): void {
            e.preventDefault()
            e.stopPropagation()
            if (option.type == 'folder' || option.type == 'project') {
                optionsContextmenu.value = [
                    {
                        label: 'Create Api',
                        key: 'create_api',
                        props: {
                            onClick: () => {
                                if (!Array.isArray(option.children)) {
                                    option.children = []
                                }
                                if (defaultExpandedKeys.value.indexOf(option.key as string) == -1) {
                                    defaultExpandedKeys.value.push(option.key)
                                    setTimeout(() => {
                                        option.children.push({
                                            key: nanoid(),
                                            label: 'New Api',
                                            type: 'api',
                                            isLeaf: true,
                                            edit: true,
                                            prefix: () => h(NIcon, null, { default: () => h(CodeWorkingOutline) }),
                                            children: []
                                        })
                                    }, 100);
                                } else {
                                    option.children.push({
                                        key: nanoid(),
                                        label: 'New Api',
                                        type: 'api',
                                        isLeaf: true,
                                        edit: true,
                                        prefix: () => h(NIcon, null, { default: () => h(CodeWorkingOutline) }),
                                        children: []
                                    })
                                }
                                showContextmenu.value = false
                            }
                        }
                    },
                    {
                        label: 'Create Folder',
                        key: 'create_folder',
                        props: {
                            onClick: () => {
                                if (!Array.isArray(option.children)) {
                                    option.children = []
                                }
                                let key = nanoid()
                                let label = 'New Folder'
                                if (defaultExpandedKeys.value.indexOf(option.key as string) == -1) {
                                    defaultExpandedKeys.value.push(option.key)
                                    setTimeout(() => {
                                        option.children.push({
                                            key: key,
                                            label: label,
                                            type: 'folder',
                                            edit: true,
                                            prefix: () => h(NIcon, null, { default: () => h(FolderOutline) }),
                                            children: []
                                        })
                                    }, 100);
                                } else {
                                    option.children.push({
                                        key: key,
                                        label: label,
                                        type: 'folder',
                                        edit: true,
                                        prefix: () => h(NIcon, null, { default: () => h(FolderOutline) }),
                                        children: []
                                    })
                                }
                                let item = new Item()
                                item.key = key
                                item.label = label
                                item.type = 'folder'
                                item.project = props.project.id
                                item.parent = option.type == 'project' ? 0 : option.id
                                showContextmenu.value = false
                            }
                        }
                    },
                    {
                        label: 'Rename',
                        key: 'rename',
                        props: {
                            onClick: () => {
                                option.edit = true
                                showContextmenu.value = false
                            }
                        }
                    },
                    {
                        label: 'Delete',
                        key: 'delete',
                        props: {
                            onClick: () => {
                                showContextmenu.value = false
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
                        label: 'Rename',
                        key: 'rename',
                        props: {
                            onClick: () => {
                                option.edit = true
                                showContextmenu.value = false
                            }
                        }
                    },
                    {
                        label: 'Delete',
                        key: 'delete',
                        props: {
                            onClick: () => {
                                showContextmenu.value = false
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

const data = ref<TreeOption[]>([{
    key: `project:${props.project.id}`,
    label: props.project.name,
    type: 'project',
    edit: false,
    children: []
}])
</script>

<template>
    <div>
        <n-dropdown trigger="manual" size="small" placement="bottom-start" :show="showContextmenu"
            :options="(optionsContextmenu as any)" :x="xPos" :y="yPos" @clickoutside="showContextmenu = false" />
        <n-tree :data="data" :selectable="false" :node-props="nodeProps" expand-on-click
            :render-switcher-icon="renderSwitcherIcon" :default-expanded-keys="defaultExpandedKeys"
            :render-label="renderLabel" />
    </div>
</template>

<style scoped>

</style>
