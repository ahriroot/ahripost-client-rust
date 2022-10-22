<script setup lang="ts">
import { onBeforeMount, onMounted, shallowRef, ref } from 'vue'
import {
    darkTheme, NConfigProvider, NGlobalStyle, NIcon, NLayout,
    NButton, NModal, NSelect, SelectRenderLabel, NInput, NCard, NSpace,
    NTabs, NTabPane, NLoadingBarProvider, NMessageProvider, NDialogProvider,
    NCheckbox, zhCN, enUS, NH2
} from 'naive-ui'
import { ArrowForward, Add, CloseOutline, Settings } from '@vicons/ionicons5'
import { invoke } from '@tauri-apps/api/tauri'
import { useIndexStore } from '@/store'
import { useI18n } from 'vue-i18n'
import ProjectVue from '@/components/Project.vue'
import Project from '@/models/Project'
import { nanoid } from 'nanoid'
import tauriConfig from '../src-tauri/tauri.conf.json'
import { checkUpdate, installUpdate } from '@tauri-apps/api/updater'
import { relaunch } from '@tauri-apps/api/process'


const store = useIndexStore()
const { t, locale } = useI18n()
const showSide = ref<boolean>(true)  // 显示侧边栏
const tree = shallowRef<{ [x: string]: any }>({
    project: ProjectVue
})
const projects = ref<{ [x: string]: any }[]>([])
const tabs = ref<{ [x: string]: any }[]>([])

const handleShowSide = async () => {
    showSide.value = !showSide.value
    store.updateConfig({
        ...store.config,
        showSideBar: showSide.value
    })
}

// =========== SETTING MODAL START ===========
const langs = ref([
    { label: '简体中文', value: 'zh-CN' },
    { label: 'English', value: 'en-US' }
])
const languages = ref<{ [x: string]: any }>({
    'zh-CN': zhCN,
    'en-US': enUS
})
const handleUpdateLang = async (_: string) => {
    store.updateConfig({
        ...store.config,
        lang: locale.value
    })
}

const showSetting = ref(false)
const handleCheckedChange = async (val: boolean) => {
    store.updateConfig({
        ...store.config,
        deleteNoConfirm: val
    })
}

const loadingClear = ref(false)
const handleClearAllData = async () => {
    loadingClear.value = true
    localStorage.clear()
    setTimeout(() => {
        loadingClear.value = false
    }, 1000)
}

const showUpdateInfo = ref(false)
const updateStatus = ref<any>(null)
const updateLoading = ref(false)
const handleUpdate = async () => {
    try {
        const { shouldUpdate, manifest } = await checkUpdate()
        if (shouldUpdate) {
            updateStatus.value = manifest
            showUpdateInfo.value = true
        } else {
            alert('当前已是最新版本')
        }
    } catch (error) {
        console.log(error)
    }
}
const handleStartUpdate = async () => {
    updateLoading.value = true
    await installUpdate()
    await relaunch()
    updateLoading.value = false
}
const handleCancelUpdate = () => {
    showUpdateInfo.value = false
}
// =========== SETTING MODAL END ===========

const handleCloseTab = async (event: Event, id: number) => {
    event.stopPropagation()
    console.log(id)
}

const handleLoadProjects = async () => {
    projects.value = await Project.all()
}

const showNewProject = ref(false)
const newProjectName = ref('')
const loadingNewProject = ref(false)
const handleNewProject = async () => {
    let name = newProjectName.value.trim()
    if (name == '') {
        window.$message.error(t('common.null'))
        return
    }
    let project = new Project()
    project.key = nanoid()
    project.name = newProjectName.value
    project.create_at = Date.now()
    project.save()
    await handleLoadProjects()
    showNewProject.value = false
}

onBeforeMount(async () => {
    try {
        let config = localStorage.getItem('config')
        if (config) {
            store.updateConfig(JSON.parse(config))
        } else {
            store.updateConfig({
                deleteNoConfirm: false,
                showSideBar: true,
                sideBarWidth: 250,
                pageSize: 20,
                lang: 'zh-CN'
            }, false)
        }
        locale.value = store.config.lang

        width.value = store.config.sideBarWidth
        oldWidth.value = width.value
    } catch { }
    await handleLoadProjects()
})

