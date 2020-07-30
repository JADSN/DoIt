import modal from "../modal/index.js"
import updateStatus from '../update_status_list/index.js'

const renderTodo = () => {
    console.log("renderTodo()")
    renderTodoPresenter()
}

const renderTodoView = (obj) => {
    const { todos } = obj
    console.log(todos)

    let template = ""

    todos.forEach(element => {
        template += `<li class="list-group-item ${element.done === true ? "line_through" : ""}" role="${element.id}"><input type="checkbox" class="mr-2" ${element.done === true ? "checked" : ""}/>${element.description}<button class="btn btn-danger float-right ml-3 deletar">Deletar</button><button class="btn btn-warning float-right editar">Editar</button></li>`
    })

  
    return template
}

const renderTodoModel = () => {
    return fetch("/api")
        .then(response => response.json())
}

const renderTodoPresenter = async () => {
    const model = await renderTodoModel()
    const view = renderTodoView(model)

    document.querySelector("ul#ul-entry-point").innerHTML = view

    const buttonEditar = document.querySelectorAll("button.editar")
    const buttonDeletar = document.querySelectorAll("button.deletar")
    const inputCheckbox = document.querySelectorAll("input[type=checkbox]")

    buttonEditar.forEach(element => {
        element.addEventListener("click", ({ target }) => {
            modal({
                id: target.parentElement.getAttribute("role"),
                title: "Editar",
                placeholder: target.parentElement.childNodes[1].textContent
            })
        })
    })

    buttonDeletar.forEach(element => {
        element.addEventListener("click", ({ target }) => {
            modal({
                id: target.parentElement.getAttribute("role"),
                title: "Deletar",
                placeholder: target.parentElement.childNodes[1].textContent
            })
        })
    })

    //! TODO: Passar para o modal.
    inputCheckbox.forEach(element => {
        element.addEventListener("click", ({ target }) => {
            let done = true
            element.checked ? done = true : done = false
            const answers = {
                id: target.parentElement.getAttribute("role"),
                "description": target.parentElement.childNodes[1].textContent,
                done
              }
              updateStatus(answers)
              
        })
    })

}

export default renderTodo