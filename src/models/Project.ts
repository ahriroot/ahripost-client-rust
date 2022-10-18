import DB from "./Base"

import {
    KeyPathField,
    StringField,
    InteagerField
} from 'iorm'

class Project extends DB {
    constructor() {
        super({ store_name: 'project' })  // store_name: 存储对象名
    }
    id = KeyPathField({ auto_increment: true })  // key_path_name: 主键名，默认字段名
    key = StringField({ verbose_name: '项目唯一 id', nullable: false, unique: true, index: 'name_index', default: '' })
    name = StringField({ verbose_name: '项目名', nullable: false, unique: true, index: 'name_index', default: 'New Project' })
    create_at = InteagerField({ verbose_name: '创建时间', nullable: false, default: 0 })
}

export default Project
