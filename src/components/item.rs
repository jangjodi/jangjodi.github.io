use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub struct Item {
    pub title: String,
    pub additional_info: String,
    pub content: Vec<String>
}

pub fn build_item(item: Vec<&str>, content: Vec<&str>) -> Item {
    let content: Vec<String> = content.into_iter().map(|x| x.to_string() ).collect();
    return Item {
        title: item[0].to_string(),
        additional_info: item[1].to_string(),
        content: content,
    }
}

#[derive(Properties, PartialEq)]
pub struct ItemListProps {
    pub items: Vec<Item>,
}

#[function_component(ItemList)]
pub fn item_list(ItemListProps { items }: &ItemListProps) -> Html {
    items.iter().map(|item| html! {
        <div>
            <div class="item-header">
                <h3>{item.title.clone()}</h3>
                <p>{item.additional_info.clone()}</p>
            </div>
            <ul>
                { for item.content.clone().into_iter().map(|des| html_nested! { <li>{des}</li> }) }
            </ul>
        </div>
    }).collect()
}