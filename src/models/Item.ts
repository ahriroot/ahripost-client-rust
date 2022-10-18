import DB from "./Base"

import {
    KeyPathField,
    StringField,
    InteagerField,
    ObjectField
} from 'iorm'

class Item extends DB {
    constructor() {
        super({ store_name: 'item' })  // store_name: 存储对象名
    }
    _id = KeyPathField({ auto_increment: true })  // key_path_name: 主键名，默认字段名
    key = StringField({ verbose_name: '唯一 id', nullable: false, unique: true, index: 'name_index' })
    label = StringField({ nullable: false, default: 'New Api' })
    type = StringField({ nullable: false })
    project = InteagerField({ nullable: false, default: 0 })
    parent = InteagerField({ nullable: false, default: 0 })
    detail = ObjectField({ nullable: false, default: {} })
}

export default Item
