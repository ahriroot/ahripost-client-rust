import { BaseModel, IORMConfigStore } from "iorm"

class DB extends BaseModel {
    constructor(store: IORMConfigStore | null = null) {
        super({
            db: {
                db_name: 'ahripost',
                db_version: 1
            },
            store: store
        })
    }
}

export default DB