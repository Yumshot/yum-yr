use yew::prelude::*;


pub fn svg_icon() -> Html {
    html! {
        <svg
  width="24"
  height="24"
  viewBox="0 0 24 24"
  fill="none"
  xmlns="http://www.w3.org/2000/svg"
>
  <path
    fill-rule="evenodd"
    clip-rule="evenodd"
    d="M4 1.5C2.89543 1.5 2 2.39543 2 3.5V4.5C2 4.55666 2.00236 4.61278 2.00698 4.66825C0.838141 5.07811 0 6.19118 0 7.5V19.5C0 21.1569 1.34315 22.5 3 22.5H21C22.6569 22.5 24 21.1569 24 19.5V7.5C24 5.84315 22.6569 4.5 21 4.5H11.874C11.4299 2.77477 9.86384 1.5 8 1.5H4ZM9.73244 4.5C9.38663 3.9022 8.74028 3.5 8 3.5H4V4.5H9.73244ZM3 6.5C2.44772 6.5 2 6.94772 2 7.5V19.5C2 20.0523 2.44772 20.5 3 20.5H21C21.5523 20.5 22 20.0523 22 19.5V7.5C22 6.94772 21.5523 6.5 21 6.5H3Z"
    fill="currentColor"
  />
</svg>
    }
}

struct ProjectStructure {
    name: String,
    description: String,
}



fn generate_project_list() -> Vec<i32> {
let mut my_vec: Vec<i32> = Vec::new();
my_vec.push(1);
my_vec.push(2);
my_vec.push(3);
my_vec.push(4);
my_vec.push(5);
return my_vec
}

#[function_component(Projects)]
pub fn project_view() -> Html {
    let container_one = classes!(
        "flex",
        "flex-col",
        "items-center",
        "justify-center",
        "min-h-screen"
    );
    html!(<div class={container_one}>
    <div class="file-explorer">
    <a href="#">
      <div> {svg_icon()} {"Project 1"}</div>
      <div>{"Description of Project 1"}</div>
    </a>
    <a href="#">
      <div>{svg_icon()}{"Project 2"}</div>
      <div>{"Description of Project 2"}</div>
    </a>
    <a href="#">
      <div>{svg_icon()}{"Project 3"}</div>
      <div>{"Description of Project 3"}</div>
    </a>
  </div>
  </div>)
}
