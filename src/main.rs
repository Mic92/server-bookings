use js_sys::{Date, Object};
use yew::html::Scope;
use yew::{html, Classes, Component, ComponentLink, Html, ShouldRender};

enum Msg {
    SaveOrEdit(usize),
    Delete(usize),
    Book,
}

#[derive(Debug)]
pub struct Booking {
    pub begin: Date,
    pub end: Date,
    pub who: String,
    pub server: String,
    pub editing: bool,
}

struct Model {
    // `ComponentLink` is like a reference to a component.
    // It can be used to send messages to the component
    link: ComponentLink<Self>,
    bookings: Vec<Booking>,
}

fn view_input_col(entry: &Booking, value: String) -> Html {
    let editclass = if entry.editing {
        "form-control"
    } else {
        "form-control-plaintext"
    };
    html!{
      <th scope="col">
        <input type="text"
         readonly=!entry.editing
         class=editclass
         value={value}/>
      </th>
    }

}

fn view_booking((idx, entry): (usize, &Booking), link: &Scope<Model>) -> Html {
    let mut class = Classes::from("booking");
    if entry.editing {
        class.push(" editing");
    }
    let opts = Object::new();
    let begin = String::from(entry.begin.to_locale_string("de-DE", &opts));
    let end = String::from(entry.end.to_locale_string("de-DE", &opts));
    html! {
      <tr class={class}>
        <th scope="row">{idx}</th>
        {view_input_col(entry, begin)}
        {view_input_col(entry, end)}
        {view_input_col(entry, entry.server.to_string())}
        {view_input_col(entry, entry.who.to_string())}
        <th scope="col">
          <div class="btn-group" role="group" aria-label="Actions">
            <button
             type="button"
             class="btn btn-primary"
             onclick=link.callback(move |_| Msg::SaveOrEdit(idx))>
             {if entry.editing { "Save" } else { "Edit" }}
            </button>
            <button
              type="button"
              class="btn btn-danger"
              onclick=link.callback(move |_| Msg::Delete(idx))>
              {"Delete"}
            </button>
          </div>
        </th>
      </tr>
    }
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();
    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let now = Date::new_0();
        let later = Date::new_0();
        later.set_hours(later.get_hours() + 1);

        Self {
            link,
            bookings: vec![
                Booking {
                    begin: now.clone(),
                    end: later.clone(),
                    who: String::from("Dimitria"),
                    server: String::from("Martha"),
                    editing: false,
                },
                Booking {
                    begin: now,
                    end: later,
                    who: String::from("Dimitrios"),
                    server: String::from("Rose"),
                    editing: false,
                },
            ],
        }
    }
    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::SaveOrEdit(idx) => match self.bookings.get_mut(idx) {
                Some(e) => {
                    e.editing = !e.editing;
                    true
                }
                None => false,
            },
            Msg::Delete(idx) => {
                if self.bookings.get(idx).is_some() {
                    self.bookings.remove(idx);
                    true
                } else {
                    false
                }
            }
            Msg::Book => {
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        false
    }
    fn view(&self) -> Html {
        html! {
            <div>
               <table class="table">
                <thead>
                  <tr>
                    <th scope="col">{"#"}</th>
                    <th scope="col">{"Begin"}</th>
                    <th scope="col">{"End"}</th>
                    <th scope="col">{"Server"}</th>
                    <th scope="col">{"Who"}</th>
                    <th scope="col">{"Action"}</th>
                  </tr>
                </thead>
                <tbody>
                  { for self.bookings.iter().enumerate().map(|e| view_booking(e, &self.link)) }
                   <tr>
                     <th scope="row">{self.bookings.len()}</th>
                     <th scope="col">
                       <input type="text"
                       class="form-control"
                       value=""/> // begin
                     </th>
                     <th scope="col">
                       <input type="text"
                       class="form-control"
                       value=""/> // end
                     </th>
                     <th scope="col">
                       <input type="text"
                       class="form-control"
                       value=""/> // server
                     </th>
                     <th scope="col">
                       <input type="text"
                       class="form-control"
                       value=""/> // who
                     </th>
                     <th scope="col">
                       <div class="btn-group" role="group" aria-label="Actions">
                         <button
                          type="button"
                          class="btn btn-primary"
                          onclick=self.link.callback(move |_| Msg::Book)>
                          { "Book" }
                         </button>
                       </div>
                     </th>
                   </tr>
                  </tbody>
               </table>
            </div>
        }
    }
}
fn main() {
    yew::start_app::<Model>();
}
