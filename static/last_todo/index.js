const lastTodo = () => lastTodoPresenter()

const lastTodoView = (obj) => {
    console.log(obj)
    const elementsInSide = ["li", "input", "span", "button"]

    // CREATE THE ELEMENTS
    const html = arr => arr.map(element => document.createElement(element))

    // SETTING THE ATTRIBUTES INTO THE ELEMENTS
    const att = (element, name, value) => element.setAttribute(name, value)

    html(elementsInSide)
    const [li, input, span, button] = html(elementsInSide)

    const spanText = document.createTextNode(obj.description)
    const buttonText = document.createTextNode("Delete")
    att(input, "type", "checkbox")
    att(span, "class", "editable")
    att(button, "class", "delete")
    button.appendChild(buttonText)
    span.appendChild(spanText)
    li.appendChild(input)
    li.appendChild(span)
    li.appendChild(button)

    if(obj.status === 1) {
        input.setAttribute("checked", true)
        input.nextElementSibling.classList.add("sublinado")
    }

    return li
}

const lastTodoModel = () => {
    return fetch("/last")
        .then(response => response.json())
}

const lastTodoPresenter = async () => {
    const model = await lastTodoModel()
    const view = lastTodoView(model)

    document.querySelector("ul").appendChild(view)
}

export default lastTodo