import db from "./Base"

import {
    BaseModel,
    KeyPathField,
    StringField,
    InteagerField,
    ObjectField,
} from 'iorm'

class Environ extends BaseModel {
    constructor() {
        super({ db: { name: 'dbname', version: 1 }, store: { name: 'environ' } })  // store_name: 存储对象名
    }
    id = KeyPathField({ auto_increment: true })  // key_path_name: 主键名，默认字段名
    _id = InteagerField({ nullable: false, default: 0 })
    name = StringField({ nullable: false, default: '' })
    key = StringField({ verbose_name: '唯一 id', nullable: false, unique: true, index: 'name_index', default: '' })
    environs = ObjectField({ nullable: false, default: [] })
}

export default Environ
