import renderTodo from '../render_list/index.js'
// import lastTodo from '../last_todo/index.js'

const todoPost = (answers) => todoPostPresenter(answers)

const todoPostView = (obj) => {
    return JSON.stringify(obj)
}

const todoPostModel = (answers) => {
    return fetch("/todos", {
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

        //! Criar função para renderizar somente o último todo
        renderTodo()
        // lastTodo()
        document.querySelector("input#input-add-todo").value = ""
    } else {
        //! TODO: criar snackbar or toast
        alert("Verifique a sua conexão!")
    }
}

export default todoPost