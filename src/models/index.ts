import { DB, init } from 'iorm'
import Item from './item'
import Project from './project'

const create = async (): Promise<number> => {
    let count = await init({
        models: [Item, Project],
    })
    return count
}

export default create
export { Item, Project }
