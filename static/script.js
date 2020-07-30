import renderTodo from './render_list/index.js'
import todoPost from './post_todo/index.js'

const initialState = () => {
    initialStatePresenter()
}

const initialStateModel = () => {
    return {
        "h1_text": "To Do List",
        "btn_text": "Submit",
        "input_text": "Add To Do"
    }
}

const initialStateView = (obj) => {
    const { h1_text, btn_text, input_text } = obj
    const elements = ["h1", "div", "form", "input", "button"]

    // CREATE THE ELEMENTS
    const html = arr => arr.map(element => document.createElement(element))

    // SET THE ATTRIBUTES INTO THE ELEMENTS
    const att = (element, name, value) => element.setAttribute(name, value)

    // APPEND THE ELEMENTS OR TEXTNODE INTO PARENT
    const append = (parent, child) => parent.appendChild(child)

    const [h1, div, form, input, button] = html(elements)

    // CREATE THE TEXT-NODE OF THE ELEMENTS
    const h1Text = document.createTextNode(h1_text)
    const buttonText = document.createTextNode(btn_text)

    append(h1, h1Text)
    append(button, buttonText)

    att(div, "id", "entry-point")
    att(input, "type", "text")
    att(input, "placeholder", input_text)
    att(input, "name", "todo")
    att(input, "id", "input-add-todo")
    att(button, "id", "btn-add-todo")

    append(form, input)
    append(form, button)

    const arrayElements = [h1, form, div]

    return arrayElements
}

const initialStatePresenter = () => {
    const model = initialStateModel()
    const view = initialStateView(model)

    view.forEach(element => {
        document.querySelector("div#root").appendChild(element)
    })

    document.querySelector("form").addEventListener("submit", event => {
        event.preventDefault()

        const inputValue = document.querySelector("input#input-add-todo").value

        if (!inputValue.length) {
            //! TODO: criar snackbar or toast
            alert("Please, digit some To Do")
        } else {
            const answers = {"description": inputValue}
            todoPost(answers)
        }
    })

    renderTodo()
}


// INITIALIZE THE PROGRAM
initialState()