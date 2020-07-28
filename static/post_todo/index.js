import renderTodo from '../render_list/index.js'
// import lastTodo from '../last_todo/index.js'

const todoPost = (answers) => todoPostPresenter(answers)

const todoPostView = (obj) => {

}

const todoPostModel = (answers) => {
   
}

const todoPostPresenter = async (answers) => {
    const model = await todoPostModel(answers)
    const view = todoPostView(model)
    
}

export default todoPost