import  renderTodo from './render_list/index.js'
import todoPost from './post_todo/index.js'

const initialTemplate = () => {
    console.log("initialTemplate()")
    initialTemplatePresenter()
}

const initialTemplateView = () => {
    return `<h1 class="text-center mt-2">ToDo List</h1>
            <div>
                <div class="input-group">
                    <input type="text" class="form-control">
                    <span class="input-group-btn">
                        <button id="btn-add" class="btn btn-primary" type="button">Adicionar</button>
                    </span>
                </div>
            </div>
            <div class="container-fluid mt-3">
                <ul id="ul-entry-point" class="list-group"></ul>
            </div>
            <div id="entry-point-modal"></div>
            `
}

const initialTemplateModel = () => {

}

const initialTemplatePresenter = () => {
    const model = initialTemplateModel()
    const view = initialTemplateView(model)

    document.querySelector("div#app").innerHTML = view

    document.querySelector("button#btn-add").addEventListener("click", () => {
        const inputValue = document.querySelector("input[type=text]").value 
        const answers = {
            description: inputValue,
            done: true
        }
        todoPost(answers)
        document.querySelector("input[type=text]").value = ""
        
    })

    renderTodo()
}

initialTemplate()