import { createI18n } from 'vue-i18n'
import zh_CN from './zh'
import en_US from './en'

const messages = {
    'zh-CN': zh_CN,
    'en-US': en_US
}
const i18n = createI18n({
    legacy: false,
    // globalInjection: true,

    locale: 'zh-CN',
    messages
})

export default i18n
