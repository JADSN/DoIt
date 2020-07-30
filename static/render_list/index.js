import deleteTodo from '../delete_todo/index.js'
import updateStatus from '../update_status_list/index.js'
import modalUpdate from '../modal/index.js'

const renderTodo = () => {
    renderTodoPresenter()
}

const renderTodoView = (obj) => {
    const { todos } = obj
    console.log(obj.todos)
    const elementsOutSide = ["ul", "p"]
    const elementsInSide = ["li", "input", "span", "button"]

     // CREATE THE ELEMENTS
     const html = arr => arr.map(element => document.createElement(element)) 

     // SETTING THE ATTRIBUTES INTO THE ELEMENTS
    const att = (element, name, value) => element.setAttribute(name, value)

    const [ul, p] = html(elementsOutSide)

    const paragraphText = document.createTextNode("Nothing to do! Add a task?")
    p.appendChild(paragraphText)

    todos.forEach(list => {
        const [li, input, span, button] = html(elementsInSide)

        const spanText = document.createTextNode(list.description)
        const buttonText = document.createTextNode("Delete")

        att(input, "type", "checkbox")
        att(span, "class", "editable")
        att(button, "class", "delete")
        att(button, "role", list.id)
        att(input, "role", list.id)

        button.appendChild(buttonText)
        span.appendChild(spanText)
        li.appendChild(input)
        li.appendChild(span)
        li.appendChild(button)
        ul.appendChild(li)

        if (list.status === 1) {
            input.setAttribute("checked", true)
            input.nextElementSibling.classList.add("line_through")
        }
    })


    const element = obj.length === 0 ? p : ul

    return [element]
}

const renderTodoModel = () => {
    return fetch("/api")
        .then(response => response.json())
}

const renderTodoPresenter = async () => {
    const model = await renderTodoModel()
    const view = renderTodoView(model)

    //! VERIFICAR UMA MELHOR SOLUÇÃO
    // APAGA TO OS FILHOS DA DIV ANTES DE RENDERIZAR NOVAMENTE.
    document.querySelector("div#entry-point").innerHTML = ""

    view.forEach(element => {
        document.querySelector("div#entry-point").appendChild(element)
    })

    // DELETE TODO
    const button = document.querySelectorAll("button.delete")
    button.forEach(element => {
        element.addEventListener("click", ({ target }) => {
            const id = target.getAttribute("role")
            deleteTodo(id)
        })
    })

    // UPDATE STATUS
    const input = document.querySelectorAll("input[type=checkbox]")
    input.forEach(element => {
        element.addEventListener("click", ({ target }) => {
            const id = target.getAttribute("role")
            const status = element.checked === false ? false : true
            const answers = { id, status }
            updateStatus(answers)
        })
    })

    // UPDATE DESCRIPTION
    const span = document.querySelectorAll("span.editable")
    span.forEach(element => {
        element.addEventListener("click", ({ target }) => {
            const text = target.textContent
            modalUpdate(text)
        })
    })
}

export default renderTodo