import renderTodo from "../render_list/index.js"

const updateStatus = (answers) => updateStatusPresenter(answers)

const updateStatusView = (obj) => {
    return JSON.stringify(obj)
}

const updateStatusModel = (answers) => {
    const { id, status } = answers
    
    return fetch(`/todos/status/${id}`, {
        method: 'PATCH',
        headers: {
            'Accept': 'application/json',
            'Content-Type': 'application/json'
        },
        body: JSON.stringify({"status": status})
    }).then(respone => respone.status)
}

const updateStatusPresenter = async (answers) => {
    const model = await updateStatusModel(answers)
    const view = updateStatusView(model)

    if(view === "200") {
        renderTodo()
    }
}

export default updateStatus