import updateStatus from '../update_status_list/index.js'
import deleteTodo from '../delete_todo/index.js'

const modal = (obj) => modalPresenter(obj)

const modalView = (obj) => {
  return `
  <div id="modal-window" class="modal hidden">
    <div class="container">
        <div class="body-modal">
            <div id="" class="modal-header">
                <h5 class="modal-title">${obj.title}</h5>
            </div>
            <div class="modal-body">
              ${obj.title === "Editar" ? `<input type="text" id="input-modal" class="form-control" value="${obj.placeholder}"/>` :
      `<label><strong>Deseja realmente deletar?</strong></label>
              <span>${obj.placeholder}</span>`}
            </div>
            <div class="modal-footer">
                <button id="btn-close-modal" type="button" class="btn btn-secondary">Fechar</button>
                <button id="btn-save-modal" type="button" class="btn btn-primary">Salvar</button>
            </div>
        </div>
    </div>
  </div>
  `
}

const modalModel = (obj) => {
  const { id, title, placeholder } = obj

  return {
    id,
    title,
    placeholder
  }
}

const modalPresenter = (obj) => {
  const model = modalModel(obj)
  const view = modalView(model)
  console.log(model)
  document.querySelector("div#entry-point-modal").innerHTML = view
  document.querySelector('div#modal-window').classList.remove('hidden')

  document.querySelector("button#btn-close-modal").addEventListener("click", () => {
    document.querySelector('div#modal-window').classList.add('hidden')
  })

  document.querySelector("button#btn-save-modal").addEventListener("click", () => {
    if (model.title === "Editar") {
      const inputValue = document.querySelector("input#input-modal").value
      const answers = {
        "id": model.id,
        "description": inputValue,
        "done": false
      }
      updateStatus(answers)

    } else if (model.title === "Deletar") {
      const { id } = model
      deleteTodo(id)
    }

    document.querySelector('div#modal-window').classList.add('hidden')
  })
}

export default modal