<script setup lang="ts">
import { h, ref, shallowRef, onMounted, onBeforeMount, render } from 'vue'
import {
    NLayout, NH2, NInputGroup, NButton, NInput, useDialog,
    NSelect, NTabs, NTabPane, NDataTable, SelectOption, DataTableColumns
} from 'naive-ui'
import { FolderOutline, ChevronForward, CodeWorkingOutline } from '@vicons/ionicons5'
import { nanoid } from 'nanoid'
import AInput from '@/components/AInput.vue'
import ACheckbox from '@/components/ACheckbox.vue'
import Item from '@/models/Item'
import { useMessage } from 'naive-ui'
import { useI18n } from 'vue-i18n'
import { useIndexStore } from '@/store'

window.$message = useMessage()
const store = useIndexStore()
const dialog = useDialog()
const data = ref<any>({})

const props = defineProps<{
    item: any
}>()
const emits = defineEmits<{
    // (e: 'handle', id: any): void
}>()

onBeforeMount(async () => {
    let res = await Item.where({ id: props.item }).get()
    data.value = res
    height.value = store.config.apiAreaHeight
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
        title: '',
        key: 'checked',
        align: 'center',
        render(row: any, index: number) {
            return h('div',
                {
                    class: 'input',
                },
                [
                    h(ACheckbox, {
                        value: row.checked,
                        onUpdateValue: (val: any) => {
                            row.checked = val
                        }
                    })
                ]
            )
        }
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
        key: 'desc',
        render(row: any, index: number) {
            return h('div',
                {
                    class: 'input'
                },
                [
                    h(AInput, {
                        value: row.desc,
                        onUpdateValue: (val: any) => {
                            row.desc = val
                        }
                    })
                ]
            )
        }
    }
])

const topRef = shallowRef<HTMLElement | null>(null)
const bottomRef = shallowRef<HTMLElement | null>(null)
const resizeable = ref<boolean>(false)
const height = ref(store.config.apiAreaHeight)
const oldHeight = ref(250)
const cursor = ref('default')
const currentMoveY = ref(0)
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
        document.body.addEventListener('mouseup', (ev) => {
            resizeable.value = false
            store.updateConfig({
                ...store.config,
                apiAreaHeight: height.value
            })
        })
    }
})
</script>

<template>
    <div class="tab-api">
        <div ref="topRef" class="top" :style="`height: ${height - 2}px; cursor: ${resizeable ? 'ns-resize' : cursor}`">
            <div class="title">
                <span>{{ data.label }}</span>
                <span>env</span>
            </div>
            <div class="location">
                <n-input-group>
                    <n-select v-model:value="method" :options="options" style="width: 150px" />
                    <n-input placeholder="Location" />
                    <n-button secondary>SEND</n-button>
                </n-input-group>
            </div>
            <n-tabs style="top: 72px; bottom: 6px">
                <n-tab-pane name="chap1" tab="第一章">
                    <template #tab>
                        <div style="padding: 0 10px">
                            <span>Header</span>
                        </div>
                    </template>
                    <n-layout position="absolute" style="top: 0; bottom: 0; background: #21252b"
                        :native-scrollbar="false">
                        <n-data-table v-if="data.detail?.headers" size="small" :columns="columns"
                            :data="data.detail.headers" :single-line="false" :bordered="false" />
                    </n-layout>
                </n-tab-pane>
                <n-tab-pane name="chap2" tab="第二章">
                    <template #tab>
                        <div style="padding: 0 10px">
                            <span>Param</span>
                        </div>
                    </template>
                    <div style="padding: 10px">
                        我这辈子最疯狂的事，发生在我在 Amazon
                        当软件工程师的时候，故事是这样的：<br><br>
                        那时我和女朋友住在一起，正在家里远程工作。忽然同事给我发来了紧急消息：”我们的服务出现了
                        SEV 2 级别的故障！需要所有的人马上协助！“我们组的应用全挂掉了。<br><br>
                        当我还在费力的寻找修复方法的时候，忽然闻到隔壁房间的的焦味，防火报警器开始鸣叫。
                    </div>
                </n-tab-pane>
                <n-tab-pane name="chap3" tab="第三章">
                    <template #tab>
                        <div style="padding: 0 10px">
                            <span>Body</span>
                        </div>
                    </template>
                    <div style="padding: 10px">
                        我这辈子最疯狂的事，发生在我在 Amazon
                        当软件工程师的时候，故事是这样的：<br><br>
                        那时我和女朋友住在一起，正在家里远程工作。忽然同事给我发来了紧急消息：”我们的服务出现了
                        SEV 2 级别的故障！需要所有的人马上协助！“我们组的应用全挂掉了。<br><br>
                        当我还在费力的寻找修复方法的时候，忽然闻到隔壁房间的的焦味，防火报警器开始鸣叫。
                    </div>
                </n-tab-pane>
            </n-tabs>
        </div>
        <div ref="bottomRef" class="bottom" :style="`top: ${height}px`">
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
            </n-layout>
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
