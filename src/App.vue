<script setup lang="ts">
import { onBeforeMount, onMounted, shallowRef, ref } from 'vue'
import {
    darkTheme, NConfigProvider, NGlobalStyle, NIcon, NLayout, NDivider,
    NButton, NModal, NSelect, NInput, NSpace, NInputGroup,
    NTabs, NTabPane, NLoadingBarProvider, NMessageProvider, NDialogProvider,
    NCheckbox, zhCN, enUS
} from 'naive-ui'
import { ArrowForward, Add, CloseOutline, Settings, PersonCircleOutline } from '@vicons/ionicons5'
import { invoke } from '@tauri-apps/api/tauri'
import { useIndexStore } from '@/store'
import { useI18n } from 'vue-i18n'
import ProjectVue from '@/components/Project.vue'
import ApiVue from '@/components/tab/Api.vue'
import Project from '@/models/Project'
import Item from '@/models/Item'
import tauriConfig from '../src-tauri/tauri.conf.json'
import { checkUpdate, installUpdate } from '@tauri-apps/api/updater'
import { relaunch } from '@tauri-apps/api/process'
import create from '@/models'
import { OpenTabMesagae } from '@/types/store'
import { load_project, start_login_server } from '@/net/http'
import { open } from '@tauri-apps/api/shell'


const store = useIndexStore()
const { t, locale } = useI18n()
const showSide = ref<boolean>(true)  // 显示侧边栏
const token = ref<string>('')  // token
const tokenLoading = ref<boolean>(false)  // token
const tabComs = shallowRef<{ [x: string]: any }>({
    api: ApiVue
})
const projects = ref<{ [x: string]: any }[]>([])
const tab = ref<string>('')
const tabs = ref<OpenTabMesagae<any>[]>([])

const handleSetToken = async () => {
    tokenLoading.value = true
    await store.updateConfig({
        ...store.config,
        token: token.value
    })
    tokenLoading.value = false
}

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

const handleLoadProjects = async () => {
    projects.value = await Project.all()
    await Item.all()
}

const showNewProject = ref(false)
const newProjectName = ref('New Project')
const loadingNewProject = ref(false)
const handleNewProject = async () => {
    let name = newProjectName.value.trim()
    if (name == '') {
        window.$message.error(t('common.null'))
        return
    }
    let project = new Project()
    project.key = window.crypto.randomUUID()
    project.name = newProjectName.value
    project.create_at = Date.now()
    project.save()
    await handleLoadProjects()
    showNewProject.value = false
}

const handleDeleteProject = async (id: number) => {
    let project = projects.value.find((p: any) => p.id == id)
    if (project) {
        await Project.where({ id: id }).delete()
        projects.value = projects.value.filter((item) => item.id != id)
        await handleLoadProjects()
    }
}

