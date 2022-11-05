import { DB, init } from 'iorm'
import Item from './Item'
import Project from './Project'
import Environ from './Environ'

const create = async (): Promise<number> => {
    let count = await init({
        models: [Item, Project, Environ],
    })
    return count
}

export default create
export { Item, Project }