const sidebarRef = shallowRef<HTMLElement | null>(null)
const contentRef = shallowRef<HTMLElement | null>(null)
const resizeable = ref<boolean>(false)
const width = ref(store.config.sideBarWidth)
const oldWidth = ref(250)
const cursor = ref('default')
const currentMoveX = ref(0)
onMounted(async () => {
    if (sidebarRef.value && contentRef.value) {
        sidebarRef.value.addEventListener('mousedown', (ev) => {
            if (cursor.value == 'ew-resize') {
                resizeable.value = true
                currentMoveX.value = ev.clientX
                oldWidth.value = width.value
            }
        })
        contentRef.value.addEventListener('mousedown', (ev) => {
            if (cursor.value == 'ew-resize') {
                resizeable.value = true
                currentMoveX.value = ev.clientX
                oldWidth.value = width.value
            }
        })
        document.body.addEventListener('mousemove', (ev) => {
            if (sidebarRef.value && (width.value - 4 < ev.offsetX && ev.offsetX < width.value + 4)) {
                cursor.value = 'ew-resize'
            } else {
                cursor.value = 'default'
            }
            if (resizeable.value) {
                const tmp = oldWidth.value + ev.clientX - currentMoveX.value
                if (tmp < 150) {
                    width.value = 150
                } else if (tmp > 1000) {
                    width.value = 1000
                } else {
                    width.value = tmp
                }
            }
        })
        document.body.addEventListener('mouseup', (ev) => {
            resizeable.value = false
            store.updateConfig({
                ...store.config,
                sideBarWidth: width.value
            })
        })
    }

    setTimeout(async () => {
        await invoke('close_splashscreen')
    }, 1000)
})
</script>

<template>
    <n-config-provider :theme="darkTheme" :locale="languages[locale]">
        <n-global-style />
        <n-loading-bar-provider>
            <n-message-provider>
                <n-dialog-provider>

                    <n-modal v-model:show="showUpdateInfo" preset="card" style="width: 600px;" :title="t('update.title')"
                        size="small">
                        <h1>Version: {{updateStatus.version}}</h1>
                        <br>
                        <p>Info: {{updateStatus.body}}</p>
                        <br>
                        <p>Publish Date: {{updateStatus.date}}</p>
                        <br>
                        <n-button size="small" @click="handleStartUpdate" :loading="updateLoading">Install</n-button>
                        &nbsp;
                        <n-button size="small" @click="handleCancelUpdate">{{t('common.cancel')}}</n-button>
                    </n-modal>

                    <n-modal v-model:show="showSetting" preset="card" style="width: 600px;" :title="t('setting.title')"
                        size="small">
                        <n-select size="small" v-model:value="locale" :options="langs"
                            @update:value="handleUpdateLang" />
                        <br>
                        <n-checkbox :checked="store.config?.deleteNoConfirm" @update:checked="handleCheckedChange">
                            {{ t('copywriting.noConfirmationForDeletion') }}
                        </n-checkbox>
                        <br>
                        <br>
                        <n-button :loading="loadingClear" size="small" @click="handleClearAllData">
                            {{ t('copywriting.clearCache') }}
                        </n-button>
                        <br>
                        <br>
                        <n-button :loading="updateLoading" size="small" @click="handleUpdate">
                            {{ t('copywriting.checkUpdate') }}
                        </n-button>
                        <br>
                        <br>
                        <div>Version: {{tauriConfig.package.version}}</div>
                    </n-modal>

                    <n-modal v-model:show="showNewProject" preset="card" style="width: 600px;" :title="t('project.new')"
                        size="small">
                        <n-space vertical>
                            <n-input v-model:value="newProjectName" type="text" :placeholder="t('project.name')"
                                :disabled="loadingNewProject" />
                            <n-button secondary size="small" @click.stop="handleNewProject"
                                :loading="loadingNewProject">
                                {{ t('common.confirm') }}
                            </n-button>
                        </n-space>
                    </n-modal>

                    <div id="main" class="nocopy">
                        <aside class="side nocopy" :class="store.config?.showSideBar ? '' : 'show'">
                            <div class="sidebar">
                                <n-button circle quaternary size="small" @click.stop="showSetting = true">
                                    <template #icon>
                                        <n-icon class="btn-icon-setting">
                                            <Settings />
                                        </n-icon>
                                    </template>
                                </n-button>
                            </div>
                        </aside>
                        <main class="main" :class="store.config?.showSideBar ? '' : 'show'"
                            :style="`cursor: ${resizeable ? 'ew-resize' : cursor}`">
                            <div class="connection" ref="sidebarRef" :style="`width: ${width}px`">
                                <div class="header">
                                    <n-button secondary size="small" @click.stop="showNewProject = true">
                                        <template #icon>
                                            <n-icon>
                                                <add />
                                            </n-icon>
                                        </template>
                                    </n-button>
                                </div>
                                <div class="conn">
                                    <n-layout position="absolute" style="background: #21252b; color: #fff"
                                        :native-scrollbar="false" content-style="padding: 10px;">
                                        <component v-for="project in projects" :is="ProjectVue" :project="project" />
                                    </n-layout>
                                </div>
                                <div class="btn-side">
                                    <n-icon size="20" @click="handleShowSide">
                                        <arrow-forward class="icon" :class="store.config?.showSideBar ? 'show' : ''" />
                                    </n-icon>
                                </div>
                            </div>
                            <div class="content" ref="contentRef" :style="`left: ${width}px`">
                                <section class="workspace">
                                    <n-tabs closable :tab-style="{
                                        padding: '6px 10px',
                                    }">
                                        <n-tab-pane name="chap1" v-for="tab in tabs">
                                            <template #tab>
                                                <div
                                                    style="display: flex; justify-content: center; align-items: center;">
                                                    <span>{{ tab.name }}</span>
                                                    <n-button circle quaternary size="small"
                                                        style="height: 20px; width: 20px; margin-left: 10px;"
                                                        @click="handleCloseTab($event, 1)">
                                                        <template #icon>
                                                            <n-icon>
                                                                <close-outline />
                                                            </n-icon>
                                                        </template>
                                                    </n-button>
                                                </div>
                                            </template>
                                            我这辈子最疯狂的事，发生在我在 Amazon
                                            当软件工程师的时候，故事是这样的：<br><br>
                                            那时我和女朋友住在一起，正在家里远程工作。忽然同事给我发来了紧急消息：”我们的服务出现了
                                            SEV 2 级别的故障！需要所有的人马上协助！“我们组的应用全挂掉了。<br><br>
                                            当我还在费力的寻找修复方法的时候，忽然闻到隔壁房间的的焦味，防火报警器开始鸣叫。
                                        </n-tab-pane>
                                        <!-- <n-tab-pane name="chap3" tab="第三章">
                                            <div style=" height: 100%;position: relative;">
                                                <NH2>123</NH2>
                                                <NH2>123</NH2>
                                                <NH2>123</NH2>
                                                <NH2>123</NH2>
                                                <NH2>123</NH2>
                                                <NH2>123</NH2>
                                                <NH2>123</NH2>
                                                <NH2>123</NH2>
                                                <NH2>123</NH2>
                                                <NH2>123</NH2>
                                                <NH2>123</NH2>
                                                <NH2>123</NH2>
                                                <NH2>123</NH2>
                                                <NH2>123</NH2>
                                                <NH2>123</NH2>
                                                <NH2>123</NH2>
                                                <NH2>123</NH2>
                                                <NH2>123</NH2>
                                                <NH2>123</NH2>
                                                <NH2>123</NH2>
                                                <NH2>123</NH2>
                                                <NH2>123</NH2>
                                                <NH2>123</NH2>
                                                <NH2>123</NH2>
                                                <NH2>123</NH2>
                                                <NH2>123</NH2>
                                                <NH2>123</NH2>
                                                <NH2>123</NH2>
                                                <NH2>123</NH2>
                                                <NH2>123</NH2>
                                                <NH2>123</NH2>
                                                <NH2>123</NH2>
                                            </div>
                                        </n-tab-pane> -->
                                    </n-tabs>
                                </section>
                            </div>
                        </main>
                    </div>
                </n-dialog-provider>
            </n-message-provider>
        </n-loading-bar-provider>
    </n-config-provider>
