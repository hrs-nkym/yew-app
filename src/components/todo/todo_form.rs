use yew::{function_component, html, Html};

#[function_component(TodoForm)]
pub fn todo_item() -> Html {
	html! {
		<form class="mb-5">
		<div class="mb-3">
		<label for="title" class="form-label">{"タイトル"}</label>
		<input type="text" class="form-control" id="title" />
		</div>
		<button type="submit" class="btn btn-primary">{"追加"}</button>
		</form>
	}
}