import deleteTodo from '../delete_todo/index.js'
import updateStatus from '../update_status_list/index.js'
import modalUpdate from '../modal/index.js'

const renderTodo = () => {
    renderTodoPresenter()
}

const renderTodoView = (obj) => {

}

const renderTodoModel = () => {

}

const renderTodoPresenter = async () => {
    const model = await renderTodoModel()
    const view = renderTodoView(model)

}

export default renderTodo