import renderTodo from './render_list/index.js'
import todoPost from './post_todo/index.js'

const initialState = () => {
    initialStatePresenter()
}

const initialStateModel = () => {
    return {}
}

const initialStateView = (obj) => {
    return `
            <h1 class="text-center">ToDo</h1>
            <div class="container col-5 mt-5">
              <ul class="list-group">
                <li class="list-group-item">
                  <input class="form-check-input mr-2" type="checkbox" value="">Make coffee and let's code.
                  <div class="btn-group float-right" role="group" aria-label="Basic example">
                    <button type="button" class="btn btn-warning">Update</button>
                    <button type="button" class="btn btn-danger">Delete</button>
                  </div>
                </li>
              </ul>
            </div>
            `
}

const initialStatePresenter = () => {
    const model = initialStateModel()
    const view = initialStateView(model)

    document.querySelector("div#root").innerHTML = view
   
}


// INITIALIZE THE PROGRAM
initialState()