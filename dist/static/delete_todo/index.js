import renderTodo from '../render_list/index.js'

const deleteTodo = (answers) => deleteTodoPresenter(answers)

const deleteTodoView = (obj) => {
    return JSON.stringify(obj)
}

const deleteTodoModel = async (answers) => {
    return await fetch(`/api/${answers}`, {
        method: "DELETE"
    }).then(response => response.status)
}

const deleteTodoPresenter = async (answers) => {
    const model = await deleteTodoModel(answers)
    const view = deleteTodoView(model)

    if(view === "200") {
        renderTodo()
    }
    
}

export default deleteTodo