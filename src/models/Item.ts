import db from "./Base"

import {
    BaseModel,
    KeyPathField,
    StringField,
    InteagerField,
    ObjectField,
    BooleanField
} from 'iorm'

class Item extends BaseModel {
    constructor() {
        super({ db: db, store: { name: 'item' } })
    }
    id = KeyPathField({ auto_increment: true })  // key_path_name: 主键名，默认字段名
    _id = InteagerField({ nullable: false, default: 0 })
    key = StringField({ verbose_name: '唯一 id', nullable: false, unique: true, index: 'name_index' })
    label = StringField({ nullable: false, default: 'New Api' })
    type = StringField({ nullable: false, index: 'type_index' })
    from = StringField({ nullable: false, default: 'client' })
    project = StringField({ nullable: false })
    parent = StringField({ nullable: true, default: '' })
    user = InteagerField({ nullable: false, default: 0 })
    tag = BooleanField({ nullable: false, default: false })
    client = StringField({ nullable: false, default: '' })
    last_sync = InteagerField({ nullable: false, default: 0 })
    last_update = InteagerField({ nullable: false, default: 0 })
    template = StringField({ nullable: false, default: '{{ title }}\n\n{{ describe }}\n\n{{ detail }}\n\n{{ example }}\n' })
    request = ObjectField({
        nullable: true, default: {
            describe: '',
            method: 'GET',
            protocol: '',
            host: '',
            port: '',
            path: '',
            href: '',
            search: '',
            tab: 'param',
            query_keys: [],
            query: [],
            params_keys: [],
            params: [],
            headers_keys: ['Content-Type'],
            headers: [{
                key: 'Content-Type',
                checked: true,
                field: 'Content-Type',
                value: 'application/json',
                describe: 'Content-Type',
                default: '',
                must: true
            }],
            body: {
                type: 'none',
                form_keys: [],
                form: [],
                json: `{}`
            }
        }
    })
    response = ObjectField({
        nullable: true, default: {
            status: 0,
            statusText: 'None',
            headers: [],
            tab: 'body',
            datetime: 0,
            body: {
                type: 'raw',
                html: '',
                json: `{}`,
                text: ''
            }
        }
    })
}

export default Item
