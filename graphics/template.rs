use crate::Sprite;

pub enum SpriteList {
{{#sprites}}
    #[allow(non_camel_case_types)]
    {{name}},
{{/sprites}}
}

impl SpriteList {
    pub fn get(self) -> Sprite {
        use SpriteList::*;
        match self {
{{#sprites}}
            {{name}} => Sprite {
                width: {{width}},
                height: {{height}},
                flags: {{flags}},
                data: vec![ {{bytes}} ],
            },
{{/sprites}}
        }
    }
}

