use yew::prelude::*;

#[derive(PartialEq)]
pub struct Experience {
    pub company: String,
    pub role: String,
    pub dates: String,
    pub team: String,
}

pub fn build_experience(experience: [&str; 4]) -> Experience {
    return Experience {
        company: experience[0].to_string(),
        role: experience[1].to_string(),
        dates: experience[2].to_string(),
        team: experience[3].to_string(),
    }
}

#[derive(Properties, PartialEq)]
pub struct ExperienceListProps {
    pub experiences: Vec<Experience>,
}

#[function_component(ExperienceList)]
pub fn experience_list(ExperienceListProps { experiences }: &ExperienceListProps) -> Html {
    experiences.iter().map(|experience| html! {
        <div>
            <h3>{format!("{} at {}", experience.role, experience.company)}</h3>
            <p class="secondary-p-font">{experience.dates.clone()}</p>
            <ul>
                <li>{experience.team.clone()}</li>
            </ul>
        </div>
    }).collect()
}