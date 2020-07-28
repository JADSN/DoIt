const modalUpdate = (text) => modalUpdatePresenter(text)

const modalUpdateView = (obj) => {
    const { text } = obj
    const elements = ["div", "div", "span", "input"]

    // CREATE THE ELEMENTS
    const html = arr => arr.map(element => document.createElement(element))

    const [div_modal, div_content, span, input] = html(elements)

    // SET THE ATTRIBUTES INTO THE ELEMENTS
    const att = (element, name, value) => element.setAttribute(name, value)

    const spanText = document.createTextNode("x")

    att(div_modal, "class", "modal")
    att(div_content, "class", "modal-content")
    att(span, "class", "close")
    att(input, "placeholder", text)

    span.appendChild(spanText)
    div_content.appendChild(span)
    div_content.appendChild(input)
    div_modal.appendChild(div_content)

    return div_modal
}

const modalUpdateModel = (text) => {
    return {
        text
    }
}

const modalUpdatePresenter = (text) => {
    const modal = modalUpdateModel(text)
    const view = modalUpdateView(modal)

    document.querySelector("div#entry-point-modal").appendChild(view)

    document.querySelector("span.close").addEventListener("click", () => {
        document.querySelector("#entry-point-modal").removeChild(document.querySelector("#entry-point-modal").firstChild)
    })
}

export default modalUpdate