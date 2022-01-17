use crate::Sprite;

#[derive(Clone, Copy, Debug)]
pub enum SpriteName {
{{#sprites}}
    #[allow(non_camel_case_types)]
    {{name}},
{{/sprites}}
}

impl SpriteName {
    pub fn get(self) -> Sprite {
        use SpriteName::*;
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

