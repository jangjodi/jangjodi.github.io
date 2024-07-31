use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub struct Contact {
    pub id: usize,
    pub name: String,
    pub image: String,
    pub link: String,
}

#[derive(Properties, PartialEq)]
pub struct ContactListProps {
    pub contacts: Vec<Contact>,
}

#[function_component(ContactList)]
pub fn experience_list(ContactListProps { contacts }: &ContactListProps) -> Html {
    contacts.iter().map(|contact| html! {
        <div class="contact-item">
            <img src={contact.image.clone()} class="icon"/>
            <a href={contact.link.clone()}>{contact.name.clone()}</a>
        </div>
    }).collect()
}