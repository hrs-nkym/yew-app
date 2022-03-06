use yew::{function_component, html, Html, Properties};
use crate::components::todo::todo_item::TodoItem;
use crate::components::todo::types::Todo;

#[derive(Properties, PartialEq)]
pub struct TodoItemProps {
	pub todo_items: Vec<Todo>,
}

#[function_component(TodoList)]
pub fn todo_list() -> Html {
	let todo_item = vec![
		Todo {
			id:1,
			title: "お茶".to_string(),
			completed: false,
		},
		Todo {
			id:2,
			title: "ねこ元気".to_string(),
			completed: false,
		},
		Todo {
			id:3,
			title: "オレンジジュース".to_string(),
			completed: true,
		},
	];
	html! {
		<ul class="list-group">
		  {todo_item.iter().map(|todo| html! {
			<TodoItem title={todo.title.clone()} completed={todo.completed} />
		  }).collect::<Html>()}
		</ul>
	}
}