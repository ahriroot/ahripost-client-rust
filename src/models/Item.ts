import db from "./Base"

import {
    BaseModel,
    KeyPathField,
    StringField,
    InteagerField,
    ObjectField
} from 'iorm'

class Item extends BaseModel {
    constructor() {
        super({ db: db, store: { name: 'item' } })
    }
    id = KeyPathField({ auto_increment: true })  // key_path_name: 主键名，默认字段名
    key = StringField({ verbose_name: '唯一 id', nullable: false, unique: true, index: 'name_index' })
    label = StringField({ nullable: false, default: 'New Api' })
    type = StringField({ nullable: false, index: 'type_index' })
    project = InteagerField({ nullable: false, default: 0 })
    parent = InteagerField({ nullable: false, default: 0 })
    detail = ObjectField({
        nullable: true, default: {
            method: 'GET',
            protocol: '',
            host: '',
            port: '',
            href: '',
            params: [{
                checked: true,
                field: 'k1',
                value: 'v1',
                describe: 'describe',
                default: '',
                must: true,
            }],
            headers: [{
                checked: true,
                field: 'k1',
                value: 'v1',
                describe: 'describe',
                default: '',
                must: true,
            }],
            body: {
                form: [{
                    checked: true,
                    field: 'k1',
                    value: 'v1',
                    describe: 'describe',
                    default: '',
                    must: true,
                }],
                json: `{"k1": "v1"}`
            }
        }
    })
}

export default Item