onBeforeMount(async () => {
    await create()
    try {
        let config = localStorage.getItem('config')
        if (config) {
            store.updateConfig(JSON.parse(config))
        } else {
            store.updateConfig({
                deleteNoConfirm: false,
                showSideBar: true,
                sideBarWidth: 250,
                apiAreaHeight: 300,
                pageSize: 20,
                lang: 'zh-CN',
                token: '',
                client: window.crypto.randomUUID()
            }, false)
        }
        locale.value = store.config.lang

        token.value = store.config.token

        width.value = store.config.sideBarWidth
        oldWidth.value = width.value
    } catch { }

    let all_tabs = localStorage.getItem('tabs')
    tabs.value = all_tabs ? JSON.parse(all_tabs) : []

    let current_tab = localStorage.getItem('current_tab')
    if (current_tab && tabs.value.some(t => t.id == current_tab)) {
        tab.value = current_tab
    } else {
        if (tabs.value.length > 0) {
            tab.value = tabs.value[0].id
        }
    }
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

const handleOpenTab = async (t: OpenTabMesagae<any>) => {
    let tt = tabs.value.find(x => x.id == t.id)
    if (tt) {
        tab.value = tt.id
    } else {
        tabs.value.push(t)
        tab.value = t.id
        localStorage.setItem('tabs', JSON.stringify(tabs.value))
    }
    localStorage.setItem('current_tab', tab.value)
}
const handleTabChanged = async (val: string) => {
    tab.value = val
    localStorage.setItem('current_tab', tab.value)
}
const handleCloseTab = async (event: Event | null, id: string) => {
    console.log(id)
    if (event) {
        event.stopPropagation()
    }
    let index = 0
    tabs.value = tabs.value.filter((t, i) => {
        index = i
        return t.id !== id
    })
    if (tab.value == id) {
        if (tabs.value.length > 0) {
            if (index < tabs.value.length) {
                tab.value = tabs.value[index].id
            } else {
                tab.value = tabs.value[tabs.value.length - 1].id
            }
        }
    }
    localStorage.setItem('tabs', JSON.stringify(tabs.value))
    localStorage.setItem('current_tab', tab.value)
}
const handleLogin = async () => {
    console.log(1)
    let res = await start_login_server()
    console.log(2)
    if (res) {
        await open('http://127.0.0.1:3000')
    }
}

const valueSelectProject = ref<number | undefined>()
const optionsRemoteProject = ref<{ label: string; value: number }[]>([])
const remoteProjects = ref<Project[]>([])
const handleLoadProject = async () => {
    let res: any = await load_project({
        server: 'http://127.0.0.1:8080',
        token: store.config.token || ''
    })
    remoteProjects.value = res.data.projects
    remoteProjects.value.forEach((item: any) => {
        optionsRemoteProject.value.push({
            value: item._id,
            label: item.name
        })
    })
    if (optionsRemoteProject.value.length > 0) {
        valueSelectProject.value = optionsRemoteProject.value[0].value
    }
}
const handleDownloadProject = async () => {
    let download = remoteProjects.value.find((item: any) => item._id == valueSelectProject.value)
    if (download) {
        if (await Project.where({ key: download.key }).get()) {
            showNewProject.value = false
            return
        }
        console.log(download)
        let p = new Project()
        p._id = download._id
        p.user = download.user._id
        p.key = download.key
        p.name = download.name
        p.create_at = download.create_at
        p.save()
        await handleLoadProjects()
        showNewProject.value = false
    }
}
</script>

<template>
    <n-config-provider :theme="darkTheme" :locale="languages[locale]">
        <n-global-style />
        <n-loading-bar-provider>
            <n-message-provider>
                <n-dialog-provider>

                    <n-modal v-model:show="showUpdateInfo" preset="card" style="width: 600px;"
                        :title="t('update.title')" size="small">
                        <h1>Version: {{ updateStatus.version }}</h1>
                        <br>
                        <p>Info: {{ updateStatus.body }}</p>
                        <br>
                        <p>Publish Date: {{ updateStatus.date }}</p>
                        <br>
                        <n-button size="small" @click="handleStartUpdate" :loading="updateLoading">Install</n-button>
                        &nbsp;
                        <n-button size="small" @click="handleCancelUpdate">{{ t('common.cancel') }}</n-button>
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
                        <n-input-group>
                            <n-input v-model:value="token" type="password" show-password-on="mousedown"
                                placeholder="Token" />
                            <n-button tertiary @click="handleSetToken" :loading="tokenLoading">
                                SET
                            </n-button>
                        </n-input-group>
                        <br>
                        <br>
                        <div>Version: {{ tauriConfig.package.version }}</div>
                    </n-modal>

                    <n-modal v-model:show="showNewProject" preset="card" style="width: 600px;" :title="t('project.new')"
                        size="small">
                        <n-space vertical>
                            <n-input-group>
                                <n-input v-model:value="newProjectName" type="text" :placeholder="t('project.name')"
                                    :disabled="loadingNewProject" />
                                <n-button secondary @click.stop="handleNewProject" :loading="loadingNewProject">
                                    {{ t('common.confirm') }}
                                </n-button>
                            </n-input-group>
                        </n-space>
                        <n-divider />
                        <n-space>
                            <n-input-group>
                                <n-select v-model:value="valueSelectProject" filterable :options="optionsRemoteProject"
                                    placeholder="加载远程项目" />
                                <n-button secondary @click.stop="handleLoadProject">
                                    {{ t('common.load') }}
                                </n-button>
                                <n-button secondary @click.stop="handleDownloadProject">
                                    {{ t('common.confirm') }}
                                </n-button>
                            </n-input-group>
                        </n-space>
                    </n-modal>

                    <div id="main" class="nocopy">
                        <aside class="side nocopy" :class="store.config?.showSideBar ? '' : 'show'">
                            <div class="sidebar">
                                <n-button circle quaternary size="small" @click.stop="handleLogin">
                                    <template #icon>
                                        <n-icon>
                                            <PersonCircleOutline />
                                        </n-icon>
                                    </template>
                                </n-button>
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
                                        <component v-for="project in projects" :is="ProjectVue" :project="project"
                                            @handleOpenTab="handleOpenTab" @handleCloseTab="handleCloseTab"
                                            @handleDeleteProject="handleDeleteProject" />
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
                                    <n-tabs v-model:value="tab" @update:value="handleTabChanged" :tab-style="{
                                        padding: '6px 10px',
                                    }">
                                        <n-tab-pane v-for="i in tabs" :name="i.id" display-directive="show">
                                            <template #tab>
                                                <div
                                                    style="display: flex; justify-content: center; align-items: center;">
                                                    <span class="tab-title">{{ i.title }}</span>
                                                    <n-button circle quaternary size="small"
                                                        style="height: 20px; width: 20px; margin-left: 10px;"
                                                        @click="handleCloseTab($event, i.id)">
                                                        <template #icon>
                                                            <n-icon>
                                                                <close-outline />
                                                            </n-icon>
                                                        </template>
                                                    </n-button>
                                                </div>
                                            </template>
                                            <component :is="tabComs[i.type]" :key="i.id" :item="i.item"></component>
                                        </n-tab-pane>
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
    left: 2px;
    right: 0;
    bottom: 0;
}

#main .main .content .workspace .tab-title {
    max-width: 120px;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    display: inline-block;
}
</style>
