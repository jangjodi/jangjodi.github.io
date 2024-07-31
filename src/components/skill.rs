use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub struct Skill {
    pub category: String,
    pub items: String,
}

pub fn build_skill(category: &str, items: &str) -> Skill {
    return Skill {
        category: category.to_string(),
        items: items.to_string()
    }
}

#[derive(Properties, PartialEq)]
pub struct SkillListProps {
    pub skills: Vec<Skill>,
}

#[function_component(SkillList)]
pub fn skill_list(SkillListProps { skills }: &SkillListProps) -> Html {
    skills.iter().map(|skill: &Skill| html! {
        <p>
            <span style="font-weight: 500">{format!("{}: ", skill.category)}</span>
            <span style="font-weight: 300">{skill.items.clone()}</span>
        </p>
    }).collect()
}