const lastTodo = () => lastTodoPresenter()

const lastTodoView = (obj) => {
    
}

const lastTodoModel = () => {

}

const lastTodoPresenter = async () => {
    const model = await lastTodoModel()
    const view = lastTodoView(model)


}

export default lastTodo