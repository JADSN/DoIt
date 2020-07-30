import renderTodo from '../render_list/index.js'

const todoPost = (answers) => todoPostPresenter(answers)

const todoPostView = (obj) => {
    return JSON.stringify(obj)
}

const todoPostModel = (answers) => {
    return fetch("/api", {
        method: "POST",
        headers: {
            "Accept": "application/json",
            "Content-Type": "application/json"
        },
        body: JSON.stringify(answers)
    }).then(response => response.status)
}

const todoPostPresenter = async (answers) => {
    const model = await todoPostModel(answers)
    const view = todoPostView(model)
    console.log(view)
    if(view === "200") {
        renderTodo()
    } else {
        //! TODO: criar snackbar or toast
        alert("Verifique a sua conexão!")
    }
}

export default todoPost