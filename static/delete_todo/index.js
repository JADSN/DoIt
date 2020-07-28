import renderTodo from '../render_list/index.js'

const deleteTodo = (answers) => deleteTodoPresenter(answers)

const deleteTodoView = (obj) => {
 
}

const deleteTodoModel = async (answers) => {
 
}

const deleteTodoPresenter = async (answers) => {
    const model = await deleteTodoModel(answers)
    const view = deleteTodoView(model)

  
}

export default deleteTodo