</template>

<style scoped>
.btn-icon-setting {
    transition: 0.3s;
}

.btn-icon-setting:hover {
    transform: rotate(90deg);
}

#main {
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
}

#main .side {
    width: 50px;
    background: #333842;
    overflow: hidden;
    transition: .3s;
    position: absolute;
    top: 0;
    left: 0;
    bottom: 0;
}

#main .side .sidebar {
    padding: 10px 0;
    height: 100%;
    width: 100%;
    display: flex;
    flex-direction: column;
    justify-content: flex-end;
    align-items: center;
}

#main .side.show {
    left: -50px;
}

#main .main {
    background: #282c34;
    transition: .3s;
    position: absolute;
    top: 0;
    left: 50px;
    right: 0;
    bottom: 0;
}

#main .main.show {
    left: 0;
}

#main .main .connection {
    background: #21252b;
    position: absolute;
    top: 0;
    left: 0;
    bottom: 0;
}

#main .main .connection .header {
    height: 32px;
    padding: 0 2px;
    display: flex;
    align-items: center;
    border-right: #282c34 1px solid;
}

#main .main .connection .conn {
    overflow: hidden;
    position: absolute;
    top: 32px;
    left: 0;
    right: 0;
    bottom: 30px;
    border-top: #333842 1px solid;
    border-bottom: #333842 1px solid;
}

#main .main .connection .btn-side {
    position: absolute;
    left: 8px;
    bottom: 0;
    cursor: pointer;
    color: #333842;
}

#main .main .connection .btn-side:hover {
    color: #3278c4;
}

#main .main .connection .btn-side .icon {
    transition: .3s;
}

#main .main .connection .btn-side .icon.show {
    transform: rotate(-180deg);
}

#main .main .content {
    background: #282c34;
    position: absolute;
    top: 0;
    right: 0;
    bottom: 0;
}

#main .main .content .workspace {
    background: #282c34;
    /* background: #21252b; */
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
}
</style>
