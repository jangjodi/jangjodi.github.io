use components::experience::build_experience;
use components::experience::ExperienceList;
use components::experience::Experience;
use components::contact::Contact;
use components::contact::ContactList;
use components::skill::build_skill;
use components::skill::SkillList;
use components::skill::Skill;
use components::item::build_item;
use components::item::Item;
use components::item::ItemList;
use constants::BINGO_PROJECT;
use constants::EDUCATION;
use constants::PINTOS_PROJECT;
use constants::THALES_EXPERIENCE;
use yew::prelude::*;
mod components;
pub mod constants;
use constants::SENTRY_EXPERIENCE;
use constants::FORD_EXPERIENCE;
use constants::SUNNYBROOK_EXPERIENCE;
use constants::SKILLS;

#[function_component(App)]
fn app() -> Html {    
    let contacts: Vec<Contact> = vec![
        Contact {
            id: 0,
            name: "LinkedIn".to_string(),
            image: "assets/linkedin_logo.png".to_string(),
            link: "https://www.linkedin.com/in/jodi-jang-0433951a1/".to_string()
        },
        Contact {
            id: 1,
            name: "Github".to_string(),
            image: "assets/github_logo.svg".to_string(),
            link: "https://www.https://github.com/jangjodi".to_string()
        }
    ];
    
    let skills: Vec<Skill> = vec![
        build_skill("Languages", SKILLS[0]),
        build_skill("Frameworks/Libraries", SKILLS[1]),
        build_skill("Databases", SKILLS[2])
    ];

    let experiences: Vec<Experience> = vec![
        build_experience(SENTRY_EXPERIENCE),
        build_experience(FORD_EXPERIENCE),
        build_experience(SUNNYBROOK_EXPERIENCE),
        build_experience(THALES_EXPERIENCE)
    ];

    let education: Vec<Item> = vec![
        build_item(EDUCATION[0..2].to_vec(), EDUCATION[2..3].to_vec())
    ];

    let projects: Vec<Item> = vec![
        build_item(PINTOS_PROJECT[0..2].to_vec(), PINTOS_PROJECT[2..3].to_vec()),
        build_item(BINGO_PROJECT[0..2].to_vec(), BINGO_PROJECT[2..3].to_vec()),
    ];

    html! {
        <div class="main">
            <div class="container"> 
                <meta name="viewport" content="width=device-width, initial-scale=0.60" />
                <div class="intro-container">
                    <img class="self-image" src="assets/self_image.png" alt="image of Jodi" />
                    <div>
                        <h1>{"Hi, I'm Jodi! I'm a software engineer."}</h1>
                        <div class="contact-container">
                            <ContactList contacts={contacts}/>
                        </div>
                    </div>
                </div>
                <div>
                    <h2>{"SKILLS"}</h2>
                    <SkillList skills={skills}/>
                </div>
                <div>
                    <h2>{"EXPERIENCE"}</h2>
                    <ExperienceList experiences={experiences}/>
                </div>
                <div>
                    <h2>{"EDUCATION"}</h2>
                    <ItemList items={education}/>
                </div>
                <div>
                    <h2>{"PROJECTS"}</h2>
                    <ItemList items={projects}/>
                </div>
            </div>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
